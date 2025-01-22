use iced::Task as Command;
use iced_layershell::to_layer_message;
use crate::keyboard::KeyboardMessage;
use super::layout::{KeyboardLayout, KeyboardLayoutMessage, TypeCheck};
use std::any::Any;

#[derive(Debug, Clone)]
pub struct QwertyLayout;

//#[to_layer_message]
#[derive(Debug, Clone)]
pub enum QwertyMessage {
    KeyPress(char),
}

// impl TypeCheck for QwertyMessage {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

impl KeyboardLayoutMessage for QwertyMessage {
    fn as_keyboard_message(&self) -> KeyboardMessage {
        match self {
            QwertyMessage::KeyPress(_) => todo!(),
        }
    }
    
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
}

impl KeyboardLayout for QwertyLayout {
    fn view(&self) -> iced::Element<'_, Box<dyn KeyboardLayoutMessage>> {
        todo!()
    }

    fn update(&mut self, message: &Box<dyn KeyboardLayoutMessage>) -> iced::Task<Box<dyn KeyboardLayoutMessage>> {
        if let Some(qwerty_message) = message.as_any().downcast_ref::<QwertyMessage>() {
            // Handle CompactMessage
        }
        Command::none()
    }

    fn new() -> Self where Self: Sized {
        QwertyLayout {}
    }

    fn convert_message(&self, message: KeyboardMessage) -> Box<dyn KeyboardLayoutMessage> {
        match message {
            //KeyboardMessage::Debug(debug) => Box::new(QwertyMessage::Debug(debug)),
            // Add more conversions here if needed
            _ => unimplemented!(),
        }
    }
}