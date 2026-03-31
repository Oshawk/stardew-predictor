use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DropdownProperties<T: Copy + PartialEq + ToString + 'static> {
    pub updated: Callback<Option<T>>,
    pub items: Vec<T>,
    #[prop_or(AttrValue::from("Dropdown"))]
    pub label: AttrValue,
}

#[component]
pub fn Dropdown<T: Copy + PartialEq + ToString + 'static>(
    properties: &DropdownProperties<T>,
) -> Html {
    let selected_updated = {
        let items = properties.items.clone();
        let updated = properties.updated.clone();
        Callback::from(move |event: Event| {
            let index = event
                .target_unchecked_into::<HtmlSelectElement>()
                .selected_index();
            if index > 0 {
                updated.emit(Some(*items.get(index as usize - 1).unwrap()));
            } else {
                updated.emit(None);
            }
        })
    };

    html!(
        <div class="field">
            <label class="label">{ properties.label.clone() }</label>
            <div class="control">
                <div class="select">
                    <select onchange={ selected_updated }>
                        <option disabled=true hidden=true selected=true>{ properties.label.clone() }</option>
                        { for properties.items.iter().map(|item| html!(<option>{ item.to_string() }</option>)) }
                    </select>
                </div>
            </div>
        </div>
    )
}
