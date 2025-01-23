use iced::{Element, Task as Command};
use crate::app::MainMessage;

pub trait ViewTrait {
    fn new() -> Self where Self: Sized;
    fn view(&self) -> Element<MainMessage>;
    //fn update(&mut self, message: &Box<dyn KeyboardLayoutMessage>) -> Command<Box<dyn KeyboardLayoutMessage>>;
    
    //fn convert_message(&self, message: MainMessage) -> Box<dyn KeyboardLayoutMessage>;
    fn name(&self) -> String;
}