use std::str::FromStr;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputProperties<T: Copy + FromStr + PartialEq + ToString + 'static> {
    pub updated: Callback<Option<T>>,
    #[prop_or("Input".to_string())]
    pub label: String,
}

#[function_component]
pub fn Input<T: Copy + FromStr + PartialEq + ToString + 'static>(properties: &InputProperties<T>) -> Html {
    let value: UseStateHandle<Option<T>> = use_state(|| None);
    let value_change: Callback<InputEvent> = {
        let value: UseStateHandle<Option<T>> = value.clone();
        let updated: Callback<Option<T>> = properties.updated.clone();
        Callback::from(move |event: InputEvent| {
            let value_string: String = event.target_unchecked_into::<HtmlInputElement>().value();
            match value_string.parse::<T>() {
                Ok(value_) => {
                    value.set(Some(value_));
                }
                Err(a) => {
                    if value_string != "-" {
                        value.set(None);
                    }
                }
            }
            updated.emit(*value);
        })
    };

    html!(
        <div class="field">
            <label class="label">{ properties.label.clone() }</label>
            <div class="control">
                <input class="input" oninput={ value_change } placeholder={ properties.label.clone() } type="text" value={ match *value { Some(value) => value.to_string(), None => "".to_string() } }/>
            </div>
        </div>
    )
}
