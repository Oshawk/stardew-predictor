use std::fmt::Display;
use yew::prelude::*;

#[derive(PartialEq)]
pub enum MessageColour {
    // $colors in derived-variables.scss
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
}

impl Display for MessageColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MessageColour::Default => "",
                MessageColour::White => "is-white",
                MessageColour::Black => "is-black",
                MessageColour::Light => "is-light",
                MessageColour::Dark => "is-dark",
                MessageColour::Text => "is-text",
                MessageColour::Primary => "is-primary",
                MessageColour::Link => "is-link",
                MessageColour::Info => "is-info",
                MessageColour::Success => "is-success",
                MessageColour::Warning => "is-warning",
                MessageColour::Danger => "is-danger",
            }
        )
    }
}

#[derive(Properties, PartialEq)]
pub struct MessageProperties {
    #[prop_or(MessageColour::Info)]
    pub colour: MessageColour,
    #[prop_or(None)]
    pub header: Option<AttrValue>,
    #[prop_or(None)]
    pub body: Option<AttrValue>,
}

#[function_component]
pub fn Message(properties: &MessageProperties) -> Html {
    html!(
        <article class={ classes!("message", properties.colour.to_string()) }>
            {
                match properties.header.clone() {
                    Some(header) => html!(
                        <div class="message-header">
                            <p>{ header }</p>
                        </div>
                    ),
                    None => html!(),
                }
            }
            {
                match properties.body.clone() {
                    Some(body) => html!(
                        <div class="message-body">{ body }</div>
                    ),
                    None => html!(),
                }
            }
        </article>
    )
}
