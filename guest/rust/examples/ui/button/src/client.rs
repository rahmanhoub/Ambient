use ambient_api::prelude::*;
use ambient_element::{element_component, Element, ElementComponentExt, Hooks};
use ambient_guest_bridge::components::{layout::space_between_items, transform::translation};
use ambient_ui_components::{
    button::{Button, ButtonStyle},
    default_theme::STREET,
    layout::{FlowColumn, FlowRow},
    setup_ui_camera,
    text::Text,
    UIExt,
};

#[element_component]
fn App(_hooks: &mut Hooks) -> Element {
    let card_inner = |text| {
        FlowRow(vec![Text::el(text)])
            .el()
            .with_background(vec4(0.3, 0.3, 0.3, 1.))
            .with_padding_even(20.)
    };

    FlowRow(vec![
        FlowColumn(vec![
            Button::new("Regular", |_| {}).el(),
            Button::new("Primary", |_| {})
                .style(ButtonStyle::Primary)
                .tooltip(Text::el("Tooltip"))
                .el(),
            Button::new("Flat", |_| {}).style(ButtonStyle::Flat).el(),
            Button::new(card_inner("Card"), |_| {})
                .style(ButtonStyle::Card)
                .el(),
            Button::new("Inline", |_| {})
                .style(ButtonStyle::Inline)
                .el(),
        ])
        .el()
        .set(space_between_items(), STREET)
        .with_padding_even(STREET),
        FlowColumn(vec![
            Button::new("Regular toggled", |_| {}).toggled(true).el(),
            Button::new("Primary toggled", |_| {})
                .toggled(true)
                .style(ButtonStyle::Primary)
                .el(),
            Button::new("Flat toggled", |_| {})
                .toggled(true)
                .style(ButtonStyle::Flat)
                .el(),
            Button::new(card_inner("Card toggled"), |_| {})
                .toggled(true)
                .style(ButtonStyle::Card)
                .el(),
            Button::new("Inline toggled", |_| {})
                .toggled(true)
                .style(ButtonStyle::Inline)
                .el(),
        ])
        .el()
        .set(space_between_items(), STREET)
        .with_padding_even(STREET),
        FlowColumn(vec![
            Button::new("Regular disabled", |_| {}).disabled(true).el(),
            Button::new("Primary disabled", |_| {})
                .disabled(true)
                .style(ButtonStyle::Primary)
                .el(),
            Button::new("Flat disabled", |_| {})
                .disabled(true)
                .style(ButtonStyle::Flat)
                .el(),
            Button::new(card_inner("Card disabled"), |_| {})
                .disabled(true)
                .style(ButtonStyle::Card)
                .el(),
            Button::new("Inline disabled", |_| {})
                .disabled(true)
                .style(ButtonStyle::Inline)
                .el(),
        ])
        .el()
        .set(space_between_items(), STREET)
        .with_padding_even(STREET),
        Button::new("\u{f1e2}", |_| {}).el(),
    ])
    .el()
    .set(space_between_items(), STREET)
    .set(translation(), vec3(100., 100., 0.))
}

#[main]
pub async fn main() -> EventResult {
    setup_ui_camera();
    App.el().spawn_interactive();

    EventOk
}
