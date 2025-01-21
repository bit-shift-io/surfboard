use iced::widget::{button, column, row, text, text_input, pick_list, PickList};
use iced::{event, Alignment, Color, Element, Event, Length, Task as Command, Theme};
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use iced_layershell::Application;
use iced_layershell::to_layer_message;
use iced::Renderer;

use super::screen_edge::ScreenEdge;


pub struct Keyboard {
    value: i32,
    text: String,
    screen_edge: ScreenEdge,
}


// Because new iced delete the custom command, so now we make a macro crate to generate
// the Command
#[to_layer_message]
#[derive(Debug, Clone)]
#[doc = "Some docs"]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    TextInput(String),
    IcedEvent(Event),
    ScreenEdgeSelected(ScreenEdge),
}


impl Application for Keyboard {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                value: 0,
                text: "hello, write something here".to_string(),
                screen_edge: ScreenEdge::Bottom,
            },
            Command::none(),
        )
    }

    fn namespace(&self) -> String {
        String::from("surfboard")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        event::listen().map(Message::IcedEvent)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IcedEvent(event) => {
                println!("{event:?}");
                Command::none()
            }
            Message::IncrementPressed => {
                self.value += 1;
                Command::none()
            }
            Message::DecrementPressed => {
                self.value -= 1;
                Command::none()
            }
            Message::TextInput(text) => {
                self.text = text;
                Command::none()
            }

            // change screen edge the app is anchored to
            Message::ScreenEdgeSelected(direction) => {
                self.screen_edge = direction;
                match direction {
                    ScreenEdge::Left => Command::done(Message::AnchorSizeChange(
                        Anchor::Left | Anchor::Top | Anchor::Bottom,
                        (400, 0),
                    )),
                    ScreenEdge::Right => Command::done(Message::AnchorSizeChange(
                        Anchor::Right | Anchor::Top | Anchor::Bottom,
                        (400, 0),
                    )),
                    ScreenEdge::Bottom => Command::done(Message::AnchorSizeChange(
                        Anchor::Bottom | Anchor::Left | Anchor::Right,
                        (0, 400),
                    )),
                    ScreenEdge::Top => Command::done(Message::AnchorSizeChange(
                        Anchor::Top | Anchor::Left | Anchor::Right,
                        (0, 400),
                    )),
                }
            },

            _ => {
                info!("Unknown message");
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let pick_screen_edge: PickList<'_, ScreenEdge, &[ScreenEdge], ScreenEdge, Message, Theme, Renderer> = pick_list(
            &ScreenEdge::ALL[..],
            Some(self.screen_edge),
            Message::ScreenEdgeSelected,
        )
        .placeholder("Edge");

        row![
            button("Tab").on_press(Message::IncrementPressed),
            button("q").on_press(Message::IncrementPressed),
            button("w").on_press(Message::DecrementPressed),
            button("e").on_press(Message::DecrementPressed),
            pick_screen_edge
        ]
        .padding(20)
        .width(Length::Fill)
        .into()
    }

    fn style(&self, theme: &Self::Theme) -> iced_layershell::Appearance {
        use iced_layershell::Appearance;
        Appearance {
            background_color: Color::TRANSPARENT,
            text_color: theme.palette().text,
        }
    }
}
