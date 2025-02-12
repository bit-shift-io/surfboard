use iced::{
    widget::row, 
    Element, 
    Length, 
    Task
};
use crate::app::*;
use crate::components::*;


#[derive(Debug, Clone)]
pub struct PickView {
    shortcuts: Option<Vec<Shortcut>>,
}


impl ViewTrait for PickView {
    fn new() -> Self {
        PickView {
            shortcuts: None,
        }
    }

    fn init(&mut self, view_handler: &mut ViewHandler) {
        // convert all views to shortcuts
        let shortcuts: Vec<Shortcut> = view_handler.views
            .iter()
            .enumerate()
            .map(|(index, view)| Shortcut::new(view.name(), view.icon(), None))
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
        View::Pick
    }
}

