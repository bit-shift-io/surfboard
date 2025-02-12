use iced::{
    widget::row, 
    Element, 
    Length, 
    Task
};
use crate::app::*;
use crate::components::*;
use crate::utils::*;


#[derive(Debug, Clone)]
pub struct LauncherView {
    applications: [App; 3],
}


impl ViewTrait for LauncherView {
    fn new() -> Self {
        // add apps here
        let applications = [
            App::new("/usr/share/applications/code.desktop"),
            App::new("/usr/share/applications/org.kde.konsole.desktop"),
            App::new("/usr/share/applications/firefox.desktop"),
        ];

        LauncherView {
            applications,
        }
    }

    fn view(&self, _view_handler: &ViewHandler) -> Element<main_app::Message> {
        let apps: Vec<Element<main_app::Message>> = 
            self.applications
            .iter()
            .enumerate()
            .map(|(filter_index, app)| app.view(filter_index))
            .collect();

        row(apps).width(Length::Fill).into()
    }

    fn update(&mut self, message: view::Message) -> Task<main_app::Message> {
        match message {
            view::Message::ViewMessage(index) => {
                // optionally we should have each app with an on_pressed? custom widget is needed then?
                self.applications[index].launch();
            }
            _ => {}
        }
        
        Task::none()
    }
    
    fn class(&self) -> View {
        View::Launcher
    }

    fn icon(&self) -> &'static [u8] {
        globals::ICON_TABLE
    }
}

