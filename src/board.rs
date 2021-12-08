// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::action::Action;
use crate::board_util::{self, Coord, HeapElement, Tiles};
use crate::fixed::{Arrow, COLORS, TINY_DELAY};
use fltk::enums::Color;
use fltk::prelude::*;
use rand::seq::SliceRandom;
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

type CoordSet = HashSet<Coord>;
type CoordForCoord = HashMap<Coord, Coord>;

pub struct Board {
    widget: fltk::widget::Widget,
    drawing: Rc<RefCell<bool>>,
    game_over: Rc<RefCell<bool>>,
    selected: Rc<RefCell<Option<Coord>>>,
    tiles: Rc<RefCell<Tiles>>,
    columns: Rc<RefCell<u8>>,
    rows: Rc<RefCell<u8>>,
    maxcolors: Rc<RefCell<u8>>,
    delay_ms: Rc<RefCell<u16>>,
    score: Rc<RefCell<u16>>,
    adjoining: Rc<RefCell<CoordSet>>,
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
            columns: Rc::default(),
            rows: Rc::default(),
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
        *self.columns.borrow_mut() = config.board_columns;
        *self.rows.borrow_mut() = config.board_rows;
        *self.maxcolors.borrow_mut() = config.board_maxcolors;
        *self.delay_ms.borrow_mut() = config.board_delay_ms;
        *self.tiles.borrow_mut() = self.get_tiles();
        *self.game_over.borrow_mut() = false;
        *self.drawing.borrow_mut() = false;
        self.sender.send(Action::UpdatedScore(*self.score.borrow()));
        self.widget.redraw();
    }

    fn get_tiles(&self) -> Tiles {
        let columns = *self.columns.borrow() as usize;
        let rows = *self.rows.borrow() as usize;
        let colors = self.get_colors();
        let mut rng = rand::thread_rng();
        let mut tiles = Vec::with_capacity(columns);
        for column in 0..columns {
            tiles.push(Vec::with_capacity(rows));
            for _ in 0..rows {
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
        if self.selected.borrow().is_none() {
            *self.selected.borrow_mut() = Some(Coord::new(
                *self.columns.borrow() / 2,
                *self.rows.borrow() / 2,
            ));
        } else {
            let mut coord = self.selected.borrow().unwrap().clone();
            match arrow {
                Arrow::Left => coord.x -= 1,
                Arrow::Right => coord.x += 1,
                Arrow::Up => coord.y -= 1,
                Arrow::Down => coord.y += 1,
            }
            let tiles = &*self.tiles.borrow();
            // Coord.{x,y} are u8 so guaranteed >= 0
            if coord.x < *self.columns.borrow()
                && coord.y < *self.rows.borrow()
                && tiles[coord.x as usize][coord.y as usize].is_some()
            {
                *self.selected.borrow_mut() = Some(coord);
            }
        }
        self.widget.redraw();
    }

    pub fn on_press_tile(&mut self) {
        if *self.drawing.borrow() || *self.game_over.borrow() {
            return;
        }
        let coord = *self.selected.borrow();
        if let Some(coord) = coord {
            self.delete_tile(coord);
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
        self.delete_tile(Coord::new(x as u8, y as u8));
    }

    fn get_tile_size(&self) -> (i32, i32) {
        let columns = *self.columns.borrow() as i32;
        let rows = *self.rows.borrow() as i32;
        board_util::get_tile_size(
            columns,
            rows,
            self.widget.width(),
            self.widget.height(),
        )
    }

    fn delete_tile(&mut self, coord: Coord) {
        let color =
            self.tiles.borrow()[coord.x as usize][coord.y as usize];
        if color.is_none() {
            return;
        }
        let color = color.unwrap();
        if !self.is_legal(&coord, color.clone()) {
            return;
        }
        self.dim_adjoining(&coord, &color);
    }

    fn is_legal(&self, coord: &Coord, color: Color) -> bool {
        // A legal click is on a colored tile that is adjacent to another
        // tile of the same color.
        let tiles = &*self.tiles.borrow();
        let x = coord.x as usize;
        let y = coord.y as usize;
        let color = Some(color);
        let columns = *self.columns.borrow() as usize;
        let rows = *self.rows.borrow() as usize;
        if x > 0 && tiles[x - 1][y] == color {
            return true;
        }
        if x + 1 < columns && tiles[x + 1][y] == color {
            return true;
        }
        if y > 0 && tiles[x][y - 1] == color {
            return true;
        }
        if y + 1 < rows && tiles[x][y + 1] == color {
            return true;
        }
        false
    }

    fn dim_adjoining(&mut self, coord: &Coord, color: &Color) {
        self.adjoining.borrow_mut().clear();
        self.populate_adjoining(*coord, *color);
        *self.score.borrow_mut() += (self.adjoining.borrow().len()
            as u16)
            .pow(*self.maxcolors.borrow() as u32 - 2);
        self.sender.send(Action::UpdatedScore(*self.score.borrow()));
        let tiles = &mut *self.tiles.borrow_mut();
        for &coord in self.adjoining.borrow().iter() {
            let x = coord.x as usize;
            let y = coord.y as usize;
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

    fn populate_adjoining(&self, coord: Coord, color: Color) {
        let columns = *self.columns.borrow() as usize;
        let rows = *self.rows.borrow() as usize;
        let x = coord.x as usize;
        let y = coord.y as usize;
        if x >= columns || y >= rows {
            return; // Falled off an edge; Coord.{x,y} cannot be < 0
        }
        let tiles = &*self.tiles.borrow();
        if self.adjoining.borrow().contains(&coord)
            || tiles[x][y] != Some(color)
        {
            return; // Color doesn't match or already done
        }
        self.adjoining.borrow_mut().insert(coord);
        if coord.x > 0 {
            self.populate_adjoining(
                Coord::new(coord.x - 1, coord.y),
                color,
            );
        }
        self.populate_adjoining(Coord::new(coord.x + 1, coord.y), color);
        if coord.y > 0 {
            self.populate_adjoining(
                Coord::new(coord.x, coord.y - 1),
                color,
            );
        }
        self.populate_adjoining(Coord::new(coord.x, coord.y + 1), color);
    }

    pub fn delete_adjoining(&mut self) {
        let tiles = &mut *self.tiles.borrow_mut();
        for &coord in self.adjoining.borrow().iter() {
            tiles[coord.x as usize][coord.y as usize] = None
        }
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
        dbg!("close_up"); // Use board_util::ripple()
    }

    fn move_tiles(&mut self) {
        let tiles = self.tiles.clone();
        let columns = *self.columns.borrow() as usize;
        let rows = *self.rows.borrow() as usize;
        let mut moved = true;
        let mut moves = CoordForCoord::new();
        while moved {
            moved = false;
            for x in board_util::ripple(columns) {
                for y in board_util::ripple(rows) {
                    if let Some(color) = tiles.borrow()[x][y] {
                        if self.move_if_possible(
                            color,
                            Coord::new(x as u8, y as u8),
                            &mut moves,
                        ) {
                            moved = true;
                            break;
                        }
                    }
                }
            }
        }
    }

    fn move_if_possible(
        &mut self,
        color: Color,
        coord: Coord,
        moves: &mut CoordForCoord,
    ) -> bool {
        let mut empties = self.get_empty_neighbours(coord);
        if !empties.is_empty() {
            let (do_move, new_coord) =
                self.nearest_to_middle(color, coord, &empties);
            if let Some(key_coord) = moves.get(&new_coord) {
                if key_coord == &coord {
                    return false; // avoid endless loop back and forth
                }
            }
            if do_move {
                let tiles = self.tiles.clone();
                let new_color =
                    tiles.borrow()[coord.x as usize][coord.y as usize];
                if let Some(new_color) = new_color {
                    self.sender.send(Action::MoveTile {
                        new_coord,
                        new_color,
                        coord,
                    });
                }
                moves[&coord] = new_coord; // ###################
                return true;
            }
        }
        false
    }

    pub fn move_tile(
        &mut self,
        new_coord: Coord,
        new_color: Color,
        coord: Coord,
    ) {
        let tiles = &mut *self.tiles.borrow_mut();
        tiles[new_coord.x as usize][new_coord.y as usize] =
            Some(new_color);
        tiles[coord.x as usize][coord.y as usize] = None;
        let delay = 1.0_f64.max((*self.delay_ms.borrow() / 7000) as f64);
        let sender = self.sender.clone();
        fltk::app::add_timeout(delay, move || {
            sender.send(Action::Redraw);
        });
    }

    fn get_empty_neighbours(&mut self, coord: Coord) -> CoordSet {
        let columns = *self.columns.borrow() as usize;
        let rows = *self.rows.borrow() as usize;
        let mut coords = Vec::with_capacity(4);
        if coord.x > 0 {
            coords.push(Coord::new(coord.x - 1, coord.y));
        }
        coords.push(Coord::new(coord.x + 1, coord.y));
        if coord.y > 0 {
            coords.push(Coord::new(coord.x, coord.y - 1));
        }
        coords.push(Coord::new(coord.x, coord.y + 1));
        let mut neighbours = CoordSet::new();
        let tiles = self.tiles.clone();
        for new_coord in coords.iter() {
            let x = new_coord.x as usize;
            let y = new_coord.y as usize;
            if x < columns && y < rows {
                if tiles.borrow()[x][y].is_none() {
                    neighbours.insert(new_coord.clone());
                }
            }
        }
        neighbours
    }

    fn nearest_to_middle(
        &self,
        color: Color,
        coord: Coord,
        empties: &CoordSet,
    ) -> (bool, Coord) {
        let x_mid = (*self.columns.borrow() as u8) / 2;
        let y_mid = (*self.rows.borrow() as u8) / 2;
        let d_old =
            ((x_mid - coord.x) as f64).hypot((y_mid - coord.y) as f64);
        let mut heap = BinaryHeap::new();
        for new_coord in empties.iter() {
            if self.is_square(&new_coord) {
                let mut d_new = ((x_mid - new_coord.x) as f64)
                    .hypot((y_mid - new_coord.y) as f64);
                if self.is_legal(&new_coord, color) {
                    d_new -= 0.1; // Make same colors slightly attractive
                }
                heap.push(Reverse(HeapElement::new(
                    d_new,
                    new_coord.clone(),
                )));
            }
        }
        if let Some(Reverse(element)) = heap.pop() {
            if d_old > element.d {
                return (true, element.coord);
            }
        }
        (false, coord)
    }

    fn is_square(&self, coord: &Coord) -> bool {
        let columns = *self.columns.borrow() as usize;
        let rows = *self.rows.borrow() as usize;
        let x = coord.x as usize;
        let y = coord.y as usize;
        let tiles = &*self.tiles.borrow();
        if x > 0 && tiles[x - 1][y].is_some() {
            true
        } else if x + 1 < columns && tiles[x + 1][y].is_some() {
            true
        } else if y > 0 && tiles[x][y - 1].is_some() {
            true
        } else if y + 1 < rows && tiles[x][y + 1].is_some() {
            true
        } else {
            false
        }
    }

    pub fn redraw(&mut self) {
        self.widget.redraw();
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
    let columns = board.columns.clone();
    let rows = board.rows.clone();
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
            *columns.borrow(),
            *rows.borrow(),
            &*tiles.borrow(),
            *selected.borrow(),
        );
        // *MUST* restore the line style after custom drawing
        fltk::draw::set_line_style(fltk::draw::LineStyle::Solid, 0);
        *drawing.borrow_mut() = false;
    });
}
