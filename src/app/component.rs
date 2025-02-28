use iced::{
    Point, 
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
        info!("ComponentHandler started");
        Task::none()
    }

    pub fn end(&mut self) -> Task<main_app::Message> {
        info!("ComponentHandler end");
        Task::none()
    }

    pub fn update_move(&mut self, position: Point) -> Task<main_app::Message> {

        // loop through components, and find the one that is closest to the position
        let mut closest_component = None;
        let mut closest_distance = f32::MAX;
        for component in &self.components {
            let distance = component.bounds.center().distance(position);
            if distance < closest_distance {
                closest_distance = distance;
                closest_component = Some(component);
            }
        }

        if let Some(component) = closest_component {
            info!("{:?}", component.text);
        }
        Task::none()
    }
}