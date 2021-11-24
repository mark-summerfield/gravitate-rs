// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use fltk::app;

pub fn center() -> (i32, i32) {
    (
        (app::screen_size().0 / 2.0) as i32,
        (app::screen_size().1 / 2.0) as i32,
    )
}
