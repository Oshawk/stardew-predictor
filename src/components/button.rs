use std::fmt::Display;
use yew::prelude::*;

#[derive(PartialEq)]
pub enum ButtonColour {
    // $colors in derived-variables.scss + ghost in button.scss
    Default,
    White,
    Black,
    Light,
    Dark,
    Text,
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
    Ghost,
}

impl Display for ButtonColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ButtonColour::Default => "",
                ButtonColour::White => "is-white",
                ButtonColour::Black => "is-black",
                ButtonColour::Light => "is-light",
                ButtonColour::Dark => "is-dark",
                ButtonColour::Text => "is-text",
                ButtonColour::Primary => "is-primary",
                ButtonColour::Link => "is-link",
                ButtonColour::Info => "is-info",
                ButtonColour::Success => "is-success",
                ButtonColour::Warning => "is-warning",
                ButtonColour::Danger => "is-danger",
                ButtonColour::Ghost => "is-ghost",
            }
        )
    }
}

#[derive(Properties, PartialEq)]
pub struct ButtonProperties {
    pub updated: Callback<()>,
    #[prop_or(ButtonColour::Default)]
    pub colour: ButtonColour,
    #[prop_or(AttrValue::from("Button"))]
    pub label: AttrValue,
}

#[function_component]
pub fn Button(properties: &ButtonProperties) -> Html {
    let button_updated: Callback<MouseEvent> = {
        let updated: Callback<()> = properties.updated.clone();
        Callback::from(move |_: MouseEvent| {
            updated.emit(());
        })
    };

    html!(
        <div class="field">
            <div class="control">
                <button class={ classes!("button", properties.colour.to_string()) } onclick={ button_updated }>{ properties.label.clone() }</button>
            </div>
        </div>
    )
}
