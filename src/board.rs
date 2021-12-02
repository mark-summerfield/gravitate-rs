// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use fltk::enums::Color;
use fltk::prelude::*;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct Board {
    inner: fltk::frame::Frame,
    game_over: Rc<RefCell<bool>>,
    drawing: Rc<RefCell<bool>>,
    tiles: Rc<RefCell<Vec<Vec<Option<Color>>>>>,
}

impl Board {
    pub fn new() -> Self {
        let mut inner = fltk::frame::Frame::default();
        let game_over = Rc::from(RefCell::from(false));
        let drawing = Rc::from(RefCell::from(false));
        let tiles = Rc::from(RefCell::from(vec![vec![None]]));
        inner.handle(move |inner, event| match event {
            fltk::enums::Event::Push => {
                inner.do_callback();
                inner.redraw();
                true
            }
            _ => false,
        });
        inner.draw(|inner| { // TODO acces board fields
            //let mut inner = inner.clone();
            //let game_over = Rc::clone(&game_over);
            println!(
                //"board draw: {}x{} game_over={:#?}", //tiles={:#?}
                "board draw: {}x{}",
                inner.width(),
                inner.height(),
            );
        });
        Self { inner, game_over, drawing, tiles }
    }

    pub fn on_click(&self) { // TODO get this called!
        println!(
            "on_click {},{}",
            fltk::app::event_x(),
            fltk::app::event_y()
        );
    }
}

impl Deref for Board {
    type Target = fltk::frame::Frame;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
