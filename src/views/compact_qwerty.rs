use iced::{
    widget::{
        column, 
        row,
    }, 
    Length,
};
use crate::app::*;
use crate::comp::*;
use crate::utils::*;


#[derive(Copy, Debug, Clone)]
pub struct CompactQwertyView {}


impl ViewTrait for CompactQwertyView {
    fn new() -> Self {
        CompactQwertyView {}
    }

    fn view(&self, _view_handler: &ViewHandler) -> iced::Element<main_app::Message> {
        column![
            row![
                Key::from_str("q")
                    .on_press(main_app::Message::Debug(String::from("q")))
                    .on_bounds(|bounds| main_app::Message::ComponentHandler(component::Message::Update(String::from("q"), bounds))),
                    
                Key::from_str("w").on_press(main_app::Message::Debug(String::from("w"))),
                Key::from_str("e").on_press(main_app::Message::Debug(String::from("e"))),
                Key::from_str("r").on_press(main_app::Message::Debug(String::from("r"))),
                Key::from_str("t").on_press(main_app::Message::Debug(String::from("t"))),
                Key::from_str("y").on_press(main_app::Message::Debug(String::from("y"))),
                Key::from_str("u").on_press(main_app::Message::Debug(String::from("u"))),
                Key::from_str("i").on_press(main_app::Message::Debug(String::from("i"))),
                Key::from_str("o").on_press(main_app::Message::Debug(String::from("o"))),
                Key::from_str("p").on_press(main_app::Message::Debug(String::from("p"))),
            ].padding(0).width(Length::Fill).height(Length::Fill),

            row![
                Key::from_str("a").on_press(main_app::Message::Debug(String::from("q"))),
                Key::from_str("s").on_press(main_app::Message::Debug(String::from("w"))),
                Key::from_str("d").on_press(main_app::Message::Debug(String::from("e"))),
                Key::from_str("f").on_press(main_app::Message::Debug(String::from("r"))),
                Key::from_str("g").on_press(main_app::Message::Debug(String::from("t"))),
                Key::from_str("h").on_press(main_app::Message::Debug(String::from("y"))),
                Key::from_str("j").on_press(main_app::Message::Debug(String::from("u"))),
                Key::from_str("k").on_press(main_app::Message::Debug(String::from("i"))),
                Key::from_str("l").on_press(main_app::Message::Debug(String::from("o"))),
            ].padding(0).width(Length::Fill).height(Length::Fill),

            row![
                Key::from_str("z").on_press(main_app::Message::Debug(String::from("q"))),
                Key::from_str("x").on_press(main_app::Message::Debug(String::from("w"))),
                Key::from_str("c").on_press(main_app::Message::Debug(String::from("e"))),
                Key::from_str("v").on_press(main_app::Message::Debug(String::from("r"))),
                Key::from_str("b").on_press(main_app::Message::Debug(String::from("t"))),
                Key::from_str("n").on_press(main_app::Message::Debug(String::from("y"))),
                Key::from_str("m").on_press(main_app::Message::Debug(String::from("u"))),
                Key::from_str("Enter").on_press(main_app::Message::ViewHandler(view::Message::ChangeView(View::Settings))),
            ].padding(0).width(Length::Fill).height(Length::Fill),

            row![
                Key::from_str("@").on_press(main_app::Message::ViewHandler(view::Message::ChangeView(View::Settings))),
                Key::from_str(":)").on_press(main_app::Message::Debug(String::from("q"))),
                Key::from_str("     ").on_press(main_app::Message::Debug(String::from("w"))),
                Key::from_str(".").on_press(main_app::Message::Debug(String::from("e"))),
                Key::from_str(".").on_press(main_app::Message::Debug(String::from("r"))),
                Key::from_str(">").on_press(main_app::Message::Debug(String::from("t"))),
                Key::from_str("^").on_press(main_app::Message::Debug(String::from("y"))),
                Key::from_str("<").on_press(main_app::Message::Debug(String::from("u"))),
                Key::from_str("^").on_press(main_app::Message::ViewHandler(view::Message::ChangeView(View::Settings))),
            ].padding(0).width(Length::Fill).height(Length::Fill),

        ].padding(0).width(Length::Fill).height(Length::Fill)
        .into()
    }

    fn has_gesture(&self) -> bool {
        true
    }
    
    fn class(&self) -> View {
        View::CompactQwerty
    }
    
    fn icon(&self) -> &'static [u8] {
        globals::ICON_KEYBOARD
    }
}

