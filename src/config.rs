// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::fixed::APPNAME;
use dirs;
use std::{env, path};

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
    pub filename: path::PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let mut config = Config::default();
        config.filename = get_config_filename();
        if config.filename.exists() {
            println!("TODO Config.new read from {:?} if poss", config.filename); // TODO
        }
        config
    }

    pub fn save(&self) {
        if self.filename.to_string_lossy() == "" {
            eprintln!("failed to save configuration: no filename");
        } else {
            println!("TODO Config.save to {:?}", self.filename);
            // TODO
        }
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
            filename: path::PathBuf::new(),
        }
    }
}

fn get_config_filename() -> path::PathBuf {
    let mut dir = dirs::config_dir();
    let mut dot = "";
    if dir.is_none() {
        if env::consts::FAMILY == "unix" {
            dot = ".";
        }
        dir = dirs::home_dir();
    }
    if let Some(dir) = dir { // to_lowercase is for backwards compatability
        dir.join(format!("{}{}.ini", dot, APPNAME.to_lowercase()))
    } else {
        path::PathBuf::new()
    }
}
