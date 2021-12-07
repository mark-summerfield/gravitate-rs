// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use fltk::enums::Color;
use std::fmt;

pub const BACKGROUND_COLOR: Color = Color::from_hex(0xFFFEE0);

pub type Tiles = Vec<Vec<Option<Color>>>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: u8,
    pub y: u8,
}

impl Coord {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, out: &mut std::fmt::Formatter) -> fmt::Result {
        write!(out, "({},{})", self.x, self.y)
    }
}

pub fn get_tile_size(
    columns: i32,
    rows: i32,
    width: i32,
    height: i32,
) -> (i32, i32) {
    let tile_width = width / columns as i32;
    let tile_height = height / rows as i32;
    (tile_width, tile_height)
}

pub fn draw_background(x1: i32, y1: i32, width: i32, height: i32) {
    fltk::draw::set_draw_color(BACKGROUND_COLOR);
    fltk::draw::draw_rect_fill(x1, y1, width, height, BACKGROUND_COLOR);
}

pub fn draw_tiles(
    x1: i32,
    y1: i32,
    width: i32,
    height: i32,
    columns: u8,
    rows: u8,
    tiles: &Vec<Vec<Option<Color>>>,
    selected: Option<Coord>,
) {
    let (tile_width, tile_height) =
        get_tile_size(columns as i32, rows as i32, width, height);
    for column in 0..columns as usize {
        let x = x1 + (tile_width * column as i32);
        for row in 0..rows as usize {
            let y = y1 + (tile_height * row as i32);
            if let Some(color) = tiles[column][row] {
                draw_tile(x, y, tile_width, tile_height, color);
                if let Some(coord) = selected {
                    if coord.x as usize == column
                        && coord.y as usize == row
                    {
                        draw_focus(x, y, tile_width, tile_height);
                    }
                }
            }
        }
    }
}

fn draw_tile(x: i32, y: i32, width: i32, height: i32, color: Color) {
    fltk::draw::draw_box(
        fltk::enums::FrameType::RoundUpBox,
        x,
        y,
        width,
        height,
        color,
    );
    fltk::draw::draw_rect(x, y, width, height);
}

fn draw_focus(x: i32, y: i32, width: i32, height: i32) {
    fltk::draw::set_line_style(fltk::draw::LineStyle::Dot, 2);
    fltk::draw::draw_rect_with_color(
        x + 4,
        y + 4,
        width - 8,
        height - 8,
        Color::White,
    );
    fltk::draw::draw_rect_with_color(
        x + 3,
        y + 3,
        width - 6,
        height - 6,
        Color::Black,
    );
    fltk::draw::set_line_style(fltk::draw::LineStyle::Solid, 0);
}

pub fn ripple(n: usize) -> Vec<usize> {
    // The purpose is to favor the player especially in the end game by
    // working from the middle out.
    let mut ripple = Vec::with_capacity(n);
    let middle = n / 2;
    for (i, j) in (middle..n).zip((0..middle).rev()) {
        ripple.push(i);
        ripple.push(j);
    }
    if (n % 2) != 0 {
        ripple.push(n - 1);
    }
    ripple
}
