use iced::{
    widget::{
        column, 
        row,
    }, Length, Task
};
use crate::app::*;
use crate::components::*;


#[derive(Debug, Clone)]
pub struct MainView {
}


impl ViewTrait for MainView {
    fn new() -> Self {
        MainView {
        }
    }

    fn view(&self) -> iced::Element<MainMessage> {
        column![
            row![
                Key::from_str("q").on_press(MainMessage::Debug(String::from("q"))),
                Key::from_str("w").on_press(MainMessage::Debug(String::from("w"))),
                Key::from_str("e").on_press(MainMessage::Debug(String::from("e"))),
                Key::from_str("r").on_press(MainMessage::Debug(String::from("r"))),
                Key::from_str("t").on_press(MainMessage::Debug(String::from("t"))),
                Key::from_str("y").on_press(MainMessage::Debug(String::from("y"))),
                Key::from_str("u").on_press(MainMessage::Debug(String::from("u"))),
                Key::from_str("i").on_press(MainMessage::Debug(String::from("i"))),
                Key::from_str("o").on_press(MainMessage::Debug(String::from("o"))),
                Key::from_str("p").on_press(MainMessage::Debug(String::from("p"))),
            ].padding(0).width(Length::Fill).height(Length::Fill),

            row![
                Key::from_str("a").on_press(MainMessage::Debug(String::from("q"))),
                Key::from_str("s").on_press(MainMessage::Debug(String::from("w"))),
                Key::from_str("d").on_press(MainMessage::Debug(String::from("e"))),
                Key::from_str("f").on_press(MainMessage::Debug(String::from("r"))),
                Key::from_str("g").on_press(MainMessage::Debug(String::from("t"))),
                Key::from_str("h").on_press(MainMessage::Debug(String::from("y"))),
                Key::from_str("j").on_press(MainMessage::Debug(String::from("u"))),
                Key::from_str("k").on_press(MainMessage::Debug(String::from("i"))),
                Key::from_str("l").on_press(MainMessage::Debug(String::from("o"))),
            ].padding(0).width(Length::Fill).height(Length::Fill),

            row![
                Key::from_str("z").on_press(MainMessage::Debug(String::from("q"))),
                Key::from_str("x").on_press(MainMessage::Debug(String::from("w"))),
                Key::from_str("c").on_press(MainMessage::Debug(String::from("e"))),
                Key::from_str("v").on_press(MainMessage::Debug(String::from("r"))),
                Key::from_str("b").on_press(MainMessage::Debug(String::from("t"))),
                Key::from_str("n").on_press(MainMessage::Debug(String::from("y"))),
                Key::from_str("m").on_press(MainMessage::Debug(String::from("u"))),
                Key::from_str("Enter").on_press(MainMessage::ChangeView(View::Configuration)),
            ].padding(0).width(Length::Fill).height(Length::Fill),

            row![
                Key::from_str("@").on_press(MainMessage::ChangeView(View::Configuration)),
                Key::from_str(":)").on_press(MainMessage::Debug(String::from("q"))),
                Key::from_str("     ").on_press(MainMessage::Debug(String::from("w"))),
                Key::from_str(".").on_press(MainMessage::Debug(String::from("e"))),
                Key::from_str(".").on_press(MainMessage::Debug(String::from("r"))),
                Key::from_str(">").on_press(MainMessage::Debug(String::from("t"))),
                Key::from_str("^").on_press(MainMessage::Debug(String::from("y"))),
                Key::from_str("<").on_press(MainMessage::Debug(String::from("u"))),
                Key::from_str("^").on_press(MainMessage::ChangeView(View::Configuration)),
            ].padding(0).width(Length::Fill).height(Length::Fill),

        ].padding(0).width(Length::Fill).height(Length::Fill)
        .into()
    }
    
    fn name(&self) -> String {
        String::from("compact")
    }
    
    fn class(&self) -> View {
        View::Main
    }
    
    fn has_gesture(&self) -> bool {
        true
    }
    
    fn update(&mut self, message: MainMessage) -> Task<MainMessage> {
        Task::none()
    }
}

