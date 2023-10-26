use std::str::FromStr;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProperties {
    pub updated: Callback<()>,
    #[prop_or("Button".to_string())]
    pub label: String,
}

#[function_component]
pub fn Button(properties: &ButtonProperties) -> Html {
    let clicked: Callback<MouseEvent> = {
        let updated: Callback<()> = properties.updated.clone();
        Callback::from(move |event: MouseEvent| {
            updated.emit(());
        })
    };

    html!(
        <div class="field">
            <div class="control">
                <button class="button" onclick={ clicked }>{ properties.label.clone() }</button>
            </div>
        </div>
    )
}
