use iced::{Element, Task as Command};
use std::fmt::Debug;
use std::marker::Send;
use std::any::Any;
use crate::keyboard::KeyboardMessage;

pub trait KeyboardLayoutMessage: TypeCheck {
    //fn as_any(&self) -> &dyn Any;
    fn as_keyboard_message(&self) -> KeyboardMessage;
}

pub trait TypeCheck: Any + Debug + Send {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any + Debug + Send> TypeCheck for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub trait KeyboardLayout: Debug {
    fn view(&self) -> Element<'_, Box<dyn KeyboardLayoutMessage>>;
    fn update(&mut self, message: &Box<dyn KeyboardLayoutMessage>) -> Command<Box<dyn KeyboardLayoutMessage>>;
    fn new() -> Self where Self: Sized;
    fn convert_message(&self, message: KeyboardMessage) -> Box<dyn KeyboardLayoutMessage>;
}
