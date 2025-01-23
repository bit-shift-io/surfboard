use iced::{
    widget::{Button, Responsive, Text}, Element, Length, Size
};

pub struct KeyButton {
    label: String,
}

impl KeyButton {
    pub fn new(label: String) -> Self {
        KeyButton { label }
    }

    // Function to create a responsive widget
    pub fn view(&self) -> Responsive<'_, ()> {
        Responsive::new(move |size: Size| {
            // Create a button that adapts based on the size
            let button = Button::new(Text::new(&self.label))
                .width(if size.width > 400.0 { Length::Fixed(200.0) } else { Length::Fill })
                .height(Length::Fixed(50.0));

            // Return the button as an Element
            button.into()
        })
    }
}