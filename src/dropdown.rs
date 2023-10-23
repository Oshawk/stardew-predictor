use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DropdownProperties {
    pub items: Vec<String>,
}

#[function_component]
fn Dropdown(properties: &DropdownProperties) -> Html {
    let active: UseStateHandle<bool> = use_state_eq(|| false);
    let active_true: Callback<FocusEvent> = {
        let active: UseStateHandle<bool> = active.clone();
        Callback::from(move |_| active.set(true))
    };
    let active_false: Callback<FocusEvent> = {
        let active: UseStateHandle<bool> = active.clone();
        Callback::from(move |_| active.set(false))
    };

    let selected: UseStateHandle<Option<String>> = use_state_eq(|| None);

    html!(
        <div class="field">
            <div class={ if *active { "dropdown is-active" } else { "dropdown" } }>
                <div class="control">
                    <div class="dropdown-trigger">
                        <button class="button is-justify-content-space-between" onfocus={ active_true } onblur={ active_false }>
                            <span>{ match (*selected).clone() { Some(selected) => selected, None => "".to_string() } }</span>
                            <span class="material-symbols-outlined">{ "expand_more" }</span>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    )
}
