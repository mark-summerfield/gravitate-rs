// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::about_form;
use crate::action::WindowAction;
use crate::fixed::{
    ABOUT_ICON, APPNAME, HELP_ICON, ICON, NEW_ICON, OPTIONS_ICON, PAD,
    QUIT_ICON,
};
use crate::util;
use fltk::prelude::*;

pub struct Application {
    app: fltk::app::App,
    main_window: fltk::window::Window,
    receiver: fltk::app::Receiver<WindowAction>,
}

impl Application {
    pub fn new() -> Self {
        let app = fltk::app::App::default()
            .with_scheme(fltk::app::Scheme::Gleam);
        let (sender, receiver) = fltk::app::channel::<WindowAction>();
        let mut main_window = make_window(sender);
        make_bindings(&mut main_window, sender);
        main_window.show();
        Self { app, main_window, receiver }
    }

    pub fn run(&mut self) {
        while self.app.wait() {
            if let Some(action) = self.receiver.recv() {
                match action {
                    WindowAction::New => println!("New TODO"), // TODO
                    WindowAction::Options => println!("Options TODO"), // TODO
                    WindowAction::About => self.on_about(),
                    WindowAction::Help => println!("Help TODO"), // TODO
                    WindowAction::Quit => self.on_quit(),
                }
            }
        }
    }

    fn on_about(&mut self) {
        about_form::Form::default();
    }

    fn on_quit(&mut self) {
        let config = CONFIG.get().read().unwrap();
        config.save(
            self.main_window.x(),
            self.main_window.y(),
            self.main_window.width(),
            self.main_window.height(),
        );
        self.app.quit();
    }
}

fn make_window(
    sender: fltk::app::Sender<WindowAction>,
) -> fltk::window::Window {
    let icon = fltk::image::PngImage::from_data(ICON).unwrap();
    let (x, y, width, height) = get_xywh();
    let mut main_window = fltk::window::Window::default()
        .with_pos(x, y)
        .with_size(width, height)
        .with_label(APPNAME);
    main_window.set_icon(Some(icon));
    main_window.make_resizable(true);
    let mut vbox = fltk::group::Pack::default()
        .size_of_parent()
        .with_type(fltk::group::PackType::Vertical);
    fltk::frame::Frame::default().with_size(width, PAD);
    add_toolbar(sender, width);
    // TODO add board (just a frame for now)
    // TODO add status bar: info  score highscore
    vbox.end();
    main_window.end();
    main_window
}

fn add_toolbar(sender: fltk::app::Sender<WindowAction>, width: i32) {
    let mut button_box = fltk::group::Pack::default()
        .with_size(width, (TOOLBUTTON_SIZE * 3) / 2)
        .with_align(fltk::enums::Align::Left)
        .with_type(fltk::group::PackType::Horizontal);
    button_box.set_spacing(6);
    add_toolbutton(
        sender,
        'n',
        "New game • n",
        WindowAction::New,
        NEW_ICON,
    );
    add_toolbutton(
        sender,
        'o',
        "Options… • o",
        WindowAction::Options,
        OPTIONS_ICON,
    );
    fltk::frame::Frame::default().with_size(TOOLBUTTON_SIZE, PAD);
    add_toolbutton(
        sender,
        'a',
        "About • a",
        WindowAction::About,
        ABOUT_ICON,
    );
    add_toolbutton(
        sender,
        'h',
        "New game • F1 or h",
        WindowAction::Help,
        HELP_ICON,
    );
    fltk::frame::Frame::default().with_size(TOOLBUTTON_SIZE, PAD);
    add_toolbutton(
        sender,
        'q',
        "New game • Esc or q",
        WindowAction::Quit,
        QUIT_ICON,
    );
    button_box.end();
}

fn add_toolbutton(
    sender: fltk::app::Sender<WindowAction>,
    shortcut: char,
    tooltip: &str,
    action: WindowAction,
    icon: &[u8],
) -> fltk::button::Button {
    let mut button = fltk::button::Button::default();
    button.set_size(TOOLBUTTON_SIZE + PAD, TOOLBUTTON_SIZE + PAD);
    button.visible_focus(false);
    button.set_label_size(0);
    button.set_shortcut(fltk::enums::Shortcut::from_char(shortcut));
    button.set_tooltip(tooltip);
    let mut icon = fltk::image::PngImage::from_data(icon).unwrap();
    icon.scale(TOOLBUTTON_SIZE, TOOLBUTTON_SIZE, true, true);
    button.set_image(Some(icon));
    button.emit(sender, action);
    button
}

fn get_xywh() -> (i32, i32, i32, i32) {
    let mut config = CONFIG.get().write().unwrap();
    let x = if config.window_x >= 0 {
        config.window_x
    } else {
        util::x() - (config.window_width / 2)
    };
    let y = if config.window_y >= 0 {
        config.window_y
    } else {
        util::y() - (config.window_height / 2)
    };
    if x != config.window_x {
        config.window_x = x;
    }
    if y != config.window_y {
        config.window_y = y;
    }
    (x, y, config.window_width, config.window_height)
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
    main_window.handle(move |_, event| match event {
        fltk::enums::Event::KeyUp => {
            match fltk::app::event_key() {
                // TODO | F1
                fltk::enums::Key::Help => sender.send(WindowAction::Help),
                _ => {}
            }
            false
        }
        _ => false,
    });
}

const TOOLBUTTON_SIZE: i32 = 32;
