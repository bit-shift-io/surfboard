use iced::{widget::{button, pick_list, row, Button, Container, PickList, Text}, Alignment, Length, Renderer, Task as Command, Theme};
use crate::{app::{MainMessage, ViewTrait}, ScreenEdge, View};


#[derive(Debug, Clone)]
pub struct SettingsView;


impl ViewTrait for SettingsView {
    fn new() -> Self where Self: Sized {
        SettingsView{}
    }

    fn view(&self) -> iced::Element<MainMessage> {

        // // menu
        // let menu_tpl_1 = |items| Menu::new(items).max_width(180.0).offset(15.0).spacing(5.0);
        // let menu_tpl_2 = |items| Menu::new(items).max_width(180.0).offset(0.0).spacing(5.0);
        // let menu = menu_bar!(
        //     (debug_button_s("="), {
        //         let menu_screen_edge = menu_tpl_2(menu_items!(
        //             (button("Top").width(Length::Fill).on_press(MainWindowMessage::ScreenEdgeSelected(ScreenEdge::Top)))
        //             (button("Bottom").width(Length::Fill).on_press(MainWindowMessage::ScreenEdgeSelected(ScreenEdge::Bottom)))
        //             (button("Left").width(Length::Fill).on_press(MainWindowMessage::ScreenEdgeSelected(ScreenEdge::Left)))
        //             (button("Right").width(Length::Fill).on_press(MainWindowMessage::ScreenEdgeSelected(ScreenEdge::Right)))
        //         )).width(120);

        //         menu_tpl_1(menu_items!(
        //             (submenu_button("Screen Edge >"), menu_screen_edge)
        //         ))
        //     })
        // )        
        // .draw_path(menu::DrawPath::Backdrop)
        // .style(|theme:&iced::Theme, status: Status | menu::Style{
        //     path_border: Border{
        //         radius: Radius::new(1.0),
        //         ..Default::default()
        //     },
        //     ..primary(theme, status)
        // });

        let view_main = Button::new(Text::new("main")).on_press(MainMessage::ChangeView(View::Main));
        
        let pick_screen_edge: PickList<'_, ScreenEdge, &[ScreenEdge], ScreenEdge, MainMessage, Theme, Renderer> = pick_list(
            &ScreenEdge::ALL[..],
            None,
            MainMessage::ChangeScreenEdge,
        )
        .placeholder("Edge");

        row![
            button("Tab").on_press(MainMessage::Debug("tab".into())),
            button("q").on_press(MainMessage::Debug("q".into())),
            button("w").on_press(MainMessage::Debug("w".into())),
            button("e").on_press(MainMessage::Debug("e".into())),
            pick_screen_edge,
            view_main,
            //menu,
        ]
        .padding(20)
        .width(Length::Fill)
        .into()
    }

    fn name(&self) -> String {
        String::from("settings")
    }
    
    fn class(&self) -> View {
        View::Settings
    }
    
    fn has_gesture(&self) -> bool {
        false
    }
}




/*
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