// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::about_form;
use crate::action::Action;
use crate::board;
use crate::fixed::{Arrow, MESSAGE_DELAY};
use crate::mainwindow;
use fltk::prelude::*;
use thousands::Separable;

pub struct Application {
    app: fltk::app::App,
    mainwindow: fltk::window::Window,
    board: board::Board,
    statusbar: fltk::frame::Frame,
    scorelabel: fltk::frame::Frame,
    receiver: fltk::app::Receiver<Action>,
    score: u16,
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
        let mut app = Self {
            app,
            mainwindow,
            board,
            statusbar,
            scorelabel,
            receiver,
            score: 0,
        };
        app.on_new_game();
        app
    }

    pub fn run(&mut self) {
        while self.app.wait() {
            if let Some(action) = self.receiver.recv() {
                match action {
                    Action::New => self.on_new_game(),
                    Action::Options => self.on_options(),
                    Action::About => self.on_about(),
                    Action::Help => self.on_help(),
                    Action::Quit => self.on_quit(),
                    Action::MoveUp => {
                        self.clear_status();
                        self.board.on_arrow(Arrow::Up);
                    }
                    Action::MoveDown => {
                        self.clear_status();
                        self.board.on_arrow(Arrow::Down);
                    }
                    Action::MoveLeft => {
                        self.clear_status();
                        self.board.on_arrow(Arrow::Left);
                    }
                    Action::MoveRight => {
                        self.clear_status();
                        self.board.on_arrow(Arrow::Right)
                    }
                    Action::ClickTile => {
                        self.clear_status();
                        self.board.on_click_tile();
                    }
                    Action::PressTile => {
                        self.clear_status();
                        self.board.on_press_tile();
                    }
                    Action::DeleteAdjoining => {
                        self.board.delete_adjoining()
                    }
                    Action::CloseUp => self.board.close_up(),
                    Action::UpdatedScore(score) => {
                        self.updated_score(score)
                    }
                    Action::Redraw => self.board.redraw(),
                    Action::GameOver => self.game_over(),
                    Action::UserWon => self.user_won(),
                }
            }
        }
    }

    pub fn on_new_game(&mut self) {
        self.score = 0;
        self.board.new_game();
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
        self.score = score;
        let config = CONFIG.get().read().unwrap();
        self.scorelabel.set_label(&format!(
            "{} • {}",
            score,
            config.board_highscore.separate_with_commas()
        ));
        fltk::app::redraw(); // redraws the world
    }

    fn game_over(&mut self) {
        self.set_status("Game Over", Some(MESSAGE_DELAY));
    }

    fn user_won(&mut self) {
        // TODO
        // if new high score, save & change message to
        //  "You Won with a New Highscore!"
        self.set_status("You Won!", Some(MESSAGE_DELAY));
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
        self.statusbar.set_label("");
        fltk::app::redraw(); // redraws the world
    }
}
