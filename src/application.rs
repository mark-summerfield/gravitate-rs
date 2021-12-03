// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::about_form;
use crate::action::Action;
use crate::board;
use crate::fixed::MESSAGE_DELAY;
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
                    Action::New => self.board.new_game(),
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
                    Action::NewGame => self.on_new_game(),
                    Action::UpdatedScore(score) => {
                        self.updated_score(score)
                    }
                    Action::GameOver(score) => self.game_over(score),
                    Action::UserWon(score) => self.user_won(score),
                }
            }
        }
    }

    fn on_new_game(&mut self) {
        self.set_status("New game! Click a tile…", Some(MESSAGE_DELAY));
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

    fn updated_score(&mut self, score: u16) {
        println!("Application.updated_score"); // TODO update score
    }

    fn game_over(&mut self, score: u16) {
        println!("Application.game_over"); // TODO update score
    }

    fn user_won(&mut self, score: u16) {
        println!("Application.user_won"); // TODO update score & maybe highscore
    }

    fn set_status(&mut self, message: &str, timeout: Option<f64>) {
        self.statusbar.set_label(message);
        fltk::app::redraw(); // redraws the world
        if let Some(timeout) = timeout {
            fltk::app::add_timeout(timeout, {
                let mut statusbar = self.statusbar.clone();
                move || {
                    statusbar.set_label("");
                    fltk::app::redraw(); // redraws the world
                }
            });
        }
    }

    fn clear_status(&mut self) {
        println!("clear_status");
        self.statusbar.set_label("");
        fltk::app::redraw(); // redraws the world
    }
}
