use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TabsProperties<T: Copy + PartialEq + ToString + 'static> {
    pub tabs: Vec<T>,
    pub selected: Option<T>,
    pub updated: Callback<T>,
}

#[function_component]
pub fn Tabs<T: Copy + PartialEq + ToString + 'static>(properties: &TabsProperties<T>) -> Html {
    html!(
        <div class="tabs">
            <ul>
                {
                    properties.tabs.iter().map(|item: &T| {
                        html!(
                            <li class={ match properties.selected { Some(selected) => if selected == *item { "is-active" } else { "" }, None => "" } } >
                                <a onclick={
                                    let item: T = *item;
                                    let updated: Callback<T> = properties.updated.clone();
                                    Callback::from(move |_: MouseEvent| {
                                        updated.emit(item);
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
