// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

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
    pub board_score: u16,
    pub board_highscore: u16,
    pub filename: String,
}

impl Config {
    pub fn new() -> Self {
        let mut config = Config::default();
        // TODO read from file(s) if poss
        config
    }

    pub fn save(&self) {
        println!("TODO Config.save");
        // TODO
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
            board_score: 0,
            board_highscore: 0,
            filename: String::new(),
        }
    }
}
