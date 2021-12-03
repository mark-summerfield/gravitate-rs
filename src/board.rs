// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::action::Action;
use fltk::enums::Color;
use fltk::prelude::*;
use std::cell::RefCell;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

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
    game_over: Rc<RefCell<bool>>,
    drawing: Rc<RefCell<bool>>,
    selected: Rc<RefCell<Option<Coord>>>,
    tiles: Rc<RefCell<Vec<Vec<Option<Color>>>>>,
}

impl Board {
    pub fn new(sender: fltk::app::Sender<Action>) -> Self {
        let mut board = Board {
            widget: fltk::widget::Widget::default(),
            game_over: Rc::default(),
            drawing: Rc::default(),
            selected: Rc::default(),
            tiles: Rc::default(),
        };
        add_event_handler(&mut board, sender);
        add_draw_handler(&mut board);
        board
    }

    pub fn move_up(&mut self) {
        println!("board.move_up");
    }

    pub fn move_down(&mut self) {
        println!("board.move_down");
    }

    pub fn move_left(&mut self) {
        println!("board.move_left");
    }

    pub fn move_right(&mut self) {
        println!("board.move_right");
    }

    pub fn click_tile(&mut self) {
        let x = fltk::app::event_x() - self.widget.x();
        let y = fltk::app::event_y() - self.widget.y();
        println!("board.click_tile at {},{}", x, y);
        self.widget.redraw();
    }

    pub fn press_tile(&mut self) {
        println!("board.press_tile");
        // TODO click selected if not None else ignore
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
    let game_over = board.game_over.clone();
    let drawing = board.drawing.clone();
    board.widget.handle(move |_, event| {
        // TODO if game_over or drawing just return false
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
    let game_over = board.game_over.clone();
    let drawing = board.drawing.clone();
    let selected = board.selected.clone();
    let tiles = board.tiles.clone();
    board.widget.draw(move |widget| {
        // TODO if game_over or drawing just return
        println!(
            "draw board: {}x{} tiles={:#?}",
            widget.width(),
            widget.height(),
            tiles
        );
    });
}
