use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TabsProperties<T: Copy + PartialEq + ToString + 'static> {
    pub tabs: Vec<T>,
    pub selected: Option<T>,
    pub updated: Callback<T>,
}

#[component]
pub fn Tabs<T: Copy + PartialEq + ToString + 'static>(properties: &TabsProperties<T>) -> Html {
    html!(
        <div class="tabs">
            <ul>
                { for properties.tabs.iter().map(|item| {
                    let is_active = properties.selected == Some(*item);
                    let item = *item;
                    let updated = properties.updated.clone();
                    html!(
                        <li class={ if is_active { "is-active" } else { "" } }>
                            <a onclick={ Callback::from(move |_: MouseEvent| updated.emit(item)) }>{ item.to_string() }</a>
                        </li>
                    )
                })}
            </ul>
        </div>
    )
}
