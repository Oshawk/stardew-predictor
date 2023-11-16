use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TabsProperties<T: Copy + PartialEq + ToString + 'static> {
    pub tabs: Vec<T>,
    pub updated: Callback<T>,
}

#[function_component]
pub fn Tabs<T: Copy + PartialEq + ToString + 'static>(properties: &TabsProperties<T>) -> Html {
    let selected: UseStateHandle<T> = use_state_eq(|| *properties.tabs.get(0).unwrap());

    html!(
        <div class="tabs">
            <ul>
                {
                    properties.tabs.iter().map(|item: &T| {
                        html!(
                            <li class={ if *selected == *item { "is-active" } else { "" } } >
                                <a onclick={
                                    let item: T = *item;
                                    let selected: UseStateHandle<T> = selected.clone();
                                    let updated: Callback<T> = properties.updated.clone();
                                    Callback::from(move |_: MouseEvent| {
                                        updated.emit(item);
                                        selected.set(item);
                                    })
                                }>{ item.to_string() }</a>
                            </li>
                        )
                    }).collect::<Html>()
                }
            </ul>
        </div>
    )
}
