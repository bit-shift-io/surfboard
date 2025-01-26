use iced::{
    advanced::{
        layout, 
        mouse, 
        renderer::{self, Quad}, 
        widget::Tree, 
        Clipboard, 
        Layout, 
        Shell, 
        Text, 
        Widget
    }, alignment::{Horizontal, Vertical}, border, event, keyboard::{self, key::Named}, touch, widget::{center, container, horizontal_space, mouse_area, stack, text::{LineHeight, Shaping, Wrapping}}, Border, Color, Element, Event, Length::{self, Fill}, Rectangle, Shadow, Size, Theme
};

use crate::app::*;

// https://giesch.dev/iced-hoverable/
// https://docs.iced.rs/iced/widget/struct.Responsive.html
// https://docs.iced.rs/src/iced_widget/lazy/responsive.rs.html#25-30
// https://docs.iced.rs/src/iced_widget/button.rs.html#72
// https://github.com/iced-rs/iced/tree/master/examples/custom_widget


// https://discourse.iced.rs/t/how-to-make-an-advanced-button-widget/826/2


// pub fn key<'a, Message>(
//     content: &'a str,
//     //footnote: Option<&'a str>,
//     on_enter: Message,
//     on_exit: Message,
//     on_press: Message,
//     on_release: Message,
// ) -> Element<'a, Message>
// where
//     Message: Clone + 'static,
// {
//     // let footnote = container(match footnote {
//     //     Some(footnote) => text(footnote).size(12).into(),
//     //     None => Element::from(horizontal_space()),
//     // })
//     // .align_bottom(Fill)
//     // .align_right(Fill)
//     // .width(Fill)
//     // .height(Fill)
//     // .padding(2);

//     container(
//         mouse_area(stack![
//             center(text(content).size(16)).width(Fill).height(Fill),
//             //footnote
//         ])
//         .on_enter(on_enter)
//         .on_exit(on_exit)
//         .on_press(on_press)
//         .on_release(on_release),
//     )
//     .style(container::rounded_box)
//     .into()
// }


pub struct Key<'a, Message, Renderer = iced::Renderer> 
where
    Renderer: iced::advanced::Renderer,
{
    content: Element<'a, Message, Theme, Renderer>,
    mouse_over: bool,
    highlight: bool,
    on_press: Option<OnPress<'a, Message>>,
}

enum OnPress<'a, Message> {
    Direct(Message),
    Closure(Box<dyn Fn() -> Message + 'a>),
}


impl<'a, Message, Renderer> Key<'a, Message, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    /// Creates a new [`Key`] with the given content.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>,) -> Self {
        let content = content.into();
        Self {
            content,
            mouse_over: false,
            highlight: false,
            on_press: None,
        }
    }

    /// Sets the message that will be produced when the [`Key`] is pressed.
    ///
    /// Unless `on_press` is called, the [`Key`] will be disabled.
    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_press = Some(OnPress::Direct(on_press));
        self
    }

    /// Sets the message that will be produced when the [`Button`] is pressed.
    ///
    /// This is analogous to [`Button::on_press`], but using a closure to produce
    /// the message.
    ///
    /// This closure will only be called when the [`Button`] is actually pressed and,
    /// therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow.
    pub fn on_press_with(
        mut self,
        on_press: impl Fn() -> Message + 'a,
    ) -> Self {
        self.on_press = Some(OnPress::Closure(Box::new(on_press)));
        self
    }

    /// Sets the message that will be produced when the [`Button`] is pressed,
    /// if `Some`.
    ///
    /// If `None`, the [`Button`] will be disabled.
    pub fn on_press_maybe(mut self, on_press: Option<Message>) -> Self {
        self.on_press = on_press.map(OnPress::Direct);
        self
    }
}


impl<'a, Message, Renderer> Widget<Message, Theme, Renderer> for Key<'a, Message, Renderer>
where
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
    Message: 'a + Clone,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(100.0, 100.0))
    }

    fn draw(
        &self,
        state: &Tree, // tree
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        //let content_layout = layout.children().next().unwrap();
        let is_mouse_over = cursor.is_over(bounds);

        
        if is_mouse_over {

            // draw background quad
            renderer.fill_quad(
                Quad {
                    bounds: bounds,
                    border: border::rounded(20), //Border::default(),
                    shadow: Shadow::default(),
                },
                if self.highlight {
                    Color::from_rgba(1.0, 1.0, 1.0, 0.25)
                } else {
                    Color::from_rgb(0.0, 0.0, 0.0)
                },
            );
        }



        // draw text like a button
        // self.content.as_widget().draw(
        //     &state.children[0],
        //     renderer,
        //     theme,
        //     &renderer::Style {
        //         text_color: style.text_color,
        //     },
        //     content_layout,
        //     cursor,
        //     &viewport,
        // );

        // draw text manually
        renderer.fill_text(
            Text {
                content: "q".into(),
                bounds: bounds.size(),
                size: renderer.default_size(),
                line_height: LineHeight::default(),
                font: renderer.default_font(),
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                wrapping: Wrapping::Word,
                shaping: Shaping::default(),
            },
            bounds.center(),
            Color::from_rgb(1.0, 1.0, 1.0),
            *viewport,
        );
    }

    /// cursor type
    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::Idle
        }
    }


    fn on_event(
        &mut self,
        state: &mut Tree, // tree
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {

        // // event from button.rs
        // if let event::Status::Captured = self.content.as_widget_mut().on_event(
        //     &mut state.children[0],
        //     event.clone(),
        //     layout.children().next().unwrap(),
        //     cursor,
        //     renderer,
        //     clipboard,
        //     shell,
        //     viewport,
        // ) {
        //     return event::Status::Captured;
        // }

        // match event {
        //     Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
        //     | Event::Touch(touch::Event::FingerPressed { .. }) => {
        //         if self.on_press.is_some() {
        //             let bounds = layout.bounds();

        //             if cursor.is_over(bounds) {
        //                 let state = state.state.downcast_mut::<State>();

        //                 state.is_pressed = true;

        //                 return event::Status::Captured;
        //             }
        //         }
        //     }
        //     Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
        //     | Event::Touch(touch::Event::FingerLifted { .. }) => {
        //         if let Some(on_press) = self.on_press.as_ref().map(OnPress::get)
        //         {
        //             let state = state.state.downcast_mut::<State>();

        //             if state.is_pressed {
        //                 state.is_pressed = false;

        //                 let bounds = layout.bounds();

        //                 if cursor.is_over(bounds) {
        //                     shell.publish(on_press);
        //                 }

        //                 return event::Status::Captured;
        //             }
        //         }
        //     }
        //     Event::Touch(touch::Event::FingerLost { .. }) => {
        //         let state = state.state.downcast_mut::<State>();

        //         state.is_pressed = false;
        //     }
        //     _ => {}
        // }

        // event::Status::Ignored


        // cursor over event
        if cursor.is_over(layout.bounds()) {
            self.mouse_over = true;
            self.highlight = true;
            match event {
                Event::Mouse(mouse::Event::ButtonPressed(_)) => {
                    if self.on_press.is_some() {
                        //let result = Some(self.on_press);
                        //shell.publish(Some(self.on_press).clone());
                    }
                    event::Status::Captured
                }
                _ => event::Status::Ignored,
            }
        } else {
            self.mouse_over = false;
            self.highlight = false;
            event::Status::Ignored
        }


        // keyboard event
        // match event {
        //     Event::Keyboard(keyboard::Event::KeyPressed {
        //         key: keyboard::Key::Named(Named::Space),
        //         ..
        //     }) => {
        //         self.highlight = !self.highlight;
        //         event::Status::Captured
        //     }
        //     _ => event::Status::Ignored,
        // }
    }
}

impl<'a, Message, Renderer> From<Key<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
    Message: 'a + Clone,
{
    fn from(widget: Key<'a, Message, Renderer>) -> Self {
        Self::new(widget)
    }
}

// impl<'a, Message: 'a, Theme, Renderer> From<Key<'a, Message, Theme, Renderer>> for Element<'a, Message, Theme, Renderer>
// where
//     Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
//     Message: Clone,
// {
//     fn from(widget: Key<'a, Message, Theme, Renderer>) -> Self {
//         Self::new(widget)
//     }
// }
