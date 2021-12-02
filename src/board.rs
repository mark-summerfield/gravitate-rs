// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use fltk::prelude::*;

pub struct Board {
    inner: fltk::frame::Frame,
    // TODO tiles array
}

impl Board {
    pub fn new() -> Self {
        let mut frame = fltk::frame::Frame::default();
        frame.handle(move |frame, event| match event {
            fltk::enums::Event::Push => {frame.do_callback(); true}
            _ => false
        });
        frame.draw(move |frame| { // TODO base it on tiles array
        });
        Self { inner: frame }
    }
}

fltk::widget_extends!(Board, fltk::frame::Frame, inner);
