use compact::{CompactLayout, CompactMessage};
use iced::widget::{button, column, row, text, text_input, pick_list, PickList};
use iced::{Border, theme, alignment, event, Color, Element, Event, Length, Task as Command, Theme};
use iced_aw::menu::{self, Item, Menu};
use iced_layershell::reexport::Anchor;
use iced_layershell::Application;
use iced_layershell::to_layer_message;
use iced::Renderer;
use iced::border::Radius;
use iced_aw::{menu_bar, menu_items};
use iced_aw::style::{menu_bar::primary, Status};
use qwerty::{QwertyLayout, QwertyMessage};
use std::fmt::Debug;
use std::marker::Send;

use crate::layout;
use crate::layout::layout::{KeyboardLayout, KeyboardLayoutMessage};
use crate::layout::*;
use super::screen_edge::ScreenEdge;


pub struct Keyboard {
    screen_edge: ScreenEdge,
    theme: iced::Theme,
    dark_mode: bool,
    current_layout: LayoutType,
    layouts: Vec<Box<dyn KeyboardLayout>>,
}

#[derive(Clone, Debug)]
pub enum LayoutType {
    Compact,
    Qwerty,
    // Add more layouts here (e.g., Qwerty, Dvorak)
}


// Because new iced delete the custom command, so now we make a macro crate to generate
// the Command
#[to_layer_message]
#[derive(Debug, Clone)]
pub enum KeyboardMessage {
    Debug(String),
    IcedEvent(Event),
    ScreenEdgeSelected(ScreenEdge),
    SwitchLayout(LayoutType),
}


impl Keyboard {
    fn get_current_layout_mut(&mut self) -> &mut Box<dyn KeyboardLayout> {
        match self.current_layout {
            LayoutType::Compact => &mut self.layouts[0],
            LayoutType::Qwerty => &mut self.layouts[1],
            // Add more matches for other layouts
        }
    }

    fn get_current_layout(&self) -> &Box<dyn KeyboardLayout> {
        match self.current_layout {
            LayoutType::Compact => &self.layouts[0],
            LayoutType::Qwerty => &self.layouts[1],
            // Add more matches for other layouts
        }
    }
}


impl Application for Keyboard {
    type Message = KeyboardMessage;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let compact_layout: Box<dyn KeyboardLayout> = Box::new(CompactLayout::new());
        let qwerty_layout: Box<dyn KeyboardLayout> = Box::new(QwertyLayout::new());
        let layouts: Vec<Box<dyn KeyboardLayout>> = vec![compact_layout, qwerty_layout]; 
        (
            Self {
                screen_edge: ScreenEdge::Top,
                theme: iced::Theme::Light,
                dark_mode: true,
                current_layout: LayoutType::Compact,
                layouts,
            },
            Command::none(),
        )
    }
    

    fn view(&self) -> Element<Self::Message> {
        let current_layout = self.get_current_layout();
        current_layout.view().map(|msg| msg.as_keyboard_message())
    }


    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            KeyboardMessage::SwitchLayout(layout_type) => {
                self.current_layout = layout_type;
            }
            KeyboardMessage::Debug(_) => {
            }
            KeyboardMessage::IcedEvent(event) => {
            }
            KeyboardMessage::ScreenEdgeSelected(screen_edge) => {
            }
            _ => {
                // Handle layout-specific messages
                let current_layout = self.get_current_layout_mut();
                let boxed_message = current_layout.convert_message(message);
                current_layout.update(&boxed_message);
            }
        }
        Command::none()
    }

    fn style(&self, theme: &Self::Theme) -> iced_layershell::Appearance {
        use iced_layershell::Appearance;
        Appearance {
            background_color: Color::TRANSPARENT,
            text_color: theme.palette().text,
        }
    }

    fn namespace(&self) -> String {
        String::from("surfboard")
    }
}



    /*
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                screen_edge: ScreenEdge::Bottom,
                theme: iced::Theme::Light,
                dark_mode: true,
                layout: Box::new(compact::CompactLayout::new()),
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
            Message::Debug(s) => {
                info!("Debug: {}", s);
                Command::none()
            }
            Message::IcedEvent(event) => {
                println!("{event:?}");
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

        // menu
        let menu_tpl_1 = |items| Menu::new(items).max_width(180.0).offset(15.0).spacing(5.0);
        let menu_tpl_2 = |items| Menu::new(items).max_width(180.0).offset(0.0).spacing(5.0);
        let menu = menu_bar!(
            (debug_button_s("="), {
                let menu_screen_edge = menu_tpl_2(menu_items!(
                    (button("Top").width(Length::Fill).on_press(Message::ScreenEdgeSelected(ScreenEdge::Top)))
                    (button("Bottom").width(Length::Fill).on_press(Message::ScreenEdgeSelected(ScreenEdge::Bottom)))
                    (button("Left").width(Length::Fill).on_press(Message::ScreenEdgeSelected(ScreenEdge::Left)))
                    (button("Right").width(Length::Fill).on_press(Message::ScreenEdgeSelected(ScreenEdge::Right)))
                )).width(120);

                menu_tpl_1(menu_items!(
                    (submenu_button("Screen Edge >"), menu_screen_edge)
                ))
            })
        )        
        .draw_path(menu::DrawPath::Backdrop)
        .style(|theme:&iced::Theme, status: Status | menu::Style{
            path_border: Border{
                radius: Radius::new(1.0),
                ..Default::default()
            },
            ..primary(theme, status)
        });
        
        // let pick_screen_edge: PickList<'_, ScreenEdge, &[ScreenEdge], ScreenEdge, Message, Theme, Renderer> = pick_list(
        //     &ScreenEdge::ALL[..],
        //     Some(self.screen_edge),
        //     Message::ScreenEdgeSelected,
        // )
        // .placeholder("Edge");

        row![
            button("Tab").on_press(Message::Debug("tab".into())),
            button("q").on_press(Message::Debug("q".into())),
            button("w").on_press(Message::Debug("w".into())),
            button("e").on_press(Message::Debug("e".into())),
            //pick_screen_edge,
            menu,
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





fn base_button<'a>(
    content: impl Into<Element<'a, Message>>,
    msg: Message,
) -> button::Button<'a, Message> {
    button(content)
        .padding([4, 8])
        .style(iced::widget::button::primary)
        .on_press(msg)
}

fn labeled_button(
    label: &str,
    msg: Message,
) -> button::Button<Message, iced::Theme, iced::Renderer> {
    base_button(text(label).align_y(alignment::Vertical::Center), msg)
}

fn debug_button_s(label: &str) -> button::Button<Message, iced::Theme, iced::Renderer> {
    labeled_button(label, Message::Debug(label.into())).width(Length::Shrink)
}

fn submenu_button(label: &str) -> button::Button<Message, iced::Theme, iced::Renderer> {
    base_button(
        row![
            text(label)
                .width(Length::Fill)
                .align_y(alignment::Vertical::Center),
            // text(icon_to_string(RequiredIcons::CaretRightFill))
            //     .font(REQUIRED_FONT)
            //     .width(Length::Shrink)
            //     .align_y(alignment::Vertical::Center),
        ]
        .align_y(iced::Alignment::Center),
        Message::Debug(label.into()),
    )
    .width(Length::Fill)
}


fn debug_button(label: &str) -> button::Button<Message, iced::Theme, iced::Renderer> {
    labeled_button(label, Message::Debug(label.into())).width(Length::Fill)
}
 */