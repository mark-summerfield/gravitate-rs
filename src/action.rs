// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::board_util::Pos;
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
    MoveTile { new_pos: Pos, new_color: Color, pos: Pos },
    Redraw,
    UpdatedScore(u16), // score
    GameOver(u16),     // score
    UserWon(u16),      // score
}
