use crate::app::{MainMessage, ViewTrait};
use crate::{ScreenEdge, Views};
use iced::widget::{button, column, pick_list, row, text, text_input, Button, Column, Container, PickList, Text};
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

        //let menu = Button::new(Text::new("Go to the next page")).on_press(MainMessage::ChangeView(Views::Settings));
        //let col = Column::new().push(navigate);
        //Container::new(col).width(iced::Length::Fill).height(iced::Length::Fill).into()

        column![
            row![
                //KeyButton::new(String::from("qwe")),
                button("q").on_press(MainMessage::Debug(String::from("q"))),
                button("w").on_press(MainMessage::Debug(String::from("w"))),
                button("e").on_press(MainMessage::Debug(String::from("e"))),
                button("r").on_press(MainMessage::Debug(String::from("r"))),
                button("t").on_press(MainMessage::Debug(String::from("t"))),
                button("y").on_press(MainMessage::Debug(String::from("y"))),
                button("u").on_press(MainMessage::Debug(String::from("u"))),
                button("i").on_press(MainMessage::Debug(String::from("i"))),
                button("o").on_press(MainMessage::Debug(String::from("o"))),
                button("p").on_press(MainMessage::Debug(String::from("p"))),
            ],

            row![
                button("a").on_press(MainMessage::Debug(String::from("q"))),
                button("s").on_press(MainMessage::Debug(String::from("w"))),
                button("d").on_press(MainMessage::Debug(String::from("e"))),
                button("f").on_press(MainMessage::Debug(String::from("r"))),
                button("g").on_press(MainMessage::Debug(String::from("t"))),
                button("h").on_press(MainMessage::Debug(String::from("y"))),
                button("j").on_press(MainMessage::Debug(String::from("u"))),
                button("k").on_press(MainMessage::Debug(String::from("i"))),
                button("l").on_press(MainMessage::Debug(String::from("o"))),
            ],

            row![
                button("z").on_press(MainMessage::Debug(String::from("q"))),
                button("x").on_press(MainMessage::Debug(String::from("w"))),
                button("c").on_press(MainMessage::Debug(String::from("e"))),
                button("v").on_press(MainMessage::Debug(String::from("r"))),
                button("b").on_press(MainMessage::Debug(String::from("t"))),
                button("n").on_press(MainMessage::Debug(String::from("y"))),
                button("m").on_press(MainMessage::Debug(String::from("u"))),
                button("Enter").on_press(MainMessage::ChangeView(Views::Settings)),
            ],

            row![
                button("@").on_press(MainMessage::ChangeView(Views::Settings)).width(Length::Fill),
                button(":)").on_press(MainMessage::Debug(String::from("q"))).width(Length::Fill),
                button("     ").on_press(MainMessage::Debug(String::from("w"))).width(Length::Fill),
                button(".").on_press(MainMessage::Debug(String::from("e"))).width(Length::Fill),
                button(".").on_press(MainMessage::Debug(String::from("r"))).width(Length::Fill),
                button(">").on_press(MainMessage::Debug(String::from("t"))).width(Length::Fill),
                button("^").on_press(MainMessage::Debug(String::from("y"))).width(Length::Fill),
                button("<").on_press(MainMessage::Debug(String::from("u"))).width(Length::Fill),
                button("^").on_press(MainMessage::ChangeView(Views::Settings)).width(Length::Fill),
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
}

