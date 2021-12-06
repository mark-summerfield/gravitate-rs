// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::action::Action;
use crate::fixed::COLORS;
use fltk::enums::Color;
use fltk::prelude::*;
use rand::{prelude::*, seq::SliceRandom};
use std::cell::RefCell;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

const BACKGROUND_COLOR: Color = Color::from_hex(0xFFFEE0);

pub struct Coord {
    x: u8,
    y: u8,
}

impl std::fmt::Display for Coord {
    fn fmt(&self, out: &mut std::fmt::Formatter) -> fmt::Result {
        write!(out, "({},{})", self.x, self.y)
    }
}

pub struct Board {
    widget: fltk::widget::Widget,
    drawing: Rc<RefCell<bool>>,
    game_over: Rc<RefCell<bool>>,
    selected: Rc<RefCell<Option<Coord>>>,
    tiles: Rc<RefCell<Vec<Vec<Option<Color>>>>>,
    columns: Rc<RefCell<u8>>,
    rows: Rc<RefCell<u8>>,
    delay_ms: Rc<RefCell<u16>>,
    score: Rc<RefCell<u16>>,
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
            delay_ms: Rc::default(),
            score: Rc::default(),
            sender,
        };
        add_event_handler(&mut board, sender);
        add_draw_handler(&mut board);
        board.new_game();
        board
    }

    pub fn new_game(&mut self) {
        *self.drawing.borrow_mut() = true;
        *self.game_over.borrow_mut() = true;
        *self.selected.borrow_mut() = None;
        *self.score.borrow_mut() = 0;
        let maxcolors = {
            let config = CONFIG.get().read().unwrap();
            *self.columns.borrow_mut() = config.board_columns;
            *self.rows.borrow_mut() = config.board_rows;
            *self.delay_ms.borrow_mut() = config.board_delay_ms;
            config.board_maxcolors as usize
        };
        let mut rng = rand::thread_rng();
        let colors: Vec<Color> = {
            let colors = COLORS.get().read().unwrap();
            colors.choose_multiple(&mut rng, maxcolors).cloned().collect()
        };
        let columns = *self.columns.borrow() as usize;
        let rows = *self.rows.borrow() as usize;
        let mut tiles = Vec::with_capacity(columns);
        for column in 0..columns {
            tiles.push(Vec::with_capacity(rows));
            for row in 0..rows {
                let color = colors.choose(&mut rng);
                let color = if color.is_some() {
                    Some(color.unwrap().clone())
                } else {
                    None
                };
                tiles[column].push(color);
            }
        }
        println!("{:#?}", tiles); // TODO
        *self.tiles.borrow_mut() = tiles;
        *self.game_over.borrow_mut() = false;
        *self.drawing.borrow_mut() = false;
        self.sender.send(Action::NewGame);
        self.sender.send(Action::UpdatedScore(*self.score.borrow()));
        // self.redraw(); FIXME
    }

    pub fn move_up(&mut self) {
        let mut moved = false;
        println!("board.move_up");
        if moved {
            // self.sender.send(Action::UpdatedScore(score));
            self.widget.redraw();
        }
    }

    pub fn move_down(&mut self) {
        let mut moved = false;
        println!("board.move_down");
        if moved {
            // self.sender.send(Action::UpdatedScore(score));
            self.widget.redraw();
        }
    }

    pub fn move_left(&mut self) {
        let mut moved = false;
        println!("board.move_left");
        if moved {
            // self.sender.send(Action::UpdatedScore(score));
            self.widget.redraw();
        }
    }

    pub fn move_right(&mut self) {
        let mut moved = false;
        println!("board.move_right");
        if moved {
            // self.sender.send(Action::UpdatedScore(score));
            self.widget.redraw();
        }
    }

    pub fn click_tile(&mut self) {
        let mut moved = false;
        let x = fltk::app::event_x() - self.widget.x();
        let y = fltk::app::event_y() - self.widget.y();
        println!("board.click_tile at {},{}", x, y);
        if moved {
            // self.sender.send(Action::UpdatedScore(score));
            self.widget.redraw();
        }
    }

    pub fn press_tile(&mut self) {
        println!("board.press_tile");
        if let Some(coord) = &*self.selected.borrow() {
            let mut valid_press = false;
            // TODO click selected if not None else ignore
            if valid_press {
                // self.sender.send(Action::UpdatedScore(score));
                self.widget.redraw();
            }
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
    let columns = *board.columns.borrow();
    let rows = *board.rows.borrow();
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
        draw_background(x1, y1, width, height);
        draw_tiles(
            x1,
            y1,
            width,
            height,
            columns,
            rows,
            &*tiles.borrow(),
        );
        // TODO draw focus rect on selected if there is a selected
        // *MUST* restore the line style after custom drawing
        fltk::draw::set_line_style(fltk::draw::LineStyle::Solid, 0);
        *drawing.borrow_mut() = false;
    });
}

fn draw_background(x1: i32, y1: i32, width: i32, height: i32) {
    fltk::draw::set_draw_color(BACKGROUND_COLOR);
    fltk::draw::draw_rect_fill(x1, y1, width, height, BACKGROUND_COLOR);
}

fn draw_tiles(
    x1: i32,
    y1: i32,
    width: i32,
    height: i32,
    columns: u8,
    rows: u8,
    tiles: &Vec<Vec<Option<Color>>>,
) {
    println!("columns={} rows={} tiles.len()={}", columns, rows, tiles.len()); // TODO
    if columns == 0 || rows == 0 || tiles.len() == 0 {
        return;
    }
    let tile_width = width / columns as i32;
    let tile_height = height / rows as i32;
    for column in 0..columns as usize {
        let x = x1 + (tile_width * column as i32);
        for row in 0..rows as usize {
            let y = y1 + (tile_height * row as i32);
            let color = Some(Color::Red); // TODO get from tiles
            if let Some(color) = tiles[column][row] {
                draw_tile(x, y, tile_width, tile_height, color);
            }
        }
    }
}

fn draw_tile(x: i32, y: i32, width: i32, height: i32, color: Color) {
    fltk::draw::draw_box(
        fltk::enums::FrameType::RoundUpBox,
        x,
        y,
        width,
        height,
        color,
    );
    fltk::draw::draw_rect(x, y, width, height);
}
