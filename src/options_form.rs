// Copyright © 2021-23 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::fixed::{
    APPNAME, BUTTON_HEIGHT, BUTTON_WIDTH, COLORS, COLORS_MIN, DELAY_MS_MAX,
    DELAY_MS_MIN, ICON, PAD, SCALE_MAX, SCALE_MIN, SIZE_MAX, SIZE_MIN,
};
use crate::util;
use fltk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Form {
    form: fltk::window::Window,
    pub(crate) ok: Rc<RefCell<bool>>,
}

impl Form {
    pub fn default() -> Self {
        let ok = Rc::from(RefCell::from(false));
        let mut form = make_form();
        let mut vbox =
            fltk::group::Flex::default().size_of_parent().column();
        vbox.set_margin(PAD);
        vbox.set_pad(PAD);
        let spinners = make_spinners();
        let (button_row, mut buttons) = make_buttons();
        vbox.set_size(&button_row, BUTTON_HEIGHT);
        vbox.end();
        form.end();
        form.make_modal(true);
        add_event_handlers(
            &mut form,
            &spinners,
            &mut buttons,
            Rc::clone(&ok),
        );
        form.show();
        while form.shown() {
            fltk::app::wait();
        }
        Self { form, ok }
    }
}

impl Drop for Form {
    fn drop(&mut self) {
        fltk::app::delete_widget(self.form.clone());
    }
}

struct Spinners {
    columns_spinner: fltk::misc::Spinner,
    rows_spinner: fltk::misc::Spinner,
    max_colors_spinner: fltk::misc::Spinner,
    delay_ms_spinner: fltk::misc::Spinner,
    scale_spinner: fltk::misc::Spinner,
}

struct Buttons {
    ok_button: fltk::button::Button,
    cancel_button: fltk::button::Button,
}

fn make_form() -> fltk::window::Window {
    let image = fltk::image::SvgImage::from_data(ICON).unwrap();
    let mut form = fltk::window::Window::default()
        .with_size(200, 200)
        .with_label(&format!("Options — {APPNAME}"));
    if let Some(window) = fltk::app::first_window() {
        form.set_pos(window.x() + 50, window.y() + 100);
    }
    form.set_icon(Some(image));
    form
}

fn make_spinners() -> Spinners {
    let config = CONFIG.get().read().unwrap();
    let mut columns_spinner = make_row(
        "Co&lumns",
        config.board_columns as f64,
        "Columns (default 9)",
        SIZE_MIN as f64,
        SIZE_MAX as f64,
        1.0,
    );
    columns_spinner.take_focus().unwrap();
    let rows_spinner = make_row(
        "&Rows",
        config.board_rows as f64,
        "Rows (default 9)",
        SIZE_MIN as f64,
        SIZE_MAX as f64,
        1.0,
    );
    let limit = COLORS.get().read().unwrap().len();
    let max_colors_spinner = make_row(
        "&Max. Colors",
        config.board_maxcolors as f64,
        "Max. Colors (default 4)",
        COLORS_MIN as f64,
        limit as f64,
        1.0,
    );
    let delay_ms_spinner = make_row(
        "&Delay (ms)",
        config.board_delay_ms as f64,
        "Delay to show tile movement (default 250 milliseconds)",
        DELAY_MS_MIN as f64,
        DELAY_MS_MAX as f64,
        1.0,
    );
    let scale_spinner = make_row(
        "&Scale",
        config.window_scale as f64,
        "User interface scale (default 1.0)",
        SCALE_MIN as f64,
        SCALE_MAX as f64,
        0.1,
    );
    Spinners {
        columns_spinner,
        rows_spinner,
        max_colors_spinner,
        delay_ms_spinner,
        scale_spinner,
    }
}

fn make_row(
    label: &str,
    value: f64,
    tooltip: &str,
    minimum: f64,
    maximum: f64,
    step: f64,
) -> fltk::misc::Spinner {
    let row = fltk::group::Flex::default().row();
    let mut label = fltk::button::Button::default()
        .with_label(label)
        .with_align(fltk::enums::Align::Inside | fltk::enums::Align::Left);
    label.set_frame(fltk::enums::FrameType::NoBox);
    let mut spinner = fltk::misc::Spinner::default();
    spinner.set_value(value);
    spinner.set_step(step);
    spinner.set_range(minimum, maximum);
    spinner.set_tooltip(tooltip);
    row.end();
    label.set_callback({
        let mut spinner = spinner.clone();
        move |_| {
            spinner.take_focus().unwrap();
        }
    });
    spinner
}

fn make_buttons() -> (fltk::group::Flex, Buttons) {
    let mut row = fltk::group::Flex::default().size_of_parent().row();
    fltk::frame::Frame::default(); // pad left of buttons
    let ok_button = fltk::button::Button::default().with_label("&OK");
    let cancel_button =
        fltk::button::Button::default().with_label("&Cancel");
    fltk::frame::Frame::default(); // pad right of buttons
    row.set_size(&ok_button, BUTTON_WIDTH);
    row.set_size(&cancel_button, BUTTON_WIDTH);
    row.end();
    (row, Buttons { ok_button, cancel_button })
}

fn add_event_handlers(
    form: &mut fltk::window::Window,
    spinners: &Spinners,
    buttons: &mut Buttons,
    ok: Rc<RefCell<bool>>,
) {
    buttons.ok_button.set_callback({
        let columns_spinner = spinners.columns_spinner.clone();
        let rows_spinner = spinners.rows_spinner.clone();
        let max_colors_spinner = spinners.max_colors_spinner.clone();
        let delay_ms_spinner = spinners.delay_ms_spinner.clone();
        let scale_spinner = spinners.scale_spinner.clone();
        let mut form = form.clone();
        move |_| {
            *ok.borrow_mut() = true;
            let mut config = CONFIG.get().write().unwrap();
            config.board_columns = columns_spinner.value() as u8;
            config.board_rows = rows_spinner.value() as u8;
            config.board_maxcolors = max_colors_spinner.value() as u8;
            config.board_delay_ms = delay_ms_spinner.value() as u16;
            let old_scale = config.window_scale;
            let new_scale = scale_spinner.value() as f32;
            config.window_scale = new_scale;
            fltk::app::set_screen_scale(0, new_scale);
            form.hide();
        }
    });
    buttons.cancel_button.set_callback({
        let mut form = form.clone();
        move |_| {
            form.hide();
        }
    });
}
