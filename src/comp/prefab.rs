use iced::{widget::text, Renderer, Theme};

use crate::app::*;
use super::*;


/// Creates a new [`Key`] with the given content.
pub fn key_glide(val: &str) -> Key<'_, main_app::Message, Theme, Renderer> {
    let content = text(val.to_string()).center();
    Key::new(content)
        .on_press(main_app::Message::Debug(val.to_string()))
        .on_bounds(|bounds| main_app::Message::SearchHandler(search::Message::Update(val.to_string(), bounds)))
        .into()
}


/// Creates a new [`Key`] with the given content.
pub fn key_from_str(val: &str) -> Key<'_, main_app::Message, Theme, Renderer> {
    let content= text(val.to_string()).center();
    Key::new(content)
        .on_press(main_app::Message::Debug(val.to_string()))
        .on_bounds(|bounds| main_app::Message::SearchHandler(search::Message::Update(val.to_string(), bounds)))
        .into()
}