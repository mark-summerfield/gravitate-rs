// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::board_util::Coord;
use fltk::enums::Color;

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
    MoveTile { new_coord: Coord, new_color: Color, coord: Coord },
    Redraw,
    UpdatedScore(u16), // score
    GameOver(u16),     // score
    UserWon(u16),      // score
}
