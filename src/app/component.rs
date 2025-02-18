use iced::{
    Rectangle, 
    Task
};

use crate::{
    app::*,
};

#[derive(Debug, Clone)]
pub enum Message {
    Update(String, Rectangle),
}

/// Handles the state of widget/components.  
/// This is used for the glide typing.
#[derive(Clone, Debug)]
pub struct ComponentHandler {

}

impl ComponentHandler {
    pub fn new() -> Self {
        ComponentHandler {

        }
    }

    pub fn update(&mut self, message: Message) -> Task<main_app::Message> {
        match message {
            Message::Update(text, rectangle) => {
                Task::none()
            }
        }
    }
}