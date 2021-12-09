// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::about_form;
use crate::action::Action;
use crate::board;
use crate::fixed::{Arrow, MESSAGE_DELAY};
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
        let mut app = Self {
            app,
            mainwindow,
            board,
            statusbar,
            scorelabel,
            receiver,
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
                    Action::MoveUp => self.board.on_arrow(Arrow::Up),
                    Action::MoveDown => self.board.on_arrow(Arrow::Down),
                    Action::MoveLeft => self.board.on_arrow(Arrow::Left),
                    Action::MoveRight => {
                        self.board.on_arrow(Arrow::Right)
                    }
                    Action::ClickTile => self.board.on_click_tile(),
                    Action::PressTile => self.board.on_press_tile(),
                    Action::DeleteAdjoining => {
                        self.board.delete_adjoining()
                    }
                    Action::CloseUp => self.board.close_up(),
                    Action::MoveTile { new_pos, pos } => {
                        self.board.move_tile(new_pos, pos)
                    }
                    Action::Redraw => self.board.redraw(),
                    Action::UpdatedScore(score) => {
                        self.updated_score(score)
                    }
                    Action::GameOver(score) => self.game_over(score),
                    Action::UserWon(score) => self.user_won(score),
                }
            }
        }
    }

    pub fn on_new_game(&mut self) {
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
        println!("Application.updated_score {}", score); // TODO update score
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
