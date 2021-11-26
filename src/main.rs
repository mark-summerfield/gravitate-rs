// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod about_form;
mod action;
mod application;
mod fixed;
mod util;

use crate::application::Application;
use crate::fixed::APPNAME;
use crate::util::center;
use fltk::{app, dialog};
use std::panic;

fn main() {
    panic::set_hook(Box::new(|info| {
        dialog::message_title(&format!("Error — {}", APPNAME));
        if let Some(sender) = info.payload().downcast_ref::<&str>() {
            dialog::message(center().0 - 200, center().1 - 100, sender);
        } else {
            dialog::message(
                center().0 - 200,
                center().1 - 100,
                &info.to_string(),
            );
        }
    }));
    // let query = read_query(); // TODO read gravitate.ini
    // command line args override .ini
    handle_commandline(); // TODO pass query
                          // TODO Pass query to application object
    let mut app = Application::new();
    app.run();
}

fn handle_commandline() {
    // TODO query passed in
    let mut scale: f32 = 1.0; // TODO delete
    for arg in std::env::args().skip(1) {
        if arg.starts_with("--scale=") {
            scale = num::clamp(
                // TODO scale → query.scale
                arg.get(8..).unwrap().parse::<f32>().unwrap_or(scale),
                0.5,
                2.5,
            );
        }
    }
    app::set_screen_scale(0, scale); // TODO scale → query.scale
}
