// Copyright © 2021-23 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::board;
use crate::fixed::{
    Action, ABOUT_ICON, APPNAME, HELP_ICON, ICON, NEW_ICON, OPTIONS_ICON,
    PAD, QUIT_ICON, TOOLBAR_HEIGHT, TOOLBUTTON_SIZE,
};
use crate::util;
use fltk::prelude::*;
use thousands::Separable;

pub fn make(
    sender: fltk::app::Sender<Action>,
) -> (
    fltk::window::Window,
    board::Board,
    fltk::frame::Frame,
    fltk::frame::Frame,
) {
    fltk::window::Window::set_default_xclass(APPNAME);
    let icon = fltk::image::SvgImage::from_data(ICON).unwrap();
    let (x, y, width, height) = get_config_window_rect();
    let mut mainwindow =
        fltk::window::Window::new(x, y, width, height, APPNAME);
    mainwindow.set_icon(Some(icon));
    let size = ((TOOLBUTTON_SIZE * 4) / 3) * 6;
    mainwindow.size_range(size, size, size * 4, size * 4);
    mainwindow.make_resizable(true);
    let mut vbox = fltk::group::Flex::default().column().size_of_parent();
    vbox.set_margin(PAD);
    let toolbar = add_toolbar(sender, width);
    vbox.set_size(&toolbar, TOOLBAR_HEIGHT);
    let mut board = board::Board::new(sender);
    board.set_size(width, height - (TOOLBAR_HEIGHT * 2));
    let (statusbar, scorelabel) = add_status_row(&mut vbox, width);
    vbox.end();
    mainwindow.end();
    (mainwindow, board, statusbar, scorelabel)
}

fn add_toolbar(
    sender: fltk::app::Sender<Action>,
    width: i32,
) -> fltk::group::Flex {
    let mut button_box =
        fltk::group::Flex::default().row().with_size(width, TOOLBAR_HEIGHT);
    button_box.set_frame(fltk::enums::FrameType::UpBox);
    button_box.set_margin(PAD);
    add_toolbutton(
        sender,
        'n',
        "New game • n",
        Action::New,
        NEW_ICON,
        &mut button_box,
    );
    add_toolbutton(
        sender,
        'o',
        "Options… • o",
        Action::Options,
        OPTIONS_ICON,
        &mut button_box,
    );
    fltk::frame::Frame::default().with_size(PAD, PAD);
    add_toolbutton(
        sender,
        'a',
        "About • a",
        Action::About,
        ABOUT_ICON,
        &mut button_box,
    );
    add_toolbutton(
        sender,
        'h',
        "Help • F1 or h",
        Action::Help,
        HELP_ICON,
        &mut button_box,
    );
    fltk::frame::Frame::default().with_size(PAD, PAD);
    add_toolbutton(
        sender,
        'q',
        "Quit • Esc or q",
        Action::Quit,
        QUIT_ICON,
        &mut button_box,
    );
    button_box.end();
    button_box
}

fn add_toolbutton(
    sender: fltk::app::Sender<Action>,
    shortcut: char,
    tooltip: &str,
    action: Action,
    icon: &str,
    button_box: &mut fltk::group::Flex,
) {
    let width = TOOLBUTTON_SIZE + PAD + 8;
    let mut button = fltk::button::Button::default();
    button.set_size(width, TOOLBUTTON_SIZE + PAD);
    button.visible_focus(false);
    button.set_label_size(0);
    button.set_shortcut(fltk::enums::Shortcut::from_char(shortcut));
    button.set_tooltip(tooltip);
    let mut icon = fltk::image::SvgImage::from_data(icon).unwrap();
    icon.scale(TOOLBUTTON_SIZE, TOOLBUTTON_SIZE, true, true);
    button.set_image(Some(icon));
    button.emit(sender, action);
    button_box.set_size(&button, width);
}

fn add_status_row(
    vbox: &mut fltk::group::Flex,
    width: i32,
) -> (fltk::frame::Frame, fltk::frame::Frame) {
    let mut status_row = fltk::group::Flex::default()
        .row()
        .with_size(width, TOOLBUTTON_SIZE);
    let mut statusbar = fltk::frame::Frame::default();
    statusbar.set_frame(fltk::enums::FrameType::EngravedFrame);
    let config = CONFIG.get().read().unwrap();
    let mut scorelabel = fltk::frame::Frame::default().with_label(
        &format!("0 • {}", config.board_highscore.separate_with_commas()),
    );
    scorelabel.set_frame(fltk::enums::FrameType::EngravedFrame);
    status_row.set_size(&scorelabel, 120);
    status_row.end();
    vbox.set_size(&status_row, TOOLBUTTON_SIZE);
    (statusbar, scorelabel)
}

fn get_config_window_rect() -> (i32, i32, i32, i32) {
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

pub fn add_event_handlers(
    mainwindow: &mut fltk::window::Window,
    sender: fltk::app::Sender<Action>,
) {
    // Both of these are really needed!
    mainwindow.set_callback(move |_| {
        if fltk::app::event() == fltk::enums::Event::Close
            || fltk::app::event_key() == fltk::enums::Key::Escape
        {
            sender.send(Action::Quit);
        }
    });
    mainwindow.handle(move |_, event| {
        if event == fltk::enums::Event::KeyUp
            && fltk::app::event_key().bits() == 32
        {
            sender.send(Action::PressTile); // Space
            return true;
        }
        match event {
            fltk::enums::Event::KeyUp => match fltk::app::event_key() {
                fltk::enums::Key::Help | fltk::enums::Key::F1 => {
                    sender.send(Action::Help);
                    true
                }
                fltk::enums::Key::Up => {
                    sender.send(Action::MoveUp);
                    true
                }
                fltk::enums::Key::Down => {
                    sender.send(Action::MoveDown);
                    true
                }
                fltk::enums::Key::Left => {
                    sender.send(Action::MoveLeft);
                    true
                }
                fltk::enums::Key::Right => {
                    sender.send(Action::MoveRight);
                    true
                }
                _ => false,
            },
            _ => false,
        }
    });
}
