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
                key_glide("q"),
                key_glide("w"),
                key_glide("e"),
                key_glide("r"),
                key_glide("t"),
                key_glide("y"),
                key_glide("u"),
                key_glide("i"),
                key_glide("o"),
                key_glide("p"),
            ].padding(0).width(Length::Fill).height(Length::Fill),

            row![
                key_glide("a"),
                key_glide("s"),
                key_glide("d"),
                key_glide("f"),
                key_glide("g"),
                key_glide("h"),
                key_glide("j"),
                key_glide("k"),
                key_glide("l"),
            ].padding(0).width(Length::Fill).height(Length::Fill),

            row![
                key_glide("z"),
                key_glide("x"),
                key_glide("c"),
                key_glide("v"),
                key_glide("b"),
                key_glide("n"),
                key_glide("m"),
                key_from_str("Enter"),
            ].padding(0).width(Length::Fill).height(Length::Fill),

            row![
                key_from_str("@"),
                key_from_str(":)"),
                key_from_str("     "),
                key_from_str("."),
                key_from_str("."),
                key_from_str(">"),
                key_from_str("^"),
                key_from_str("<"),
                key_from_str("^").on_press(main_app::Message::ViewHandler(view::Message::ChangeView(View::Settings))),
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

