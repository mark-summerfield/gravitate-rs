// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::fixed::{APPNAME, BUTTON_HEIGHT, BUTTON_WIDTH, ICON};
use fltk::prelude::*;

pub struct Form {
    form: fltk::window::Window,
}

impl Form {
    pub fn new(
        title: &str,
        html_text: &str,
        modal: bool,
        width: i32,
        height: i32,
    ) -> Self {
        let image = fltk::image::SvgImage::from_data(ICON).unwrap();
        let mut form = fltk::window::Window::default()
            .with_size(width, height)
            .with_label(&format!("{} — {}", title, APPNAME));
        form.set_icon(Some(image));
        let mut vbox =
            fltk::group::Flex::default().size_of_parent().column();
        fltk::misc::HelpView::default().set_value(html_text);
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
        form.make_modal(modal);
        form.show();
        ok_button.set_callback({
            let mut form = form.clone();
            move |_| {
                form.hide();
            }
        });
        if modal {
            while form.shown() {
                fltk::app::wait();
            }
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
