use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DropdownProperties<T: Copy + PartialEq + ToString + 'static> {
    pub items: Vec<T>,
    pub updated: Callback<Option<T>>,
    #[prop_or(AttrValue::from("Dropdown"))]
    pub label: AttrValue,
}

#[function_component]
pub fn Dropdown<T: Copy + PartialEq + ToString + 'static>(
    properties: &DropdownProperties<T>,
) -> Html {
    let selected_updated: Callback<Event> = {
        let items: Vec<T> = properties.items.clone();
        let updated: Callback<Option<T>> = properties.updated.clone();
        Callback::from(move |event: Event| {
            let index: i32 = event
                .target_unchecked_into::<HtmlSelectElement>()
                .selected_index();
            if index > 0i32 {
                updated.emit(Some(*items.get(index as usize - 1usize).unwrap()));
            } else {
                updated.emit(None);
            };
        })
    };

    html!(
        <div class="field">
            <label class="label">{ properties.label.clone() }</label>
            <div class="control">
                <div class="select">
                    <select onchange={ selected_updated }>
                        <option disabled=true hidden=true selected=true>{ properties.label.clone() }</option>
                        {
                            properties.items.iter().map(|item: &T| {
                                html!(<option>{ item.to_string() }</option>)
                            }).collect::<Html>()
                        }
                    </select>
                </div>
            </div>
        </div>
    )
}
