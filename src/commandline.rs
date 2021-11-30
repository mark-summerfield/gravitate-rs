// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::fixed::{SCALE_MAX, SCALE_MIN};
use crate::util;

pub fn read() {
    let mut scale = 0.0;
    for arg in std::env::args().skip(1) {
        if arg.starts_with("--scale=") {
            scale = num::clamp(
                arg.get(8..).unwrap().parse::<f32>().unwrap_or(1.0),
                SCALE_MIN,
                SCALE_MAX,
            );
        } else if arg == "--help" || arg == "-h" || arg == "help" {
            println!(
                "usage: gravitate [--help] [--scale=N]
0.5 <= N <= 3.5"
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
