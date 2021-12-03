// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

#[derive(Copy, Clone)]
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
    NewGame,
    UpdatedScore(u16), // score
    GameOver(u16), // score
    UserWon(u16), // score
}
