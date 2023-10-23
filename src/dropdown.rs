use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DropdownProperties<T: Copy + PartialEq + ToString + 'static> {
    pub items: Vec<T>,
    pub updated: Callback<T>,
    #[prop_or("Select".to_string())]
    pub prompt: String,
}

#[function_component]
pub fn Dropdown<T: Copy + PartialEq + ToString + 'static>(properties: &DropdownProperties<T>) -> Html {
    let active: UseStateHandle<bool> = use_state_eq(|| false);
    let active_true: Callback<FocusEvent> = {
        let active: UseStateHandle<bool> = active.clone();
        Callback::from(move |_| active.set(true))
    };
    let active_false: Callback<FocusEvent> = {
        let active: UseStateHandle<bool> = active.clone();
        Callback::from(move |_| active.set(false))
    };

    let selected: UseStateHandle<Option<T>> = use_state_eq(|| None);

    html!(
        <div class="field">
            <div class={ if *active { "dropdown is-active" } else { "dropdown" } }>
                <div class="control">
                    <div class="dropdown-trigger">
                        <button class="button is-justify-content-space-between" onfocus={ active_true } onblur={ active_false }>
                            <span>{ match &*selected { Some(selected) => selected.to_string(), None => properties.prompt.clone() } }</span>
                            <span class="material-symbols-outlined">{ "expand_more" }</span>
                        </button>
                    </div>
                </div>
                <div class="dropdown-menu" style="width:100%">
                    <div class="dropdown-content">
                        {
                            properties.items.iter().map(|item| {
                                let selected: UseStateHandle<Option<T>> = selected.clone();
                                let updated: Callback<T> = properties.updated.clone();
                                let item: T = *item;
                                html!{
                                    <a class="dropdown-item" onmousedown={ Callback::<MouseEvent>::from(move |_| {
                                        selected.set(Some(item));
                                        updated.emit(item);
                                    }) }>{ item.to_string() }</a>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </div>
        </div>
    )
}
