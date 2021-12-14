// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::board;
use crate::fixed::{about_html, Action, Arrow, HELP_HTML, MESSAGE_DELAY};
use crate::html_form;
use crate::mainwindow;
use crate::options_form;
use fltk::prelude::*;
use thousands::Separable;

pub struct Application {
    app: fltk::app::App,
    mainwindow: fltk::window::Window,
    board: board::Board,
    statusbar: fltk::frame::Frame,
    scorelabel: fltk::frame::Frame,
    helpform: Option<html_form::Form>,
    receiver: fltk::app::Receiver<Action>,
    score: u16,
}

impl Application {
    pub fn new() -> Self {
        let app =
            fltk::app::App::default().with_scheme(fltk::app::Scheme::Gleam);
        let (sender, receiver) = fltk::app::channel::<Action>();
        let (mut mainwindow, board, statusbar, scorelabel) =
            mainwindow::make(sender);
        mainwindow::add_event_handlers(&mut mainwindow, sender);
        mainwindow.show();
        let mut app = Self {
            app,
            mainwindow,
            board,
            statusbar,
            scorelabel,
            helpform: None,
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
        let form = options_form::Form::default();
        if *form.ok.borrow() {
            self.set_status(
                "Start New Game for New Options",
                Some(MESSAGE_DELAY),
            );
        } else {
            self.clear_status();
        }
    }

    fn on_about(&mut self) {
        html_form::Form::new("About", &about_html(), true, 400, 300, false);
    }

    fn on_help(&mut self) {
        if let Some(helpform) = &mut self.helpform {
            helpform.show();
        } else {
            self.helpform = Some(html_form::Form::new(
                "Help", HELP_HTML, false, 380, 420, true,
            ));
        }
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
            score.separate_with_commas(),
            config.board_highscore.separate_with_commas()
        ));
        fltk::app::redraw(); // redraws the world
    }

    fn game_over(&mut self) {
        self.set_status("Click New or press n to play…", None);
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
