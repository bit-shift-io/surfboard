use iced::{
    widget::{center, container, horizontal_space, mouse_area, stack, text},
    Element, Length::Fill,
};

// https://giesch.dev/iced-hoverable/
// https://docs.iced.rs/iced/widget/struct.Responsive.html
// https://docs.iced.rs/src/iced_widget/lazy/responsive.rs.html#25-30
// https://docs.iced.rs/src/iced_widget/button.rs.html#72
// https://github.com/iced-rs/iced/tree/master/examples/custom_widget


// https://discourse.iced.rs/t/how-to-make-an-advanced-button-widget/826/2

pub fn key<'a, Message>(
    content: &'a str,
    footnote: Option<&'a str>,
    on_enter: Message,
    on_exit: Message,
    on_press: Message,
    on_release: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'static,
{
    let footnote = container(match footnote {
        Some(footnote) => text(footnote).size(12).into(),
        None => Element::from(horizontal_space()),
    })
    .align_bottom(Fill)
    .align_right(Fill)
    .width(Fill)
    .height(Fill)
    .padding(2);

    container(
        mouse_area(stack![
            center(text(content).size(16)).width(Fill).height(Fill),
            footnote
        ])
        .on_enter(on_enter)
        .on_exit(on_exit)
        .on_press(on_press)
        .on_release(on_release),
    )
    .style(container::rounded_box)
    .into()
}