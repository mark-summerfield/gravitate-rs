// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

#[derive(Copy, Clone)]
pub enum WindowAction {
    New,
    Options,
    About,
    Help,
    Quit,
}

#[derive(Copy, Clone)]
pub enum BoardAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    ClickTile,
}
