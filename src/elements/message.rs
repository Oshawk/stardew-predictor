use yew::prelude::*;

#[derive(PartialEq)]
pub enum MessageColour {
    DARK,
    PRIMARY,
    LINK,
    INFO,
    SUCCESS,
    WARNING,
    DANGER,
}

impl ToString for MessageColour {
    fn to_string(&self) -> String {
        match self {
            MessageColour::DARK => "is-dark",
            MessageColour::PRIMARY => "is-primary",
            MessageColour::LINK => "is-link",
            MessageColour::INFO => "is-info",
            MessageColour::SUCCESS => "is-success",
            MessageColour::WARNING => "is-warning",
            MessageColour::DANGER => "is-danger",
        }.to_string()
    }
}

#[derive(Properties, PartialEq)]
pub struct MessageProperties {
    #[prop_or(MessageColour::INFO)]
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
