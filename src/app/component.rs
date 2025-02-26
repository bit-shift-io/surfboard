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
    Reset,
}

#[derive(Debug, Clone)]
pub struct ComponentData {
    pub text: String,
    pub bounds: Rectangle,
}

/// Handles the state of widget/components.  
/// This is used for the glide typing.
#[derive(Clone, Debug)]
pub struct ComponentHandler {
    components: Vec<ComponentData>,
}

impl ComponentHandler {
    pub fn new() -> Self {
        ComponentHandler {
            components: Vec::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<main_app::Message> {
        match message {
            Message::Update(text, rectangle) => {
                self.components.push(ComponentData { text, bounds: rectangle });
                Task::none()
            }
            Message::Reset => {
                self.components.clear();
                Task::none()
            }
        }
    }

    pub fn start(&mut self) -> Task<main_app::Message> {
        self.components.clear();
        Task::none()
    }

    pub fn get_components(&self) -> &Vec<ComponentData> {
        &self.components
    }
}