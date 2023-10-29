use std::str::FromStr;

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputProperties<T: Copy + FromStr + PartialEq + ToString + 'static> {
    pub updated: Callback<Option<T>>,
    #[prop_or(AttrValue::from("Input"))]
    pub label: AttrValue,
}

#[function_component]
pub fn Input<T: Copy + FromStr + PartialEq + ToString + 'static>(properties: &InputProperties<T>) -> Html {
    let value: UseForceUpdateHandle = use_force_update();
    let value_updated: Callback<InputEvent> = {
        let value: UseForceUpdateHandle = value.clone();
        let updated: Callback<Option<T>> = properties.updated.clone();
        Callback::from(move |event: InputEvent| {
            let value_string: String = event.target_unchecked_into::<HtmlInputElement>().value();
            match value_string.parse::<T>() {
                Ok(value_) => {
                    updated.emit(Some(value_));
                }
                Err(_) => {
                    updated.emit(None);
                    if value_string != "-" {
                        value.force_update();
                    }
                }
            }
        })
    };

    html!(
        <div class="field">
            <label class="label">{ properties.label.clone() }</label>
            <div class="control">
                <input class="input" oninput={ value_updated } placeholder={ properties.label.clone() } type="text" value=""/>
            </div>
        </div>
    )
}
