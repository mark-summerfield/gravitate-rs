// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::about_form;
use crate::action::WindowAction;
use crate::mainwindow;
use fltk::prelude::*;

pub struct Application {
    app: fltk::app::App,
    mainwindow: fltk::window::Window,
    statusbar: fltk::frame::Frame,
    receiver: fltk::app::Receiver<WindowAction>,
}

impl Application {
    pub fn new() -> Self {
        let app = fltk::app::App::default()
            .with_scheme(fltk::app::Scheme::Gleam);
        let (sender, receiver) = fltk::app::channel::<WindowAction>();
        let (mut mainwindow, statusbar) = mainwindow::make_window(sender);
        mainwindow::make_bindings(&mut mainwindow, sender);
        mainwindow.show();
        Self { app, mainwindow, statusbar, receiver }
    }

    pub fn run(&mut self) {
        while self.app.wait() {
            if let Some(action) = self.receiver.recv() {
                match action {
                    WindowAction::New => self.on_new(),
                    WindowAction::Options => self.on_options(),
                    WindowAction::About => self.on_about(),
                    WindowAction::Help => self.on_help(),
                    WindowAction::Quit => self.on_quit(),
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
