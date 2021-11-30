// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod about_form;
mod action;
mod application;
mod config;
mod fixed;
mod util;

use crate::application::Application;
use crate::fixed::{initialize_colors, APPNAME, SCALE_MAX, SCALE_MIN};
use std::{panic, sync};

pub static CONFIG: state::Storage<sync::RwLock<config::Config>> =
    state::Storage::new();

fn main() {
    panic::set_hook(Box::new(|info| {
        fltk::dialog::message_title(&format!("Error — {}", APPNAME));
        if let Some(sender) = info.payload().downcast_ref::<&str>() {
            fltk::dialog::message(
                util::x() - 200,
                util::y() - 100,
                sender,
            );
        } else {
            fltk::dialog::message(
                util::x() - 200,
                util::y() - 100,
                &info.to_string(),
            );
        }
    }));
    initialize_colors(); // *MUST* be done before CONFIG is created
    CONFIG.set(sync::RwLock::new(config::Config::new()));
    handle_commandline();
    let mut app = Application::new();
    app.run();
}

fn handle_commandline() {
    let mut scale = 0.0;
    for arg in std::env::args().skip(1) {
        if arg.starts_with("--scale=") {
            scale = num::clamp(
                arg.get(8..).unwrap().parse::<f32>().unwrap_or(1.0),
                SCALE_MIN,
                SCALE_MAX,
            );
        }
    }
    if !util::iszero32(scale) {
        let mut config = CONFIG.get().write().unwrap();
        config.window_scale = scale;
    }
    let config = CONFIG.get().read().unwrap();
    if !util::isone32(config.window_scale) {
        fltk::app::set_screen_scale(0, config.window_scale);
    }
}
