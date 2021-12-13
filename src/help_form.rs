// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::fixed::{APPNAME, BUTTON_HEIGHT, BUTTON_WIDTH, ICON};
use fltk::prelude::*;

pub struct Form {
    form: fltk::window::Window,
}

impl Form {
    pub fn default() -> Self {
        const WIDTH: i32 = 460;
        const HEIGHT: i32 = 480;
        let image = fltk::image::SvgImage::from_data(ICON).unwrap();
        let mut form = fltk::window::Window::default()
            .with_size(WIDTH, HEIGHT)
            .with_label(&format!("Help — {}", APPNAME));
        form.set_icon(Some(image));
        let mut vbox =
            fltk::group::Flex::default().size_of_parent().column();
        fltk::misc::HelpView::default().set_value(HELP_HTML);
        let mut button_row =
            fltk::group::Flex::default().size_of_parent().row();
        fltk::frame::Frame::default(); // pad left of button
        let mut ok_button =
            fltk::button::Button::default().with_label("&OK");
        fltk::frame::Frame::default(); // pad right of button
        button_row.set_size(&ok_button, BUTTON_WIDTH);
        button_row.end();
        vbox.set_size(&button_row, BUTTON_HEIGHT);
        vbox.end();
        form.end();
        form.show();
        ok_button.set_callback({
            let mut form = form.clone();
            move |_| {
                form.hide();
            }
        });
        while form.shown() {
            fltk::app::wait();
        }
        Self { form }
    }

    pub fn show(&mut self) {
        self.form.show();
    }
}

impl Drop for Form {
    fn drop(&mut self) {
        fltk::app::delete_widget(self.form.clone());
    }
}

static HELP_HTML: &str = "<body>
<p><center><font color=navy size=7em><b>Gravitate</b></font></center></p>
<font color=blue size=5em>The purpose of the game is to remove all the
tiles.</font>
<p>
<font color=#008000 size=4em>
Click a tile that has at least one vertically or horizontally adjoining tile
of the same color to remove it and any vertically or horizontally adjoining
tiles of the same color, and <i>their</i> vertically or horizontally
adjoining tiles, and so on. <i>(So clicking a tile with no adjoining tiles
of the same color does nothing.)</i> The more tiles that are removed in one
go, the higher the score.
</font>
</p>
<table border=1 align=center>
<font color=blue>
<tr><th>Key</th><th>Action</th></tr>
<tr><td><b>a</b></td><td>Show About box</td></tr>
<tr><td><b>h</b> or <b>F1</b></td><td>Show this Help window</td></tr>
<tr><td><b>n</b></td><td>New Game</td></tr>
<tr><td><b>o</b></td><td>View or Edit Options</td></tr>
<tr><td><b>q</b> or <b>Esc</b></td><td>Quit</td></tr>
<tr><td><b>←</b></td><td>Move the focus left</td></tr>
<tr><td><b>→</b></td><td>Move the focus right</td></tr>
<tr><td><b>↑</b></td><td>Move the focus up</td></tr>
<tr><td><b>↓</b></td><td>Move the focus down</td></tr>
<tr><td><b>Space</b></td><td>Click the focused tile</td></tr>
</font>
</table>
<font color=#008000>
Gravitate works like TileFall and the SameGame except that instead of tiles
falling to the bottom and moving off to the left, they “gravitate” to the
middle.</font>
</body>";
