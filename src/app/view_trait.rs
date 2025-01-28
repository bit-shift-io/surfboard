use iced::Element;
use super::*;


pub trait ViewTrait {
    fn new() -> Self where Self: Sized;
    fn view(&self) -> Element<MainMessage>;
    //fn update(&mut self, message: &Box<dyn KeyboardLayoutMessage>) -> Command<Box<dyn KeyboardLayoutMessage>>;
    fn name(&self) -> String;
    fn class(&self) -> View;
    fn has_gesture(&self) -> bool;
}