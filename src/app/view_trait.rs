use iced::{
    Element,
    Task
};
use super::*;


pub trait ViewTrait {
    fn new() -> Self where Self: Sized;
    fn view(&self) -> Element<MainMessage>;
    fn update(&mut self, message: MainMessage) -> Task<MainMessage>;
    fn name(&self) -> String;
    fn class(&self) -> View;
    fn has_gesture(&self) -> bool;
}