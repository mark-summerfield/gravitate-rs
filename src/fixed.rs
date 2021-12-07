// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use fltk::enums::Color;
use std::sync;

pub static APPNAME: &str = "Gravitate";
pub static VERSION: &str = "8.0.0";
pub const ABOUT_ICON: &str = include_str!("../images/about.svg");
pub const HELP_ICON: &str = include_str!("../images/help.svg");
pub const ICON: &str = include_str!("../images/gravitate.svg");
pub const NEW_ICON: &str = include_str!("../images/new.svg");
pub const OPTIONS_ICON: &str = include_str!("../images/options.svg");
pub const QUIT_ICON: &str = include_str!("../images/quit.svg");
pub const PAD: i32 = 6;
pub const TOOLBUTTON_SIZE: i32 = 28;
pub const TOOLBAR_HEIGHT: i32 = ((TOOLBUTTON_SIZE * 3) / 2) + (2 * PAD);
pub const BUTTON_HEIGHT: i32 = 40;
pub const BUTTON_WIDTH: i32 = 70;
pub const SCALE_MIN: f32 = 0.5;
pub const SCALE_MAX: f32 = 3.5;
pub const SIZE_MIN: u8 = 5;
pub const SIZE_MAX: u8 = 30;
pub const COLORS_MIN: u8 = 4;
pub const DELAY_MS_MIN: u16 = 0;
pub const DELAY_MS_MAX: u16 = 1000;
pub const MESSAGE_DELAY: f64 = 10.0; // seconds

pub enum Arrow {
    Left,
    Right,
    Up,
    Down,
}

pub static COLORS: state::Storage<sync::RwLock<Vec<Color>>> =
    state::Storage::new();

pub fn initialize_colors() {
    let colors = vec![
        Color::from_hex(0x0000A0),
        Color::from_hex(0x00A000),
        Color::from_hex(0x00A0A0),
        Color::from_hex(0xA00000),
        Color::from_hex(0xA000A0),
        Color::from_hex(0xA0A000),
        Color::from_hex(0xA0A0A0),
    ];
    COLORS.set(sync::RwLock::new(colors));
}
