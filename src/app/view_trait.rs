use iced::{Element, Task as Command};
use super::*;


pub trait ViewTrait {
    fn new() -> Self where Self: Sized;
    fn view(&self) -> Element<MainMessage>;
    //fn update(&mut self, message: &Box<dyn KeyboardLayoutMessage>) -> Command<Box<dyn KeyboardLayoutMessage>>;
    fn name(&self) -> String;
    fn class(&self) -> View;
}