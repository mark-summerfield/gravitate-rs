// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::about_form;
use crate::action::WindowAction;
use crate::fixed::{APPNAME, ICON};
use fltk::prelude::*;

pub struct Application {
    app: fltk::app::App,
    receiver: fltk::app::Receiver<WindowAction>,
}

impl Application {
    pub fn new() -> Self {
        // TODO arg should be config: Config
        let app = fltk::app::App::default()
            .with_scheme(fltk::app::Scheme::Gleam);
        let (sender, receiver) = fltk::app::channel::<WindowAction>();
        let mut main_window = make_window();
        make_bindings(&mut main_window, sender);
        main_window.show();
        Self { app, receiver }
    }

    pub fn run(&mut self) {
        while self.app.wait() {
            if let Some(action) = self.receiver.recv() {
                match action {
                    WindowAction::New => {}     // TODO
                    WindowAction::Options => {} // TODO
                    WindowAction::About => self.on_about(),
                    WindowAction::Help => {} // TODO
                    WindowAction::Quit => self.on_quit(),
                }
            }
        }
    }

    fn on_about(&mut self) {
        about_form::Form::default();
    }

    fn on_quit(&mut self) {
        println!("on_quit: save config"); // TODO
        self.app.quit();
    }
}

fn make_window() -> fltk::window::Window {
    let image = fltk::image::PngImage::from_data(ICON).unwrap();
    let mut main_window = fltk::window::Window::default()
        .with_size(260, 300)
        .center_screen()
        .with_label(APPNAME);
    main_window.set_icon(Some(image));
    main_window.make_resizable(true);
    // TODO add toolbuttons
    // TODO add board
    main_window.end();
    main_window
}

fn make_bindings(
    main_window: &mut fltk::window::Window,
    sender: fltk::app::Sender<WindowAction>,
) {
    // Both of these are really needed!
    main_window.set_callback(move |_| {
        if fltk::app::event() == fltk::enums::Event::Close
            || fltk::app::event_key() == fltk::enums::Key::Escape
        {
            sender.send(WindowAction::Quit);
        }
    });
    main_window.handle(move |_, event| {
        const A_KEY: fltk::enums::Key = fltk::enums::Key::from_char('a');
        const Q_KEY: fltk::enums::Key = fltk::enums::Key::from_char('q');
        match event {
            fltk::enums::Event::KeyUp => {
                match fltk::app::event_key() {
                    A_KEY => sender.send(WindowAction::About),
                    // TODO F1 | H_KEY help, N_KEY new game, O_KEY options
                    Q_KEY => sender.send(WindowAction::Quit),
                    _ => {}
                }
                false
            }
            _ => false,
        }
    });
}
