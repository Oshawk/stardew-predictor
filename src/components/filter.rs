use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FilterProperties {
    pub updated: Callback<String>,
}

#[function_component]
pub fn Filter(properties: &FilterProperties) -> Html {
    let value: UseStateHandle<String> = use_state_eq(|| "".to_string());

    let value_updated: Callback<InputEvent> = {
        let value: UseStateHandle<String> = value.clone();
        Callback::from(move |event: InputEvent| {
            value.set(event.target_unchecked_into::<HtmlInputElement>().value());
        })
    };

    let clear_updated: Callback<MouseEvent> = {
        let value: UseStateHandle<String> = value.clone();
        let updated: Callback<String> = properties.updated.clone();
        Callback::from(move |_: MouseEvent| {
            updated.emit("".to_string());
            value.set("".to_string());
        })
    };

    let filter_updated: Callback<MouseEvent> = {
        let value: UseStateHandle<String> = value.clone();
        let updated: Callback<String> = properties.updated.clone();
        Callback::from(move |_: MouseEvent| {
            updated.emit(value.to_lowercase());
        })
    };

    html!(
        <div class="field has-addons">
            <div class="control is-expanded">
                <input class="input" oninput={ value_updated } placeholder="Filter" type="text" value={ (*value).clone() } />
            </div>
            <div class="control">
                <button class="button is-danger" onclick={ clear_updated }>{ "Clear" }</button>
            </div>
            <div class="control">
                <button class="button is-primary" onclick={ filter_updated }>{ "Filter" }</button>
            </div>
        </div>
    )
}
