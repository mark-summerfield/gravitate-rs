// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::fixed::{APPNAME, BUTTON_HEIGHT, BUTTON_WIDTH, ICON, VERSION};
use crate::util::capitalize_first;
use chrono::prelude::*;
use fltk::prelude::*;
use std::env;

pub struct Form {
    form: fltk::window::Window,
}

impl Form {
    pub fn default() -> Self {
        const WIDTH: i32 = 400;
        const HEIGHT: i32 = 300;
        let image = fltk::image::PngImage::from_data(ICON).unwrap();
        let mut form = fltk::window::Window::default()
            .with_size(WIDTH, HEIGHT)
            .with_label(&format!("About — {}", APPNAME));
        form.set_icon(Some(image));
        let mut vbox =
            fltk::group::Flex::default().size_of_parent().column();
        fltk::misc::HelpView::default().set_value(&about_html());
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
        form.make_modal(true);
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
}

impl Drop for Form {
    fn drop(&mut self) {
        fltk::app::delete_widget(self.form.clone());
    }
}

fn about_html() -> String {
    let year = Local::today().year();
    let year = if year == 2021 {
        format!("{}", year)
    } else {
        format!("2021-{}", year - 2000)
    };
    format!(
        "<font face=\"Helvetica\"><center>
<h1><font color=\"navy\">{} v{}</font></h1>
<h3><font face=\"Helvetica\"
color=\"navy\">A TileFall/SameGame-like game.</font></h2>
</h3>
<h4>
<a href=\"http://www.qtrac.eu/gravitate.html\">www.qtrac.eu/gravitate.html</a>
</h4>
<h5><font face=\"Helvetica\"
color=\"green\">Copyright © {} Mark Summerfield.<br>
All rights reserved.</font></h5>
<h5><font face=\"Helvetica\" color=\"green\">License: GPLv3.</font></h5>
<p>fltk-rs {} • FLTK {} • {}/{}</p>
</center></font>",
        APPNAME,
        VERSION,
        year,
        fltk::app::crate_version(),
        fltk::app::version_str(),
        capitalize_first(env::consts::OS),
        env::consts::ARCH
    )
}
