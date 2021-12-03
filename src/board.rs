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
    drawing: Rc<RefCell<bool>>,
    game_over: Rc<RefCell<bool>>,
    selected: Rc<RefCell<Option<Coord>>>,
    tiles: Rc<RefCell<Vec<Vec<Option<Color>>>>>,
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
            sender
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
        // TODO choose config.maxcolors colors
        // let colors = { .... } // use a block to access Config r/o
        // TODO populate tiles
        // now access Config r/w
        println!("board.new_game");
        *self.game_over.borrow_mut() = false;
        *self.drawing.borrow_mut() = false;
        self.sender.send(Action::NewGame);
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
    board.widget.draw(move |widget| {
        if *drawing.borrow() || *game_over.borrow() {
            return;
        }
        println!(
            "draw board: {}x{} tiles={:#?}",
            widget.width(),
            widget.height(),
            tiles
        );
        // TODO horrible! And 0, 0 is top-left of app not of widget
        /*
        fltk::draw::set_draw_color(Color::Red);
        fltk::draw::set_line_style(fltk::draw::LineStyle::Solid, 5);
        fltk::draw::draw_line(0, 0, widget.width(), widget.height());
        fltk::draw::draw_line(widget.width(), 0, 0, widget.height());
        */
    });
}
