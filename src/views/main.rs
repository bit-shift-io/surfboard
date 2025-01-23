use crate::app::{MainMessage, ViewTrait};
use crate::{ScreenEdge, Views};
use iced::widget::{button, column, pick_list, row, text, text_input, Button, Column, Container, PickList, Text};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct MainView {}


impl ViewTrait for MainView {
    fn new() -> Self {
        MainView {}
    }

    fn view(&self) -> iced::Element<MainMessage> {

        let navigate = Button::new(Text::new("Go to the next page")).on_press(MainMessage::ChangeView(Views::Settings));
        let col = Column::new().push(navigate);
        Container::new(col).width(iced::Length::Fill).height(iced::Length::Fill).into()

        // Container::new(Text::new("Hello from Page 2"))
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        //     .into()

        // row![
        //     button("Tab").on_press(MainWindowMessage::Debug("tab".into())),
        //     button("q").on_press(MainWindowMessage::Debug("q".into())),
        //     button("w").on_press(MainWindowMessage::Debug("w".into())),
        //     button("e").on_press(MainWindowMessage::Debug("e".into())),
        //     pick_screen_edge,
        //     //menu,
        // ]
        // .padding(20)
        // .width(Length::Fill)
        // .into()
    }
    
    fn name(&self) -> String {
        String::from("compact")
    }
}

