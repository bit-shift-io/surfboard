use iced::{widget::text, Element, Renderer, Theme};

use crate::app::*;
use super::*;



// /// Creates a new [`Key`] with the given content.
// pub fn key_glide(s: &str) -> Key<'_, Message, Theme, Renderer>
// // where
// //     Renderer: iced_core::Renderer + iced_core::text::Renderer,
// //     Message: Clone,
// {
//     //let content= text(s.to_string()).center();
//     //Key::new(content)
//     Key::from_str("w")
//         .on_press(main_app::Message::Debug(String::from("w")))
//         .on_bounds(|bounds| main_app::Message::ComponentHandler(component::Message::Update(String::from("w"), bounds)))
// }


// /// Creates a new [`Key`] with the given content.
// pub fn key_from_str<'a, Message, Theme, Renderer>(s: &str) -> Key<'a, Message, Theme, Renderer> 
// where
//     Renderer: 'a + iced_core::Renderer + iced_core::text::Renderer,
//     Message: Clone,
// {
//     let content= text(s.to_string()).center();
//     Key::new(content)
// }