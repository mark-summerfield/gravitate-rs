// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::action::Action;
use crate::board_util::{self, Mode, Pos, Size, Tiles};
use crate::fixed::{Arrow, COLORS, TINY_DELAY};
use fltk::enums::Color;
use fltk::prelude::*;
use rand::seq::SliceRandom;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

type PosSet = HashSet<Pos>;
type PosForPos = HashMap<Pos, Pos>;

pub struct Board {
    widget: fltk::widget::Widget,
    drawing: Rc<RefCell<bool>>,
    mode: Rc<RefCell<Mode>>,
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
            mode: Rc::new(RefCell::new(Mode::GameOver)),
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
        *self.mode.borrow_mut() = Mode::Playing;
        *self.selected.borrow_mut() = None;
        *self.score.borrow_mut() = 0;
        let config = CONFIG.get().read().unwrap();
        *self.size.borrow_mut() = Size::new(
            config.board_columns as i32,
            config.board_rows as i32,
        );
        *self.maxcolors.borrow_mut() = config.board_maxcolors;
        *self.delay_ms.borrow_mut() = config.board_delay_ms;
        *self.tiles.borrow_mut() = self.get_tiles();
        *self.drawing.borrow_mut() = false;
        self.sender.send(Action::UpdatedScore(*self.score.borrow()));
        self.widget.redraw();
    }

    fn get_tiles(&self) -> Tiles {
        let size = *self.size.borrow();
        let colors = self.get_colors();
        let mut rng = rand::thread_rng();
        let mut tiles = Vec::with_capacity(size.columns as usize);
        for column in 0..size.columns {
            tiles.push(Vec::with_capacity(size.rows as usize));
            for _ in 0..size.rows {
                let color = colors.choose(&mut rng).copied();
                tiles[column as usize].push(color);
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
        if *self.drawing.borrow() || *self.mode.borrow() != Mode::Playing
        {
            return;
        }
        let size = *self.size.borrow();
        if self.selected.borrow().is_none() {
            *self.selected.borrow_mut() =
                Some(Pos::new(size.columns / 2, size.rows / 2));
        } else {
            let mut pos = self.selected.borrow().unwrap();
            match arrow {
                Arrow::Left => pos.x -= 1,
                Arrow::Right => pos.x += 1,
                Arrow::Up => pos.y -= 1,
                Arrow::Down => pos.y += 1,
            }
            let tiles = &*self.tiles.borrow();
            let x = pos.x;
            let y = pos.y;
            if 0 <= x
                && x < size.columns
                && 0 <= y
                && y < size.rows
                && tiles[x as usize][y as usize].is_some()
            {
                *self.selected.borrow_mut() = Some(pos);
            }
        }
        self.widget.redraw();
    }

    pub fn on_press_tile(&mut self) {
        if *self.drawing.borrow() || *self.mode.borrow() != Mode::Playing
        {
            return;
        }
        let pos = *self.selected.borrow();
        if let Some(pos) = pos {
            self.delete_tile(pos);
        }
    }

    pub fn on_click_tile(&mut self) {
        if *self.drawing.borrow() || *self.mode.borrow() != Mode::Playing
        {
            return;
        }
        let (tile_width, tile_height) = self.get_tile_size();
        let x = (fltk::app::event_x() - self.widget.x()) / tile_width;
        let y = (fltk::app::event_y() - self.widget.y()) / tile_height;
        *self.selected.borrow_mut() = None;
        self.delete_tile(Pos::new(x as i32, y as i32));
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
        let color = self.tiles.borrow()[pos.x as usize][pos.y as usize];
        if color.is_none() {
            return;
        }
        let color = color.unwrap();
        if !self.is_legal(&pos, color) {
            return;
        }
        self.dim_adjoining(&pos, &color);
    }

    fn is_legal(&self, pos: &Pos, color: Color) -> bool {
        // A legal click is on a colored tile that is adjacent to another
        // tile of the same color.
        let tiles = &*self.tiles.borrow();
        let size = *self.size.borrow();
        let x = pos.x as usize;
        let y = pos.y as usize;
        let color = Some(color);
        (x > 0 && tiles[x - 1][y] == color)
            || (x + 1 < size.columns as usize && tiles[x + 1][y] == color)
            || (y > 0 && tiles[x][y - 1] == color)
            || (y + 1 < size.rows as usize && tiles[x][y + 1] == color)
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
            let x = adjoining_pos.x as usize;
            let y = adjoining_pos.y as usize;
            tiles[x][y] = Some(tiles[x][y].unwrap().darker());
        }
        fltk::app::sleep(TINY_DELAY);
        self.widget.redraw();
        #[allow(clippy::clone_on_copy)] // The clone is needed
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
        if !(0 <= x && x < size.columns && 0 <= y && y < size.rows) {
            return; // Fallen off an edge
        }
        let tiles = &*self.tiles.borrow();
        if self.adjoining.borrow().contains(&pos)
            || tiles[x as usize][y as usize] != Some(color)
        {
            return; // Color doesn't match or already done
        }
        self.adjoining.borrow_mut().insert(pos);
        self.populate_adjoining(Pos::new(x - 1, y), color);
        self.populate_adjoining(Pos::new(x + 1, y), color);
        self.populate_adjoining(Pos::new(x, y - 1), color);
        self.populate_adjoining(Pos::new(x, y + 1), color);
    }

    pub fn delete_adjoining(&mut self) {
        let tiles = &mut *self.tiles.borrow_mut();
        for &pos in self.adjoining.borrow().iter() {
            tiles[pos.x as usize][pos.y as usize] = None
        }
        self.adjoining.borrow_mut().clear();
        fltk::app::sleep(TINY_DELAY);
        self.widget.redraw();
        #[allow(clippy::clone_on_copy)] // The clone is needed
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
        let mut new_selected: Option<Pos> = None;
        if let Some(selected) = *self.selected.borrow() {
            let x = selected.x as usize;
            let y = selected.y as usize;
            let tiles = &*self.tiles.borrow();
            if tiles[x][y].is_none() {
                let size = *self.size.borrow();
                new_selected =
                    Some(Pos::new(size.rows / 2, size.columns / 2));
            }
        }
        if new_selected.is_some() {
            *self.selected.borrow_mut() = new_selected;
        }
        self.check_game_over();
    }

    fn move_tiles(&mut self) {
        let tiles = self.tiles.clone();
        let size = *self.size.borrow();
        let mut moved = true;
        let mut already_moved = PosForPos::new();
        while moved {
            moved = false;
            for x in board_util::ripple(size.columns as usize) {
                for y in board_util::ripple(size.rows as usize) {
                    #[allow(clippy::collapsible_if)] // Clippy is wrong
                    if tiles.borrow()[x][y].is_some() {
                        if self.move_if_possible(
                            Pos::new(x as i32, y as i32),
                            &mut already_moved,
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
        pos: Pos,
        already_moved: &mut PosForPos,
    ) -> bool {
        let empties = self.get_empty_neighbours(pos);
        if !empties.is_empty() {
            let (do_move, new_pos) =
                self.nearest_to_middle(pos, &empties);
            if let Some(value) = already_moved.get(&new_pos) {
                if value == &pos {
                    return false; // avoid endless loop back and forth
                }
            }
            if do_move {
                let tiles = &mut *self.tiles.borrow_mut();
                let x = pos.x as usize;
                let y = pos.y as usize;
                tiles[new_pos.x as usize][new_pos.y as usize] =
                    tiles[x][y];
                tiles[x][y] = None;
                already_moved.insert(pos, new_pos);
                let delay =
                    0.2_f64.max(*self.delay_ms.borrow() as f64 / 7000.0);
                #[allow(clippy::clone_on_copy)] // The clone is needed
                let sender = self.sender.clone();
                fltk::app::add_timeout(delay, move || {
                    sender.send(Action::Redraw);
                });
                return true;
            }
        }
        false
    }

    pub fn redraw(&mut self) {
        self.widget.redraw();
    }

    fn get_empty_neighbours(&mut self, pos: Pos) -> PosSet {
        let size = *self.size.borrow();
        let x = pos.x;
        let y = pos.y;
        let mut neighbours = PosSet::new();
        let tiles = self.tiles.clone();
        for new_pos in [
            Pos::new(x - 1, y),
            Pos::new(x + 1, y),
            Pos::new(x, y - 1),
            Pos::new(x, y + 1),
        ]
        .iter()
        {
            if 0 <= new_pos.x
                && new_pos.x < size.columns
                && 0 <= new_pos.y
                && new_pos.y < size.rows
                && tiles.borrow()[new_pos.x as usize][new_pos.y as usize]
                    .is_none()
            {
                neighbours.insert(*new_pos);
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
        let color = tiles.borrow()[x as usize][y as usize].unwrap();
        let size = *self.size.borrow();
        let mid_x = size.columns / 2;
        let mid_y = size.rows / 2;
        let old_radius = ((mid_x - x) as f64).hypot((mid_y - y) as f64);
        let mut shortest_radius = f64::NAN;
        let mut radius_pos = Pos::default(); // invalid
        for new_pos in empties.iter() {
            let nx = new_pos.x;
            let ny = new_pos.y;
            if self.is_square(new_pos) {
                let mut new_radius =
                    ((mid_x - nx) as f64).hypot((mid_y - ny) as f64);
                if self.is_legal(new_pos, color) {
                    // Make same colors slightly attractive
                    new_radius -= 0.1;
                }
                if !radius_pos.is_valid() || shortest_radius > new_radius
                {
                    shortest_radius = new_radius;
                    radius_pos = *new_pos;
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
        (x > 0 && tiles[x as usize - 1][y as usize].is_some())
            || (x + 1 < size.columns
                && tiles[x as usize + 1][y as usize].is_some())
            || (y > 0 && tiles[x as usize][y as usize - 1].is_some())
            || (y + 1 < size.rows
                && tiles[x as usize][y as usize + 1].is_some())
    }

    pub fn check_game_over(&mut self) {
        let (user_won, can_move) = self.check_tiles();
        *self.mode.borrow_mut() = if user_won {
            Mode::UserWon
        } else if can_move {
            Mode::Playing
        } else {
            Mode::GameOver
        };
        if user_won {
            self.sender.send(Action::UserWon);
        } else if !can_move {
            let size = *self.size.borrow();
            let tiles = &mut *self.tiles.borrow_mut();
            for column in 0..size.columns {
                for row in 0..size.rows {
                    let x = column as usize;
                    let y = row as usize;
                    if let Some(color) = tiles[x][y] {
                        tiles[x][y] = Some(color.darker());
                    }
                }
            }
            self.sender.send(Action::GameOver);
        }
        self.widget.redraw();
    }

    fn check_tiles(&mut self) -> (bool, bool) {
        let mut count_for_color = HashMap::<Color, u32>::new();
        let mut user_won = true;
        let mut can_move = false;
        let size = *self.size.borrow();
        let tiles = &*self.tiles.borrow();
        for column in 0..size.columns {
            for row in 0..size.rows {
                if let Some(color) = tiles[column as usize][row as usize]
                {
                    if let Some(count) = count_for_color.get_mut(&color) {
                        *count += 1;
                    } else {
                        count_for_color.insert(color, 1);
                    }
                    user_won = false;
                    if self.is_legal(&Pos::new(column, row), color) {
                        can_move = true;
                    }
                }
            }
        }
        for (_, count) in count_for_color.iter() {
            if *count == 1 {
                can_move = false;
                break;
            }
        }
        (user_won, can_move)
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
    let mode = board.mode.clone();
    board.widget.handle(move |_, event| {
        if *drawing.borrow() || *mode.borrow() != Mode::Playing {
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
    let mode = board.mode.clone();
    let selected = board.selected.clone();
    let tiles = board.tiles.clone();
    let size = board.size.clone();
    board.widget.draw(move |widget| {
        if *drawing.borrow() {
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
        if *mode.borrow() != Mode::Playing {
            fltk::draw::set_font(fltk::enums::Font::TimesBoldItalic, 48);
            if *mode.borrow() == Mode::GameOver {
                fltk::draw::set_draw_color(Color::Green);
                fltk::draw::draw_text2(
                    "Game Over!",
                    x1,
                    y1,
                    width,
                    height,
                    fltk::enums::Align::Center,
                );
            } else if *mode.borrow() == Mode::UserWon {
                fltk::draw::set_draw_color(Color::Red);
                fltk::draw::draw_text2(
                    "You Won!",
                    x1,
                    y1,
                    width,
                    height,
                    fltk::enums::Align::Center,
                );
            }
            fltk::draw::set_draw_color(Color::Black);
        }
        // *MUST* restore the line style after custom drawing
        fltk::draw::set_line_style(fltk::draw::LineStyle::Solid, 0);
        *drawing.borrow_mut() = false;
    });
}
