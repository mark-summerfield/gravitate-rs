// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::fixed::{
    APPNAME, COLORS, COLORS_MIN, DELAY_MS_MAX, DELAY_MS_MIN, SCALE_MAX,
    SCALE_MIN, SIZE_MAX, SIZE_MIN,
};
use crate::util;

#[derive(Clone, Debug)]
pub struct Config {
    pub window_x: i32,
    pub window_y: i32,
    pub window_height: i32,
    pub window_width: i32,
    pub window_scale: f32,
    pub board_columns: u8,
    pub board_rows: u8,
    pub board_maxcolors: u8,
    pub board_delay_ms: u16,
    pub board_highscore: u16,
    pub filename: std::path::PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let mut config = Config {
            filename: get_config_filename(),
            ..Default::default()
        };
        if let Ok(ini) = ini::Ini::load_from_file(&config.filename) {
            if let Some(properties) = ini.section(Some(WINDOW_SECTION)) {
                read_window_properties(properties, &mut config);
            }
            if let Some(properties) = ini.section(Some(BOARD_SECTION)) {
                read_board_properties(properties, &mut config);
            }
        }
        config
    }

    pub fn save(&self, x: i32, y: i32, width: i32, height: i32) {
        if self.filename.to_string_lossy() == "" {
            self.warning("failed to save configuration: no filename");
        } else {
            let mut ini = ini::Ini::new();
            ini.with_section(Some(WINDOW_SECTION))
                .set(X_KEY, x.to_string())
                .set(Y_KEY, y.to_string())
                .set(WIDTH_KEY, width.to_string())
                .set(HEIGHT_KEY, height.to_string())
                .set(SCALE_KEY, fltk::app::screen_scale(0).to_string());
            ini.with_section(Some(BOARD_SECTION))
                .set(COLUMNS_KEY, self.board_columns.to_string())
                .set(ROWS_KEY, self.board_rows.to_string())
                .set(MAX_COLORS_KEY, self.board_maxcolors.to_string())
                .set(DELAY_MS_KEY, self.board_delay_ms.to_string())
                .set(HIGH_SCORE_KEY, self.board_highscore.to_string());
            match ini.write_to_file(&self.filename) {
                Ok(_) => {}
                Err(err) => self.warning(&format!(
                    "failed to save configuration: {}",
                    err
                )),
            }
        }
    }

    fn warning(&self, message: &str) {
        fltk::dialog::message_title(&format!("Warning — {}", APPNAME));
        fltk::dialog::message(util::x() - 200, util::y() - 100, message);
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_x: -1,
            window_y: -1,
            window_height: 300,
            window_width: 260,
            window_scale: 1.0,
            board_columns: 9,
            board_rows: 9,
            board_maxcolors: 4,
            board_delay_ms: 250,
            board_highscore: 0,
            filename: std::path::PathBuf::new(),
        }
    }
}

fn get_config_filename() -> std::path::PathBuf {
    let mut dir = dirs::config_dir();
    let mut dot = "";
    if dir.is_none() {
        if std::env::consts::FAMILY == "unix" {
            dot = ".";
        }
        dir = dirs::home_dir();
    }
    if let Some(dir) = dir {
        // to_lowercase is for backwards compatability
        dir.join(format!("{}{}.ini", dot, APPNAME.to_lowercase()))
    } else {
        std::path::PathBuf::new()
    }
}

fn read_window_properties(
    properties: &ini::Properties,
    config: &mut Config,
) {
    let max_x = (fltk::app::screen_size().0 - 100.0) as i32;
    let max_y = (fltk::app::screen_size().1 - 100.0) as i32;
    if let Some(value) = properties.get(X_KEY) {
        config.window_x = util::get_num(value, 0, max_x, config.window_x)
    }
    if let Some(value) = properties.get(Y_KEY) {
        config.window_y = util::get_num(value, 0, max_y, config.window_y)
    }
    if let Some(value) = properties.get(WIDTH_KEY) {
        config.window_width =
            util::get_num(value, 200, max_x, config.window_width)
    }
    if let Some(value) = properties.get(HEIGHT_KEY) {
        config.window_height =
            util::get_num(value, 240, max_y, config.window_height)
    }
    if let Some(value) = properties.get(SCALE_KEY) {
        config.window_scale = util::get_num(
            value,
            SCALE_MIN,
            SCALE_MAX,
            config.window_scale,
        )
    }
}

fn read_board_properties(
    properties: &ini::Properties,
    config: &mut Config,
) {
    if let Some(value) = properties.get(COLUMNS_KEY) {
        config.board_columns =
            util::get_num(value, SIZE_MIN, SIZE_MAX, config.board_columns)
    }
    if let Some(value) = properties.get(ROWS_KEY) {
        config.board_rows =
            util::get_num(value, SIZE_MIN, SIZE_MAX, config.board_rows)
    }
    if let Some(value) = properties.get(MAX_COLORS_KEY) {
        let colors_max = COLORS.get().read().unwrap().len();
        config.board_maxcolors = util::get_num(
            value,
            COLORS_MIN,
            colors_max as u8,
            config.board_maxcolors,
        )
    }
    if let Some(value) = properties.get(DELAY_MS_KEY) {
        config.board_delay_ms = util::get_num(
            value,
            DELAY_MS_MIN,
            DELAY_MS_MAX,
            config.board_delay_ms,
        )
    }
    if let Some(value) = properties.get(HIGH_SCORE_KEY) {
        config.board_highscore =
            util::get_num(value, 0, 65535, config.board_highscore)
    }
}

// These names are for backwards compatability
static WINDOW_SECTION: &str = "Window";
static X_KEY: &str = "x";
static Y_KEY: &str = "y";
static WIDTH_KEY: &str = "width";
static HEIGHT_KEY: &str = "height";
static SCALE_KEY: &str = "scale";
static BOARD_SECTION: &str = "Board";
static COLUMNS_KEY: &str = "columns";
static ROWS_KEY: &str = "rows";
static MAX_COLORS_KEY: &str = "maxColors";
static DELAY_MS_KEY: &str = "delayMs";
static HIGH_SCORE_KEY: &str = "HighScore";
