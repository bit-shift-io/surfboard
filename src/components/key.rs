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
    }, 
    alignment::{Horizontal, Vertical}, 
    event, keyboard::{self, key::Named}, 
    widget::{center, container, horizontal_space, mouse_area, stack, text::{LineHeight, Shaping, Wrapping}}, 
    Border, 
    Color, 
    Element, 
    Event, 
    Length::{self, Fill}, 
    Rectangle, 
    Settings, 
    Shadow, 
    Size, 
    Theme
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


pub struct Key<Message> {
    mouse_over: bool,
    highlight: bool,
    on_press: Message,
}


impl<Message> Key<Message> {
    pub fn new(on_press: Message) -> Self {
        Self { 
            mouse_over: false,
            highlight: false,
            on_press,
        }
    }
}


impl<Message, Renderer> Widget<Message, Theme, Renderer> for Key<Message>
where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
    Message: Clone,
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
        _state: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        // draw background quad
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border: Border {
                    color: Color::from_rgb(0.6, 0.8, 1.0),
                    width: 1.0,
                    radius: 0.0.into(),
                },
                shadow: Shadow::default(),
            },
            if self.highlight {
                Color::from_rgb(0.6, 0.8, 1.0)
            } else {
                Color::from_rgb(0.0, 0.2, 0.4)
            },
        );

        // draw text
        renderer.fill_text(
            Text {
                content: "blah".into(),
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
            Color::from_rgb(0.6, 0.8, 1.0),
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
        _state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {

        // cursor over event
        if cursor.is_over(layout.bounds()) {
            self.mouse_over = true;
            self.highlight = true;
            match event {
                Event::Mouse(mouse::Event::ButtonPressed(_)) => {
                    shell.publish(self.on_press.clone());
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


impl<'a, Message: 'a, Renderer> From<Key<Message>> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
    Message: Clone,
{
    fn from(widget: Key<Message>) -> Self {
        Self::new(widget)
    }
}
