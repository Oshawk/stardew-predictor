use std::str::FromStr;

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputProperties<T: Copy + FromStr + PartialEq + ToString + 'static> {
    pub updated: Callback<Option<T>>,
    #[prop_or(AttrValue::from("Input"))]
    pub label: AttrValue,
}

#[component]
pub fn Input<T: Copy + FromStr + PartialEq + ToString + 'static>(
    properties: &InputProperties<T>,
) -> Html {
    let text = use_state(|| String::new());

    let value_updated = {
        let text = text.clone();
        let updated = properties.updated.clone();
        Callback::from(move |event: InputEvent| {
            let input = event.target_unchecked_into::<HtmlInputElement>().value();
            match input.parse::<T>() {
                Ok(value) => {
                    updated.emit(Some(value));
                    text.set(input);
                }
                Err(_) => {
                    updated.emit(None);
                    // Allow typing "-" for negative numbers without resetting
                    if input == "-" {
                        text.set(input);
                    } else {
                        text.set(String::new());
                    }
                }
            }
        })
    };

    html!(
        <div class="field">
            <label class="label">{ properties.label.clone() }</label>
            <div class="control">
                <input class="input" oninput={ value_updated } placeholder={ properties.label.clone() } type="text" value={ (*text).clone() } />
            </div>
        </div>
    )
}
