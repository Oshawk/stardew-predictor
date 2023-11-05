use yew::prelude::*;

#[derive(PartialEq)]
pub enum MessageColour {
    Dark,
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
}

impl ToString for MessageColour {
    fn to_string(&self) -> String {
        match self {
            MessageColour::Dark => "is-dark",
            MessageColour::Primary => "is-primary",
            MessageColour::Link => "is-link",
            MessageColour::Info => "is-info",
            MessageColour::Success => "is-success",
            MessageColour::Warning => "is-warning",
            MessageColour::Danger => "is-danger",
        }.to_string()
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
