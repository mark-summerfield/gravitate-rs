// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::about_form;
use crate::action::Action;
use crate::fixed::{APPNAME, ICON};
use fltk::{
    app,
    enums::{Event, Key},
    image,
    prelude::*,
    window,
};

pub struct Application {
    app: app::App,
    receiver: app::Receiver<Action>,
}

impl Application {
    pub fn new() -> Self {
        // TODO arg should be config: Config
        let app = app::App::default().with_scheme(app::Scheme::Gleam);
        let (sender, receiver) = app::channel::<Action>();
        let mut main_window = make_window();
        make_bindings(&mut main_window, sender);
        main_window.show();
        Self { app, receiver }
    }

    pub fn run(&mut self) {
        while self.app.wait() {
            if let Some(action) = self.receiver.recv() {
                match action {
                    Action::New => {}     // TODO
                    Action::Options => {} // TODO
                    Action::About => self.on_about(),
                    Action::Help => {} // TODO
                    Action::Quit => self.on_quit(),
                    _ => {} // TODO MoveUp etc., & ClickTile
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

fn make_window() -> window::Window {
    let image = image::PngImage::from_data(ICON).unwrap();
    let mut main_window = window::Window::default()
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
    main_window: &mut window::Window,
    sender: app::Sender<Action>,
) {
    main_window.set_callback(move |_| {
        if app::event() == Event::Close || app::event_key() == Key::Escape
        {
            sender.send(Action::Quit);
        }
    });
    main_window.handle(move |_, event| match event {
        Event::KeyUp => {
            if app::event_key() == Key::from_char('a') {
                sender.send(Action::About);
            }
            false
        }
        _ => false,
    });
}
