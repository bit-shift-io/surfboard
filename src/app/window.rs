use iced::Task;
use iced_layershell::{reexport::Anchor, to_layer_message};

use super::*;



pub struct WindowHandler {
    pub dock: Dock,
    pub windowed: bool,
    pub size: (u32, u32),
    pub margin: (i32, i32, i32, i32), // top, right, bottom, left
}

#[to_layer_message] // used for extra iced messages
#[derive(Debug, Clone)]
pub enum Message {
    Dock(Dock),
}

impl WindowHandler {
    pub fn new() -> Self {
        WindowHandler {
            dock: Dock::Top,
            windowed: true,
            size: (600, 250),
            margin: (0, 0, 0, 0),
        }
    }

    pub fn update(&mut self, message: window::Message) -> Task<main_app::Message> {
        match message {
            Message::Dock(dock) => {
                self.dock = dock;
                match dock {
                    Dock::Left => {
                        return Task::done(main_app::Message::AnchorSizeChange(
                        Anchor::Left | Anchor::Top | Anchor::Bottom,
                        (400, 0),
                        ))
                    }
                    Dock::Right => {
                        return Task::done(main_app::Message::AnchorSizeChange(
                        Anchor::Right | Anchor::Top | Anchor::Bottom,
                        (400, 0),
                        ))
                    }
                    Dock::Bottom => {
                        return Task::done(main_app::Message::AnchorSizeChange(
                        Anchor::Bottom | Anchor::Left | Anchor::Right,
                        (0, 400),
                        ))
                    }
                    Dock::Top => {
                        return Task::done(main_app::Message::AnchorSizeChange(
                        Anchor::Top | Anchor::Left | Anchor::Right,
                        (0, 400),
                        ))
                    }
                }
            }
            _ => {Task::none()}
        }
    }




}