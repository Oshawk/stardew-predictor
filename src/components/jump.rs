use std::str::FromStr;

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct JumpProperties<T: Copy + FromStr + PartialEq + ToString + 'static> {
    pub updated: Callback<T>,
    #[prop_or(AttrValue::from("Jump"))]
    pub label: AttrValue,
}

#[component]
pub fn Jump<T: Copy + FromStr + PartialEq + ToString + 'static>(
    properties: &JumpProperties<T>,
) -> Html {
    let text = use_state(|| String::new());
    let parsed = use_state(|| None::<T>);

    let value_updated = {
        let text = text.clone();
        let parsed = parsed.clone();
        Callback::from(move |event: InputEvent| {
            let input = event.target_unchecked_into::<HtmlInputElement>().value();
            match input.parse::<T>() {
                Ok(value) => {
                    parsed.set(Some(value));
                    text.set(input);
                }
                Err(_) => {
                    parsed.set(None);
                    if input == "-" {
                        text.set(input);
                    } else {
                        text.set(String::new());
                    }
                }
            }
        })
    };

    let jump_updated = {
        let parsed = parsed.clone();
        let updated = properties.updated.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(value) = *parsed {
                updated.emit(value);
            }
        })
    };

    html!(
        <div class="field has-addons">
            <div class="control is-expanded">
                <input class="input" oninput={ value_updated } placeholder={ properties.label.clone() } type="text" value={ (*text).clone() } />
            </div>
            <div class="control">
                <button class="button is-primary" onclick={ jump_updated }>{ "Jump" }</button>
            </div>
        </div>
    )
}
