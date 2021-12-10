// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

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
