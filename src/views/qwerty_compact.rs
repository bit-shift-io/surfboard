use crate::app::{MainMessage, ViewTrait};
use crate::{ScreenEdge, View};
use iced::widget::{column, pick_list, row, text, text_input, Button, Column, Container, PickList, Text};
use iced::Length;
use std::fmt::Debug;

use crate::components::*;

#[derive(Debug, Clone)]
pub struct MainView {}


impl ViewTrait for MainView {
    fn new() -> Self {
        MainView {}
    }

    fn view(&self) -> iced::Element<MainMessage> {

        //let menu = Button::new(Text::new("Go to the next page"));
        //let col = Column::new().push(navigate);
        //Container::new(col).width(iced::Length::Fill).height(iced::Length::Fill).into()

        column![
            row![
                //KeyButton::new(String::from("qwe")),
                //Key::new("test").on_press(MainMessage::Debug(String::from("q"))),
                Key::new("q").on_press(MainMessage::Debug(String::from("q"))),
                Key::new("w").on_press(MainMessage::Debug(String::from("w"))),
                Key::new("e").on_press(MainMessage::Debug(String::from("e"))),
                Key::new("r").on_press(MainMessage::Debug(String::from("r"))),
                Key::new("t").on_press(MainMessage::Debug(String::from("t"))),
                Key::new("y").on_press(MainMessage::Debug(String::from("y"))),
                Key::new("u").on_press(MainMessage::Debug(String::from("u"))),
                Key::new("i").on_press(MainMessage::Debug(String::from("i"))),
                Key::new("o").on_press(MainMessage::Debug(String::from("o"))),
                Key::new("p").on_press(MainMessage::Debug(String::from("p"))),
            ],

            row![
                Key::new("a").on_press(MainMessage::Debug(String::from("q"))),
                Key::new("s").on_press(MainMessage::Debug(String::from("w"))),
                Key::new("d").on_press(MainMessage::Debug(String::from("e"))),
                Key::new("f").on_press(MainMessage::Debug(String::from("r"))),
                Key::new("g").on_press(MainMessage::Debug(String::from("t"))),
                Key::new("h").on_press(MainMessage::Debug(String::from("y"))),
                Key::new("j").on_press(MainMessage::Debug(String::from("u"))),
                Key::new("k").on_press(MainMessage::Debug(String::from("i"))),
                Key::new("l").on_press(MainMessage::Debug(String::from("o"))),
            ],

            row![
                Key::new("z").on_press(MainMessage::Debug(String::from("q"))),
                Key::new("x").on_press(MainMessage::Debug(String::from("w"))),
                Key::new("c").on_press(MainMessage::Debug(String::from("e"))),
                Key::new("v").on_press(MainMessage::Debug(String::from("r"))),
                Key::new("b").on_press(MainMessage::Debug(String::from("t"))),
                Key::new("n").on_press(MainMessage::Debug(String::from("y"))),
                Key::new("m").on_press(MainMessage::Debug(String::from("u"))),
                Key::new("Enter").on_press(MainMessage::ChangeView(View::Settings)),
            ],

            row![
                Key::new("@").on_press(MainMessage::ChangeView(View::Settings)), //.width(Length::Fill),
                Key::new(":)").on_press(MainMessage::Debug(String::from("q"))), //.width(Length::Fill),
                Key::new("     ").on_press(MainMessage::Debug(String::from("w"))), //.width(Length::Fill),
                Key::new(".").on_press(MainMessage::Debug(String::from("e"))), //.width(Length::Fill),
                Key::new(".").on_press(MainMessage::Debug(String::from("r"))), //.width(Length::Fill),
                Key::new(">").on_press(MainMessage::Debug(String::from("t"))), //.width(Length::Fill),
                Key::new("^").on_press(MainMessage::Debug(String::from("y"))), //.width(Length::Fill),
                Key::new("<").on_press(MainMessage::Debug(String::from("u"))), //.width(Length::Fill),
                Key::new("^").on_press(MainMessage::ChangeView(View::Settings)), //.width(Length::Fill),
            ],

        ]
        .padding(0)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
    
    fn name(&self) -> String {
        String::from("compact")
    }
    
    fn class(&self) -> View {
        View::Main
    }
}

