// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::action::Action;
use crate::board_util::{self, Pos, Size, Tiles};
use crate::fixed::{Arrow, COLORS, TINY_DELAY};
use fltk::enums::Color;
use fltk::prelude::*;
use rand::seq::SliceRandom;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

type PosSet = HashSet<Pos>;

pub struct Board {
    widget: fltk::widget::Widget,
    drawing: Rc<RefCell<bool>>,
    game_over: Rc<RefCell<bool>>,
    selected: Rc<RefCell<Option<Pos>>>,
    tiles: Rc<RefCell<Tiles>>,
    size: Rc<RefCell<Size>>,
    maxcolors: Rc<RefCell<u8>>,
    delay_ms: Rc<RefCell<u16>>,
    score: Rc<RefCell<u16>>,
    adjoining: Rc<RefCell<PosSet>>,
    sender: fltk::app::Sender<Action>,
}

impl Board {
    pub fn new(sender: fltk::app::Sender<Action>) -> Self {
        let mut board = Board {
            widget: fltk::widget::Widget::default(),
            drawing: Rc::default(),
            game_over: Rc::default(),
            selected: Rc::default(),
            tiles: Rc::default(),
            size: Rc::default(),
            maxcolors: Rc::default(),
            delay_ms: Rc::default(),
            score: Rc::default(),
            adjoining: Rc::default(),
            sender,
        };
        add_event_handler(&mut board, sender);
        add_draw_handler(&mut board);
        board
    }

    pub fn new_game(&mut self) {
        *self.drawing.borrow_mut() = true;
        *self.game_over.borrow_mut() = true;
        *self.selected.borrow_mut() = None;
        *self.score.borrow_mut() = 0;
        let config = CONFIG.get().read().unwrap();
        *self.size.borrow_mut() = Size::new(
            config.board_columns as usize,
            config.board_rows as usize,
        );
        *self.maxcolors.borrow_mut() = config.board_maxcolors;
        *self.delay_ms.borrow_mut() = config.board_delay_ms;
        *self.tiles.borrow_mut() = self.get_tiles();
        *self.game_over.borrow_mut() = false;
        *self.drawing.borrow_mut() = false;
        self.sender.send(Action::UpdatedScore(*self.score.borrow()));
        self.widget.redraw();
    }

    fn get_tiles(&self) -> Tiles {
        let size = *self.size.borrow();
        let colors = self.get_colors();
        let mut rng = rand::thread_rng();
        let mut tiles = Vec::with_capacity(size.columns);
        for column in 0..size.columns {
            tiles.push(Vec::with_capacity(size.rows));
            for _ in 0..size.rows {
                let color = colors.choose(&mut rng);
                let color = if color.is_some() {
                    Some(color.unwrap().clone()) // Want Color not &Color
                } else {
                    None
                };
                tiles[column].push(color);
            }
        }
        tiles
    }

    fn get_colors(&self) -> Vec<Color> {
        let mut rng = rand::thread_rng();
        let colors = COLORS.get().read().unwrap();
        colors
            .choose_multiple(&mut rng, (*self.maxcolors.borrow()).into())
            .cloned()
            .collect()
    }

    pub fn on_arrow(&mut self, arrow: Arrow) {
        if *self.drawing.borrow() || *self.game_over.borrow() {
            return;
        }
        let size = *self.size.borrow();
        if self.selected.borrow().is_none() {
            *self.selected.borrow_mut() =
                Some(Pos::new(size.columns / 2, size.rows / 2));
        } else {
            let mut pos = self.selected.borrow().unwrap().clone();
            match arrow {
                Arrow::Left => pos.x -= 1,
                Arrow::Right => pos.x += 1,
                Arrow::Up => pos.y -= 1,
                Arrow::Down => pos.y += 1,
            }
            let tiles = &*self.tiles.borrow();
            // Pos.{x,y} are usize so guaranteed >= 0
            if pos.x < size.columns
                && pos.y < size.rows
                && tiles[pos.x][pos.y].is_some()
            {
                *self.selected.borrow_mut() = Some(pos);
            }
        }
        self.widget.redraw();
    }

    pub fn on_press_tile(&mut self) {
        if *self.drawing.borrow() || *self.game_over.borrow() {
            return;
        }
        let pos = *self.selected.borrow();
        if let Some(pos) = pos {
            self.delete_tile(pos);
        }
    }

    pub fn on_click_tile(&mut self) {
        if *self.drawing.borrow() || *self.game_over.borrow() {
            return;
        }
        let (tile_width, tile_height) = self.get_tile_size();
        let x = (fltk::app::event_x() - self.widget.x()) / tile_width;
        let y = (fltk::app::event_y() - self.widget.y()) / tile_height;
        *self.selected.borrow_mut() = None;
        self.delete_tile(Pos::new(x as usize, y as usize));
    }

    fn get_tile_size(&self) -> (i32, i32) {
        let size = *self.size.borrow();
        board_util::get_tile_size(
            size.columns as i32,
            size.rows as i32,
            self.widget.width(),
            self.widget.height(),
        )
    }

    fn delete_tile(&mut self, pos: Pos) {
        let color = self.tiles.borrow()[pos.x][pos.y];
        if color.is_none() {
            return;
        }
        let color = color.unwrap();
        if !self.is_legal(&pos, color.clone()) {
            return;
        }
        self.dim_adjoining(&pos, &color);
    }

    fn is_legal(&self, pos: &Pos, color: Color) -> bool {
        // A legal click is on a colored tile that is adjacent to another
        // tile of the same color.
        let tiles = &*self.tiles.borrow();
        let size = *self.size.borrow();
        let x = pos.x;
        let y = pos.y;
        let color = Some(color);
        if x > 0 && tiles[x - 1][y] == color {
            true
        } else if x + 1 < size.columns && tiles[x + 1][y] == color {
            true
        } else if y > 0 && tiles[x][y - 1] == color {
            true
        } else if y + 1 < size.rows && tiles[x][y + 1] == color {
            true
        } else {
            false
        }
    }

    fn dim_adjoining(&mut self, pos: &Pos, color: &Color) {
        self.adjoining.borrow_mut().clear();
        self.populate_adjoining(*pos, *color);
        *self.score.borrow_mut() += (self.adjoining.borrow().len()
            as u16)
            .pow(*self.maxcolors.borrow() as u32 - 2);
        self.sender.send(Action::UpdatedScore(*self.score.borrow()));
        let tiles = &mut *self.tiles.borrow_mut();
        for &adjoining_pos in self.adjoining.borrow().iter() {
            let x = adjoining_pos.x;
            let y = adjoining_pos.y;
            tiles[x][y] = Some(tiles[x][y].unwrap().darker());
        }
        fltk::app::sleep(TINY_DELAY);
        self.widget.redraw();
        let sender = self.sender.clone();
        fltk::app::add_timeout(
            *self.delay_ms.borrow() as f64 / 1000.0,
            move || {
                sender.send(Action::DeleteAdjoining);
            },
        );
    }

    fn populate_adjoining(&self, pos: Pos, color: Color) {
        let size = *self.size.borrow();
        let x = pos.x;
        let y = pos.y;
        let tiles = &*self.tiles.borrow();
        if self.adjoining.borrow().contains(&pos)
            || tiles[x][y] != Some(color)
        {
            return; // Color doesn't match or already done
        }
        self.adjoining.borrow_mut().insert(pos);
        if x > 0 {
            self.populate_adjoining(Pos::new(x - 1, y), color);
        }
        if x + 1 < size.columns {
            self.populate_adjoining(Pos::new(x + 1, y), color);
        }
        if y > 0 {
            self.populate_adjoining(Pos::new(x, y - 1), color);
        }
        if y + 1 < size.rows {
            self.populate_adjoining(Pos::new(x, y + 1), color);
        }
    }

    pub fn delete_adjoining(&mut self) {
        let tiles = &mut *self.tiles.borrow_mut();
        for &pos in self.adjoining.borrow().iter() {
            tiles[pos.x][pos.y] = None
        }
        self.adjoining.borrow_mut().clear();
        fltk::app::sleep(TINY_DELAY);
        self.widget.redraw();
        let sender = self.sender.clone();
        fltk::app::add_timeout(
            *self.delay_ms.borrow() as f64 / 1000.0,
            move || {
                sender.send(Action::CloseUp);
            },
        );
    }

    pub fn close_up(&mut self) {
        self.move_tiles();
        if let Some(mut selected) = *self.selected.borrow_mut() {
            let x = selected.x;
            let y = selected.y;
            let tiles = &*self.tiles.borrow();
            if tiles[x][y].is_none() {
                let size = *self.size.borrow();
                selected = Pos::new(size.rows / 2, size.columns / 2);
            }
        }
        self.widget.redraw();
        // TODO check game over
    }

    fn move_tiles(&mut self) {
        let tiles = self.tiles.clone();
        let size = *self.size.borrow();
        let mut moved = true;
        while moved {
            moved = false;
            for x in board_util::ripple(size.columns) {
                for y in board_util::ripple(size.rows) {
                    if tiles.borrow()[x][y].is_some() {
                        if self.move_if_possible(Pos::new(x, y)) {
                            moved = true;
                            break;
                        }
                    }
                }
            }
        }
    }

    fn move_if_possible(&mut self, pos: Pos) -> bool {
        let empties = self.get_empty_neighbours(pos);
        if !empties.is_empty() {
            let (do_move, new_pos) =
                self.nearest_to_middle(pos, &empties);
            if do_move {
                let tiles = &mut *self.tiles.borrow_mut();
                tiles[new_pos.x][new_pos.y] = tiles[pos.x][pos.y];
                tiles[pos.x][pos.y] = None;
                fltk::app::sleep(TINY_DELAY);
                self.widget.redraw();
                return true;
            }
        }
        false
    }

    fn get_empty_neighbours(&mut self, pos: Pos) -> PosSet {
        let size = *self.size.borrow();
        let x = pos.x;
        let y = pos.y;
        let mut positions = Vec::with_capacity(4);
        if x > 0 {
            positions.push(Pos::new(x - 1, y));
        }
        if x + 1 < size.columns {
            positions.push(Pos::new(x + 1, y));
        }
        if y > 0 {
            positions.push(Pos::new(x, y - 1));
        }
        if y + 1 < size.rows {
            positions.push(Pos::new(x, y + 1));
        }
        let mut neighbours = PosSet::new();
        let tiles = self.tiles.clone();
        for new_pos in positions.iter() {
            if tiles.borrow()[new_pos.x][new_pos.y].is_none() {
                neighbours.insert(new_pos.clone());
            }
        }
        neighbours
    }

    fn nearest_to_middle(
        &self,
        pos: Pos,
        empties: &PosSet,
    ) -> (bool, Pos) {
        let x = pos.x;
        let y = pos.y;
        let tiles = self.tiles.clone();
        let color = tiles.borrow()[x][y].unwrap();
        let size = *self.size.borrow();
        let mid_x = size.columns / 2;
        let mid_y = size.rows / 2;
        let old_radius = ((mid_x - x) as f64).hypot((mid_y - y) as f64);
        let mut shortest_radius = f64::NAN;
        let mut radius_pos = Pos::default(); // invalid
        for new_pos in empties.iter() {
            let nx = new_pos.x;
            let ny = new_pos.y;
            if self.is_square(&new_pos) {
                let mut new_radius =
                    ((mid_x - nx) as f64).hypot((mid_y - ny) as f64);
                if self.is_legal(&new_pos, color) {
                    // Make same colors slightly attractive
                    new_radius -= 0.1;
                }
                if !radius_pos.is_valid() || shortest_radius > new_radius
                {
                    shortest_radius = new_radius;
                    radius_pos = new_pos.clone();
                }
            }
        }
        if !shortest_radius.is_nan() && old_radius > shortest_radius {
            (true, radius_pos)
        } else {
            (false, pos)
        }
    }

    fn is_square(&self, pos: &Pos) -> bool {
        let x = pos.x;
        let y = pos.y;
        let size = *self.size.borrow();
        let tiles = &*self.tiles.borrow();
        if x > 0 && tiles[x - 1][y].is_some() {
            true
        } else if x + 1 < size.columns && tiles[x + 1][y].is_some() {
            true
        } else if y > 0 && tiles[x][y - 1].is_some() {
            true
        } else if y + 1 < size.rows && tiles[x][y + 1].is_some() {
            true
        } else {
            false
        }
    }
}

impl Deref for Board {
    type Target = fltk::widget::Widget;

    fn deref(&self) -> &Self::Target {
        &self.widget
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.widget
    }
}

fn add_event_handler(
    board: &mut Board,
    sender: fltk::app::Sender<Action>,
) {
    let drawing = board.drawing.clone();
    let game_over = board.game_over.clone();
    board.widget.handle(move |_, event| {
        if *drawing.borrow() || *game_over.borrow() {
            return false;
        }
        match event {
            fltk::enums::Event::Push => {
                sender.send(Action::ClickTile);
                true
            }
            _ => false,
        }
    });
}

fn add_draw_handler(board: &mut Board) {
    let drawing = board.drawing.clone();
    let game_over = board.game_over.clone();
    let selected = board.selected.clone();
    let tiles = board.tiles.clone();
    let size = board.size.clone();
    board.widget.draw(move |widget| {
        if *drawing.borrow() || *game_over.borrow() {
            return;
        }
        *drawing.borrow_mut() = true;
        let width = widget.width();
        let height = widget.height();
        let x1 = widget.x();
        let y1 = widget.y();
        fltk::draw::set_line_style(fltk::draw::LineStyle::Solid, 0);
        board_util::draw_background(x1, y1, width, height);
        board_util::draw_tiles(
            x1,
            y1,
            width,
            height,
            *size.borrow(),
            &*tiles.borrow(),
            *selected.borrow(),
        );
        // *MUST* restore the line style after custom drawing
        fltk::draw::set_line_style(fltk::draw::LineStyle::Solid, 0);
        *drawing.borrow_mut() = false;
    });
}
