use iced::{
    advanced::{
        layout, 
        mouse, 
        renderer::{self, Quad}, 
        widget::{tree, Tree}, 
        Clipboard, 
        Layout, 
        Shell, 
        Widget
    }, 
    border, 
    overlay, 
    touch, 
    widget::{text, Text}, 
    Color, 
    Element, 
    Event, 
    Length, 
    Padding, 
    Rectangle, 
    Shadow, 
    Size, 
    Theme, 
    Vector
};
use iced_core::window;

use crate::app::{component, main_app, ComponentHandler};


// https://giesch.dev/iced-hoverable/
// https://docs.iced.rs/iced/widget/struct.Responsive.html
// https://docs.iced.rs/src/iced_widget/lazy/responsive.rs.html#25-30
// https://docs.iced.rs/src/iced_widget/button.rs.html#72
// https://github.com/iced-rs/iced/tree/master/examples/custom_widget
// https://discourse.iced.rs/t/how-to-make-an-advanced-button-widget/826/2


/// Key is a button that stores the visual elements of the widget
/// This is a copy of button.rs but with some extra features
pub struct Key<'a, Message, Theme, Renderer = iced::Renderer> 
where
    Renderer: iced_core::Renderer,
{
    content: Element<'a, Message, Theme, Renderer>,
    on_press: Option<OnPress<'a, Message>>,
    on_resize: Option<Box<dyn Fn(Size) -> Message + 'a>>,
    on_show: Option<Box<dyn Fn(Size) -> Message + 'a>>,
    on_bounds: Option<Box<dyn Fn(Rectangle) -> Message + 'a>>,
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

impl<'a, Message, Renderer> Key<'a, Message, Theme, Renderer>
where
    Renderer: 'a + iced_core::Renderer + iced_core::text::Renderer,
    Message: 'a + Clone,
{
    /// Creates a new [`Key`] with the given content.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>,) -> Self {
        let content = content.into();
        Self {
            content,
            on_press: None,
            on_resize: None,
            on_show: None,
            on_bounds: None
        }
    }

    /// Sets the message to be produced when the content pops into view.
    ///
    /// The closure will receive the [`Size`] of the content in that moment.
    /// from pop.rs
    pub fn on_show(mut self, on_show: impl Fn(Size) -> Message + 'a) -> Self {
        info!("on_show");
        self.on_show = Some(Box::new(on_show));
        self
    }

    /// Sets the message to be produced when the content changes [`Size`] once its in view.
    ///
    /// The closure will receive the new [`Size`] of the content.
    /// from pop.rs
    pub fn on_resize(
        mut self,
        on_resize: impl Fn(Size) -> Message + 'a,
    ) -> Self {
        self.on_resize = Some(Box::new(on_resize));
        self
    }

    /// Sets the message to be produced when the content changes [`Rectangle`] once its in view.
    ///
    /// The closure will receive the new [`Rectangle`] of the content.
    /// from pop.rs
    pub fn on_bounds(
        mut self,
        on_bounds: impl Fn(Rectangle) -> Message + 'a,
    ) -> Self {
        self.on_bounds = Some(Box::new(on_bounds));
        self
    }

    /// Creates a new [`Key`] with the given content.
    pub fn from_str(s: &str,) -> Self {
        let content = text(s.to_string()).center().into();
        Self {
            content,
            on_press: None,
            on_resize: None,
            on_show: None,
            on_bounds: None
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



// State is the internal state of the button
#[derive(Debug, Clone, Default)]
struct State {
    is_pressed: bool,
    last_size: Option<Size>,
}


/// The meat & potatoes of the widget
impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> 
    for Key<'_, Message, Theme, Renderer>
where
    Renderer: iced_core::Renderer + iced_core::text::Renderer,
    Message: Clone,
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


    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        // notify on resize
        if let Event::Window(window::Event::RedrawRequested(now)) = &event {
            let state = tree.state.downcast_mut::<State>();
            let bounds = layout.bounds();
            let top_left_distance = viewport.distance(bounds.position());
            let bottom_right_distance = viewport
                .distance(bounds.position() + Vector::from(bounds.size()));

            let distance = top_left_distance.min(bottom_right_distance);
            if let Some(on_bounds) = &self.on_bounds {
                let size = bounds.size();
                if Some(size) != state.last_size {
                    state.last_size = Some(size);
                    shell.publish(on_bounds(bounds));
                    //let msg: Message = main_app::Message::Debug(String::from("q"));
                    //let closure = |rectangle| main_app::Message::ComponentHandler(component::Message::Update(String::from("q"), rectangle));
                    //shell.publish(closure);
                }
            }
        }


        // TODO: key is gobbling our events. Need to fix this to work with gestures - short tap vs glide gesture


        // update sub component widget
        // self.content.as_widget_mut().update(
        //     &mut tree.children[0],
        //     event,
        //     layout.children().next().unwrap(),
        //     cursor,
        //     renderer,
        //     clipboard,
        //     shell,
        //     viewport,
        // );

        // // return if event captured by widget
        // if shell.is_event_captured() {
        //     return;
        // }

        // which event
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if self.on_press.is_some() {
                    let bounds = layout.bounds();

                    if cursor.is_over(bounds) {
                        let state = tree.state.downcast_mut::<State>();

                        state.is_pressed = true;

                        // disable for gestures
                        //shell.capture_event();
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if let Some(on_press) = &self.on_press {
                    let state = tree.state.downcast_mut::<State>();

                    if state.is_pressed {
                        state.is_pressed = false;

                        let bounds = layout.bounds();

                        if cursor.is_over(bounds) {
                            shell.publish(on_press.get());
                        }

                        // disable for gestures
                        //shell.capture_event();
                    }
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                let state = tree.state.downcast_mut::<State>();

                state.is_pressed = false;
            }
            _ => {}
        }

        // more stuff from button.rs
        // let current_status = if self.on_press.is_none() {
        //     Status::Disabled
        // } else if cursor.is_over(layout.bounds()) {
        //     let state = tree.state.downcast_ref::<State>();

        //     if state.is_pressed {
        //         Status::Pressed
        //     } else {
        //         Status::Hovered
        //     }
        // } else {
        //     Status::Active
        // };

        // if let Event::Window(window::Event::RedrawRequested(_now)) = event {
        //     self.status = Some(current_status);
        // } else if self.status.is_some_and(|status| status != current_status) {
        //     shell.request_redraw();
        // }



        /*

        // todo: Brons help, how do i get some sort of animation loop here for a key?
        // for animation: https://github.com/iced-rs/iced/blob/master/examples/loading_spinners/src/circular.rs
        // Fab: check the link above. Update function is for recieving messages on the message bus. So this will be for storing states.
        // use the draw function to draw any animations. Use the macro "info!("HELP!");" to print to console
        // or info!("{:?}", some_value); to print out values.. pop it in the update and draw functions to see how they operate :)
        // Fab: this should work now were on 0.14? I guess!
        
        */
        /* 
        let state = tree.state.downcast_mut::<State>();

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            state.animation = state.animation.timed_transition(
                self.cycle_duration,
                self.rotation_duration,
                *now,
            );

            state.cache.clear();
            shell.request_redraw();
        }*/
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
        operation: &mut dyn iced_core::widget::Operation,
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

impl<'a, Message, Theme, Renderer> From<Key<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: 'a + iced_core::Renderer + iced_core::text::Renderer,
    Theme: 'a,
    Message: 'a + Clone,
{
    fn from(key: Key<'a, Message, Theme, Renderer>) -> Self {
        Element::new(key)
    }
}

