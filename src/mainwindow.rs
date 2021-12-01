// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::action::WindowAction;
use crate::fixed::{
    ABOUT_ICON, APPNAME, HELP_ICON, ICON, NEW_ICON, OPTIONS_ICON, PAD,
    QUIT_ICON, TOOLBUTTON_SIZE,
};
use crate::util;
use fltk::prelude::*;

pub fn make_window(
    sender: fltk::app::Sender<WindowAction>,
) -> (fltk::window::Window, fltk::frame::Frame) {
    let icon = fltk::image::PngImage::from_data(ICON).unwrap();
    let (x, y, width, height) = get_xywh();
    let mut mainwindow = fltk::window::Window::default()
        .with_pos(x, y)
        .with_size(width, height)
        .with_label(APPNAME);
    mainwindow.set_icon(Some(icon));
    const SIZE: i32 = ((TOOLBUTTON_SIZE * 4) / 3) * 6;
    mainwindow.size_range(SIZE, SIZE, SIZE * 3, SIZE * 3);
    mainwindow.make_resizable(true);
    let mut vbox = fltk::group::Flex::default()
        .size_of_parent()
        .with_type(fltk::group::FlexType::Column);
    let top = fltk::frame::Frame::default().with_size(width, PAD / 2);
    vbox.set_size(&top, PAD / 2);
    const TOOLBAR_HEIGHT: i32 = (TOOLBUTTON_SIZE * 3) / 2;
    let toolbar = add_toolbar(sender, width, TOOLBAR_HEIGHT);
    vbox.set_size(&toolbar, TOOLBAR_HEIGHT);
    let mut board = fltk::frame::Frame::default()
        .with_label("Board") // TODO
        .with_size(width, height - (TOOLBAR_HEIGHT * 2));
    let mut statusbar = fltk::frame::Frame::default()
        .with_label("Status bar") // TODO
        .with_size(width, TOOLBAR_HEIGHT);
    vbox.set_size(&statusbar, TOOLBAR_HEIGHT);
    vbox.end();
    mainwindow.end();
    (mainwindow, statusbar)
}

fn add_toolbar(
    sender: fltk::app::Sender<WindowAction>,
    width: i32,
    height: i32,
) -> fltk::group::Flex {
    let mut button_box = fltk::group::Flex::default()
        .with_size(width, height)
        .with_type(fltk::group::FlexType::Row);
    add_toolbutton(
        sender,
        'n',
        "New game • n",
        WindowAction::New,
        NEW_ICON,
        &mut button_box,
    );
    add_toolbutton(
        sender,
        'o',
        "Options… • o",
        WindowAction::Options,
        OPTIONS_ICON,
        &mut button_box,
    );
    fltk::frame::Frame::default().with_size(PAD, PAD);
    add_toolbutton(
        sender,
        'a',
        "About • a",
        WindowAction::About,
        ABOUT_ICON,
        &mut button_box,
    );
    add_toolbutton(
        sender,
        'h',
        "New game • F1 or h",
        WindowAction::Help,
        HELP_ICON,
        &mut button_box,
    );
    fltk::frame::Frame::default().with_size(PAD, PAD);
    add_toolbutton(
        sender,
        'q',
        "New game • Esc or q",
        WindowAction::Quit,
        QUIT_ICON,
        &mut button_box,
    );
    button_box.end();
    button_box
}

fn add_toolbutton(
    sender: fltk::app::Sender<WindowAction>,
    shortcut: char,
    tooltip: &str,
    action: WindowAction,
    icon: &[u8],
    button_box: &mut fltk::group::Flex,
) {
    const TOOLBUTTON_WIDTH: i32 = TOOLBUTTON_SIZE + PAD + 8;
    let mut button = fltk::button::Button::default();
    button.set_size(TOOLBUTTON_WIDTH, TOOLBUTTON_SIZE + PAD);
    button.visible_focus(false);
    button.set_label_size(0);
    button.set_shortcut(fltk::enums::Shortcut::from_char(shortcut));
    button.set_tooltip(tooltip);
    let mut icon = fltk::image::PngImage::from_data(icon).unwrap();
    icon.scale(TOOLBUTTON_SIZE, TOOLBUTTON_SIZE, true, true);
    button.set_image(Some(icon));
    button.emit(sender, action);
    button_box.set_size(&button, TOOLBUTTON_WIDTH);
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

pub fn make_bindings(
    mainwindow: &mut fltk::window::Window,
    sender: fltk::app::Sender<WindowAction>,
) {
    // Both of these are really needed!
    mainwindow.set_callback(move |_| {
        if fltk::app::event() == fltk::enums::Event::Close
            || fltk::app::event_key() == fltk::enums::Key::Escape
        {
            sender.send(WindowAction::Quit);
        }
    });
    mainwindow.handle(move |_, event| match event {
        fltk::enums::Event::KeyUp => {
            match fltk::app::event_key() {
                fltk::enums::Key::Help | fltk::enums::Key::F1 => {
                    sender.send(WindowAction::Help)
                }
                _ => {}
            }
            false
        }
        _ => false,
    });
}
