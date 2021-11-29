// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

pub static APPNAME: &str = "Gravitate";
pub static VERSION: &str = "8.0.0";
pub const ICON: &[u8] = include_bytes!("../images/gravitate.png");
pub const BUTTON_HEIGHT: i32 = 40;
pub const BUTTON_WIDTH: i32 = 70;
pub const SCALE_MIN: f32 = 0.5;
pub const SCALE_MAX: f32 = 3.5;
pub const SIZE_MIN: u8 = 5;
pub const SIZE_MAX: u8 = 30;
pub const COLORS_MIN: u8 = 4;
pub const DELAY_MS_MIN: u16 = 0;
pub const DELAY_MS_MAX: u16 = 1000;
