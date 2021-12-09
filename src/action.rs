// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::board_util::Pos;

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
    MoveTile { new_pos: Pos, pos: Pos },
    Redraw,
    UpdatedScore(u16), // score
    GameOver(u16),     // score
    UserWon(u16),      // score
}
