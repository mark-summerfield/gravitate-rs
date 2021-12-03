// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::about_form;
use crate::action::Action;
use crate::board;
use crate::mainwindow;
use fltk::prelude::*;

pub struct Application {
    app: fltk::app::App,
    mainwindow: fltk::window::Window,
    board: board::Board,
    statusbar: fltk::frame::Frame,
    scorelabel: fltk::frame::Frame,
    receiver: fltk::app::Receiver<Action>,
}

impl Application {
    pub fn new() -> Self {
        let app = fltk::app::App::default()
            .with_scheme(fltk::app::Scheme::Gleam);
        let (sender, receiver) = fltk::app::channel::<Action>();
        let (mut mainwindow, board, statusbar, scorelabel) =
            mainwindow::make(sender);
        mainwindow::add_event_handler(&mut mainwindow, sender);
        mainwindow.show();
        Self { app, mainwindow, board, statusbar, scorelabel, receiver }
    }

    pub fn run(&mut self) {
        while self.app.wait() {
            if let Some(action) = self.receiver.recv() {
                match action {
                    Action::New => self.on_new(),
                    Action::Options => self.on_options(),
                    Action::About => self.on_about(),
                    Action::Help => self.on_help(),
                    Action::Quit => self.on_quit(),
                    Action::MoveUp => self.board.move_up(),
                    Action::MoveDown => self.board.move_down(),
                    Action::MoveLeft => self.board.move_left(),
                    Action::MoveRight => self.board.move_right(),
                    Action::ClickTile => self.board.click_tile(),
                    Action::PressTile => self.board.press_tile(),
                }
            }
        }
    }

    fn on_new(&mut self) {
        println!("Application.on_new"); // TODO
    }

    fn on_options(&mut self) {
        println!("Application.on_options"); // TODO
    }

    fn on_about(&mut self) {
        about_form::Form::default();
    }

    fn on_help(&mut self) {
        println!("Application.on_help"); // TODO
    }

    fn on_quit(&mut self) {
        let config = CONFIG.get().read().unwrap();
        config.save(
            self.mainwindow.x(),
            self.mainwindow.y(),
            self.mainwindow.width(),
            self.mainwindow.height(),
        );
        self.app.quit();
    }
}
