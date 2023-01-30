// Copyright © 2021-23 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::util::Pos;
use fltk::enums::Color;

pub const BACKGROUND_COLOR: Color = Color::BackGround;

pub type Tiles = Vec<Vec<Option<Color>>>;

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Playing,
    GameOver,
    UserWon(bool), // true if new highscore
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Size {
    pub columns: i32,
    pub rows: i32,
}

impl Size {
    pub fn new(columns: i32, rows: i32) -> Self {
        Self { columns, rows }
    }
}

pub fn get_tile_size(
    columns: i32,
    rows: i32,
    width: i32,
    height: i32,
) -> (i32, i32) {
    let tile_width = width / columns;
    let tile_height = height / rows;
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
    size: Size,
    tiles: &[Vec<Option<Color>>],
    selected: Option<Pos>,
) {
    let (tile_width, tile_height) =
        get_tile_size(size.columns, size.rows, width, height);
    for column in 0..size.columns {
        let x = x1 + (tile_width * column);
        for row in 0..size.rows {
            let y = y1 + (tile_height * row);
            if let Some(color) = tiles[column as usize][row as usize] {
                draw_tile(x, y, tile_width, tile_height, color);
                if let Some(pos) = selected {
                    if pos.x == column && pos.y == row {
                        draw_focus(x, y, tile_width, tile_height);
                    }
                }
            }
        }
    }
}

fn draw_tile(x: i32, y: i32, width: i32, height: i32, color: Color) {
    fltk::draw::draw_box(
        fltk::enums::FrameType::UpBox,
        x,
        y,
        width,
        height,
        color,
    );
    fltk::draw::draw_rect_with_color(x, y, width, height, BACKGROUND_COLOR);
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
