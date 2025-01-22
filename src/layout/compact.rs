use crate::keyboard::KeyboardMessage;
use super::layout::{KeyboardLayout, KeyboardLayoutMessage, TypeCheck};
use iced_layershell::to_layer_message;
use iced::Task as Command;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct CompactLayout {}

//#[to_layer_message]
#[derive(Debug, Clone)]
pub enum CompactMessage {
    KeyPressed(String),
    Debug(String),
}

// impl TypeCheck for CompactMessage {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

impl KeyboardLayoutMessage for CompactMessage {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }

    fn as_keyboard_message(&self) -> KeyboardMessage {
        match self {
            CompactMessage::KeyPressed(key) => KeyboardMessage::Debug(format!("Compact key pressed: {}", key)),
            CompactMessage::Debug(debug) => KeyboardMessage::Debug(debug.clone()),
        }
    }
}

impl KeyboardLayout for CompactLayout {
    fn view(&self) -> iced::Element<'_, Box<dyn KeyboardLayoutMessage>> {
        todo!()
    }

    fn update(&mut self, message: &Box<dyn KeyboardLayoutMessage>) -> iced::Task<Box<dyn KeyboardLayoutMessage>> {
        if let Some(compact_message) = message.as_any().downcast_ref::<CompactMessage>() {
            // Handle CompactMessage
        }
        Command::none()
    }

    fn new() -> Self where Self: Sized {
        CompactLayout {}
    }

    fn convert_message(&self, message: KeyboardMessage) -> Box<dyn KeyboardLayoutMessage> {
        match message {
            KeyboardMessage::Debug(debug) => Box::new(CompactMessage::Debug(debug)),
            // Add more conversions here if needed
            _ => unimplemented!(),
        }
    }
}