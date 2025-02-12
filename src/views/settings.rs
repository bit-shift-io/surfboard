use iced::{
    widget::{
        pick_list, 
        row, 
        Button, 
        PickList, 
        Text}, 
        Element, 
        Length, 
        Renderer, 
        Theme
    };
use crate::app::*;


#[derive(Copy, Debug, Clone)]
pub struct SettingsView;


impl ViewTrait for SettingsView {
    fn new() -> Self where Self: Sized {
        SettingsView{}
    }

    fn view(&self, _view_handler: &ViewHandler) -> Element<main_app::Message> {
        let view_main = Button::new(Text::new("main")).on_press(main_app::Message::ViewHandler(view::Message::ChangeView(View::CompactQwerty)));
        let view_launcher = Button::new(Text::new("launcher")).on_press(main_app::Message::ViewHandler(view::Message::ChangeView(View::Launcher)));
        
        let pick_view: PickList<'_, View, &[View], View, main_app::Message, Theme, Renderer> = pick_list(
            &View::ALL[..],
            None,
            |view| main_app::Message::ViewHandler(view::Message::ChangeView(view)),
        )
        .placeholder("View");

        let pick_dock: PickList<'_, Dock, &[Dock], Dock, main_app::Message, Theme, Renderer> = pick_list(
            &Dock::ALL[..],
            None,
            |dock| main_app::Message::WindowHandler(window::Message::Dock(dock)),
        )
        .placeholder("Edge");

        row![
            pick_dock,
            pick_view,
            view_main,
            view_launcher,
        ]
            .padding(20)
            .width(Length::Fill)
            .into()
    }
    
    fn class(&self) -> View {
        View::Settings
    }
    
    fn name(&self) -> String {
        String::from("Settings")
    }
}