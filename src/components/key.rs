use iced::{
    advanced::{
        layout, 
        mouse, 
        renderer::{self, Quad}, 
        widget::{tree, Tree}, 
        Clipboard, 
        Layout, 
        Shell, 
        Text, 
        Widget
    }, alignment::{Horizontal, Vertical}, border, event, keyboard::{self, key::Named}, overlay, touch, widget::{center, container, horizontal_space, mouse_area, stack, text}, Alignment, Border, Color, Element, Event, Length::{self, Fill}, Padding, Rectangle, Shadow, Size, Theme, Vector
};

use crate::app::*;

// https://giesch.dev/iced-hoverable/
// https://docs.iced.rs/iced/widget/struct.Responsive.html
// https://docs.iced.rs/src/iced_widget/lazy/responsive.rs.html#25-30
// https://docs.iced.rs/src/iced_widget/button.rs.html#72
// https://github.com/iced-rs/iced/tree/master/examples/custom_widget
// https://discourse.iced.rs/t/how-to-make-an-advanced-button-widget/826/2


/// Key is a button that stores the visual elements of the widget
/// This is a copy of button.rs but with some extra features
pub struct Key<'a, Message, Renderer = iced::Renderer> 
where
    Renderer: iced::advanced::Renderer,
{
    content: Element<'a, Message, Theme, Renderer>,
    mouse_over: bool,
    highlight: bool,
    on_press: Option<OnPress<'a, Message>>,
}


// State is the internal state of the button
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct State {
    is_pressed: bool,
}

/// OnPress stores the message or closure that will be produced when the [`Key`] is pressed.
enum OnPress<'a, Message> {
    Direct(Message),
    Closure(Box<dyn Fn() -> Message + 'a>),
}

impl<'a, Message: Clone> OnPress<'a, Message> {
    fn get(&self) -> Message {
        match self {
            OnPress::Direct(message) => message.clone(),
            OnPress::Closure(f) => f(),
        }
    }
}

impl<'a, Message, Renderer> Key<'a, Message, Renderer>
where
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
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

    /// Creates a new [`Key`] with the given content.
    pub fn from_str(s: &str,) -> Self {
        let content = text(s.to_string()).center().into();
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


/// The meat & potatoes of the widget
impl<'a, Message, Renderer> Widget<Message, Theme, Renderer> 
    for Key<'a, Message, Renderer>
where
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
    Message: 'a + Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill, //Length::Fill, // fill portion etc is useful!
            height: Length::Fill,
        }
    }


    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let max = limits.max();
        //let size = self.content.as_widget().size_hint();
        let width = max.width; //size.width.fluid();
        let height = max.height; //size.height.fluid();
        let padding = Padding {
            top: 5.0,
            bottom: 5.0,
            right: 10.0,
            left: 10.0,
        };
        layout::padded(
            limits,
            width,
            height,
            padding,
            |limits| {
                self.content.as_widget().layout(
                    &mut tree.children[0],
                    renderer,
                    limits,
                )
            },
        )
    }

    fn draw(
        &self,
        state: &Tree, // tree
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let content_layout = layout.children().next().unwrap();
        let is_mouse_over = cursor.is_over(bounds);

        
        if is_mouse_over {
            // draw background quad
            renderer.fill_quad(
                Quad {
                    bounds: bounds,
                    border: border::rounded(10), //Border::default(),
                    shadow: Shadow::default(),
                },
    Color::from_rgba(1.0, 1.0, 1.0, 0.15),
            );
        }

        // // draw text like a button
        // // this should handle other content such as images etc
        self.content.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            &renderer::Style {
                text_color: Color::from_rgb(1.0, 1.0, 1.0), //style.text_color,
            },
            content_layout, //content_layout,
            cursor,
            &viewport,
        );

        // draw text manually
        // renderer.fill_text(
        //     Text {
        //         content: "q".into(),
        //         bounds: bounds.size(),
        //         size: renderer.default_size(),
        //         line_height: LineHeight::default(),
        //         font: renderer.default_font(),
        //         horizontal_alignment: Horizontal::Center,
        //         vertical_alignment: Vertical::Center,
        //         wrapping: Wrapping::Word,
        //         shaping: Shaping::default(),
        //     },
        //     bounds.center(),
        //     Color::from_rgb(1.0, 1.0, 1.0),
        //     *viewport,
        // );

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

        //event::Status::Ignored
        
        // // event from button.rs
        if let event::Status::Captured = self.content.as_widget_mut().on_event(
            &mut state.children[0],
            event.clone(),
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        ) {
            return event::Status::Captured;
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if self.on_press.is_some() {
                    let bounds = layout.bounds();

                    if cursor.is_over(bounds) {
                        let state = state.state.downcast_mut::<State>();

                        state.is_pressed = true;

                        // comment the following to allow gestures to work
                        //return event::Status::Captured;
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if let Some(on_press) = self.on_press.as_ref().map(OnPress::get)
                {
                    let state = state.state.downcast_mut::<State>();

                    if state.is_pressed {
                        state.is_pressed = false;

                        let bounds = layout.bounds();

                        if cursor.is_over(bounds) {
                            shell.publish(on_press);
                        }

                        // comment the following to allow gestures to work
                        //return event::Status::Captured;
                    }
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                let state = state.state.downcast_mut::<State>();

                state.is_pressed = false;
            }
            _ => {}
        }

        event::Status::Ignored


        // cursor over event
        // if cursor.is_over(layout.bounds()) {
        //     self.mouse_over = true;
        //     self.highlight = true;
        //     match event {
        //         Event::Mouse(mouse::Event::ButtonPressed(_)) => {
        //             if self.on_press.is_some() {
        //                 //let result = Some(self.on_press);
        //                 //shell.publish(Some(self.on_press).clone());
        //             }
        //             event::Status::Captured
        //         }
        //         _ => event::Status::Ignored,
        //     }
        // } else {
        //     self.mouse_over = false;
        //     self.highlight = false;
        //     event::Status::Ignored
        // }


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
    
    fn size_hint(&self) -> Size<Length> {
        self.size()
    }
    
    /// from button.rs
    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    /// from button.rs
    fn operate(
        &self,
        state: &mut Tree, // tree
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.content.as_widget().operate(
                &mut state.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
    }
    
    /// from button.rs
    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
            translation,
        )
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

