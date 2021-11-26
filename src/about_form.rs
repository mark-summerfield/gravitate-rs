// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::fixed::{APPNAME, ICON, VERSION};
use crate::util::capitalize_first;
use chrono::prelude::*;
use fltk::{app, button, group, image, misc, prelude::*, window};
use std::env;

pub struct Form {
    form: window::Window,
}

impl Form {
    pub fn default() -> Self {
        const WIDTH: i32 = 400;
        const HEIGHT: i32 = 300;
        const BUTTON_SIZE: i32 = 40;
        const PAD: i32 = 10;
        let image = image::PngImage::from_data(ICON).unwrap();
        let mut form = window::Window::default()
            .with_size(WIDTH, HEIGHT)
            .with_label(&format!("About — {}", APPNAME));
        form.set_icon(Some(image));
        let mut pack = group::Pack::default()
            .with_size(WIDTH, HEIGHT)
            .center_of_parent()
            .with_type(group::PackType::Vertical);
        pack.set_spacing(PAD);
        let mut view = misc::HelpView::new(
            0,
            0,
            WIDTH,
            HEIGHT - (BUTTON_SIZE + PAD),
            "",
        );
        view.set_value(&about_html());
        // TODO pad button left & right or center the button
        let mut ok_button = button::Button::default()
            .with_size(0, BUTTON_SIZE)
            .with_label("&OK");
        pack.end();
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
            app::wait();
        }
        Self { form }
    }
}

impl Drop for Form {
    fn drop(&mut self) {
        app::delete_widget(self.form.clone());
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
        app::crate_version(),
        app::version_str(),
        capitalize_first(env::consts::OS),
        env::consts::ARCH
    )
}
