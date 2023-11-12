use yew::prelude::*;

#[derive(PartialEq)]
pub enum NavigationDirection {
    Backward,
    Forward,
}

#[derive(Properties, PartialEq)]
pub struct NavigationProperties {
    pub updated: Callback<NavigationDirection>,
    pub disabled: bool,
}

#[function_component]
pub fn Navigation(properties: &NavigationProperties) -> Html {
    let backward_button_updated: Callback<MouseEvent> = {
        let updated: Callback<NavigationDirection> = properties.updated.clone();
        Callback::from(move |_: MouseEvent| {
            updated.emit(NavigationDirection::Backward);
        })
    };

    let forward_button_updated: Callback<MouseEvent> = {
        let updated: Callback<NavigationDirection> = properties.updated.clone();
        Callback::from(move |_: MouseEvent| {
            updated.emit(NavigationDirection::Forward);
        })
    };

    html!(
        <div>
            <button class="button is-pulled-left" disabled={ properties.disabled } onclick={ backward_button_updated }>{ "<" }</button>
            <button class="button is-pulled-right" disabled={ properties.disabled } onclick={ forward_button_updated }>{ ">" }</button>
        </div>
    )
}
