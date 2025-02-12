use iced::{
    widget::row, 
    Element, 
    Length, 
    Task
};
use crate::app::*;
use crate::components::*;


#[derive(Debug, Clone)]
pub struct MiniPickView {
    filter_views: [View; 3],
    shortcuts: Option<Vec<Shortcut>>,
}


impl ViewTrait for MiniPickView {
    fn new() -> Self {
        // add apps here
        let views = [
            View::CompactQwerty,
            View::Settings,
            View::Launcher,
        ];


        MiniPickView {
            filter_views: views,
            shortcuts: None,
        }
    }

    fn init(&mut self, view_handler: &mut ViewHandler) {
        // get a list of views from view_handler.views that match self.filter_views
        let filtered_list: Vec<&Box<dyn ViewTrait>> = view_handler.views
            .iter()
            .filter(|view| self.filter_views.contains(&view.class()))
            .collect();

        // convert to shortcuts
        let shortcuts: Vec<Shortcut> = filtered_list
            .iter()
            .enumerate()
            .map(|(index, view)| Shortcut::new(view.name(), None, None))
            .collect();

        self.shortcuts = Some(shortcuts);

        info!("{:?}", self.shortcuts);
    }

    fn view(&self, _view_handler: &ViewHandler) -> Element<main_app::Message> {
        let shortcuts: Vec<Element<main_app::Message>> = 
            self.shortcuts.as_ref().unwrap()
            .iter()
            .enumerate()
            .map(|(filter_index, app)| app.view(filter_index))
            .collect();

        row(shortcuts).width(Length::Fill).into()
    }

    fn update(&mut self, message: view::Message) -> Task<main_app::Message> {
        match message {
            view::Message::ViewMessage(index) => {
                // optionally we should have each app with an on_pressed? custom widget is needed then?
                //self.views[index].launch();
            }
            _ => {}
        }
        
        Task::none()
    }
    
    fn class(&self) -> View {
        View::QuickPick
    }
}

