// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod about_form;
mod action;
mod application;
mod board;
mod board_util;
mod commandline;
mod config;
mod fixed;
mod mainwindow;
mod util;

use crate::application::Application;
use crate::fixed::{initialize_colors, APPNAME};
use std::{panic, sync};

pub static CONFIG: state::Storage<sync::RwLock<config::Config>> =
    state::Storage::new();

fn main() {
    panic::set_hook(Box::new(|info| {
        let err = dbg!(&info);
        fltk::dialog::message_title(&format!("Error — {}", APPNAME));
        let x = util::x() - 200;
        let y = util::y() - 100;
        fltk::dialog::message(x, y, &err.to_string());
    }));
    initialize_colors(); // *MUST* be done before CONFIG is created
    CONFIG.set(sync::RwLock::new(config::Config::new()));
    commandline::read(); // *MUST* be done after CONFIG is created
    let mut app = Application::new();
    app.run();
}
