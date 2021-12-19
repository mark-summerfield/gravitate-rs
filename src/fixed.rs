// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::util::capitalize_first;
use chrono::prelude::*;
use fltk::enums::Color;
use std::env;
use std::sync;

pub static APPNAME: &str = "Gravitate";
pub static VERSION: &str = "8.0.1";
pub const ABOUT_ICON: &str = include_str!("../images/about.svg");
pub const HELP_ICON: &str = include_str!("../images/help.svg");
pub const ICON: &str = include_str!("../images/gravitate.svg");
pub const NEW_ICON: &str = include_str!("../images/new.svg");
pub const OPTIONS_ICON: &str = include_str!("../images/options.svg");
pub const QUIT_ICON: &str = include_str!("../images/quit.svg");
pub const PAD: i32 = 6;
pub const TOOLBUTTON_SIZE: i32 = 28;
pub const TOOLBAR_HEIGHT: i32 = ((TOOLBUTTON_SIZE * 3) / 2) + (2 * PAD);
pub const BUTTON_HEIGHT: i32 = 30;
pub const BUTTON_WIDTH: i32 = 70;
pub const SCALE_MIN: f32 = 0.5;
pub const SCALE_MAX: f32 = 3.5;
pub const SIZE_MIN: u8 = 5;
pub const SIZE_MAX: u8 = 30;
pub const COLORS_MIN: u8 = 3;
pub const DELAY_MS_MIN: u16 = 0;
pub const DELAY_MS_MAX: u16 = 1000;
pub const MESSAGE_DELAY: f64 = 10.0; // seconds
pub const TINY_DELAY: f64 = 0.005; // seconds

pub const MAROON: Color = Color::from_hex(0x800000);
pub const BROWN: Color = Color::from_hex(0x9A6324);
pub const OLIVE: Color = Color::from_hex(0x808000);
pub const TEAL: Color = Color::from_hex(0x469990);
pub const NAVY: Color = Color::from_hex(0x000075);
pub const BLACK: Color = Color::from_hex(0x000000);
pub const RED: Color = Color::from_hex(0xE6194B);
pub const ORANGE: Color = Color::from_hex(0xF58231);
pub const YELLOW: Color = Color::from_hex(0xFFE119);
pub const LIME: Color = Color::from_hex(0xBFEF45);
pub const GREEN: Color = Color::from_hex(0x3CB44B);
pub const CYAN: Color = Color::from_hex(0x42D4F4);
pub const BLUE: Color = Color::from_hex(0x4363D8);
pub const PURPLE: Color = Color::from_hex(0x911EB4);
pub const MAGENTA: Color = Color::from_hex(0xF032E6);
pub const GREY: Color = Color::from_hex(0xA9A9A9);
pub const PINK: Color = Color::from_hex(0xFABED4);
pub const APRICOT: Color = Color::from_hex(0xFFD8B1);
pub const BEIGE: Color = Color::from_hex(0xFFFAC8);
pub const MINT: Color = Color::from_hex(0xAAFFC3);
pub const LAVENDER: Color = Color::from_hex(0xDCBEFF);
pub const WHITE: Color = Color::from_hex(0xFFFFFF);

pub static COLORS: state::Storage<sync::RwLock<Vec<Color>>> =
    state::Storage::new();

pub fn initialize_colors() {
    let colors = vec![MAROON, BROWN, OLIVE, TEAL, NAVY, BLACK, RED, ORANGE,
        YELLOW, LIME, GREEN, CYAN, BLUE, PURPLE, MAGENTA, GREY, PINK,
        APRICOT, BEIGE, MINT, LAVENDER, WHITE];
    COLORS.set(sync::RwLock::new(colors));
}

#[derive(Copy, Clone, Debug)]
pub enum Action {
    New,
    Options,
    About,
    Help,
    Quit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    ClickTile,
    PressTile,
    DeleteAdjoining,
    CloseUp,
    Redraw,
    UpdatedScore(u16),
    GameOver,
}

pub enum Arrow {
    Left,
    Right,
    Up,
    Down,
}

pub fn about_html() -> String {
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
<p>Rust {} • fltk-rs {} • FLTK {} • {}/{}</p>
</center></font>",
        APPNAME,
        VERSION,
        year,
        rustc_version_runtime::version(),
        fltk::app::crate_version(),
        fltk::app::version_str(),
        capitalize_first(env::consts::OS),
        env::consts::ARCH
    )
}

pub static HELP_HTML: &str = "<body>
<p><center><font color=navy size=7em><b>Gravitate</b></font></center></p>
<font color=blue size=5em>The purpose of the game is to remove all the
tiles.</font>
<p>
<font color=#008000 size=4em>
Click a tile that has at least one vertically or horizontally adjoining tile
of the same color to remove it and any vertically or horizontally adjoining
tiles of the same color, and <i>their</i> vertically or horizontally
adjoining tiles, and so on. <i>(So clicking a tile with no adjoining tiles
of the same color does nothing.)</i> The more tiles that are removed in one
go, the higher the score.
</font>
</p>
<table border=1 align=center>
<font color=blue>
<tr><th>Key</th><th>Action</th></tr>
<tr><td><b>a</b></td><td>Show About box</td></tr>
<tr><td><b>h</b> or <b>F1</b></td><td>Show this Help window</td></tr>
<tr><td><b>n</b></td><td>New Game</td></tr>
<tr><td><b>o</b></td><td>View or Edit Options</td></tr>
<tr><td><b>q</b> or <b>Esc</b></td><td>Quit</td></tr>
<tr><td><b>←</b></td><td>Move the focus left</td></tr>
<tr><td><b>→</b></td><td>Move the focus right</td></tr>
<tr><td><b>↑</b></td><td>Move the focus up</td></tr>
<tr><td><b>↓</b></td><td>Move the focus down</td></tr>
<tr><td><b>Space</b></td><td>Click the focused tile</td></tr>
</font>
</table>
<font color=#008000>
Gravitate works like TileFall and the SameGame except that instead of tiles
falling to the bottom and moving off to the left, they “gravitate” to the
middle.</font>
</body>";
