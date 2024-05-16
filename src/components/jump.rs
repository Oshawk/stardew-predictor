use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct JumpProperties<T: Copy + FromStr + PartialEq + ToString + 'static> {
    pub updated: Callback<T>,
    #[prop_or(AttrValue::from("Jump"))]
    pub label: AttrValue,
}

#[function_component]
pub fn Jump<T: Copy + FromStr + PartialEq + ToString + 'static>(
    properties: &JumpProperties<T>,
) -> Html {
    let force_update: UseForceUpdateHandle = use_force_update();

    let value: Rc<RefCell<Option<T>>> = use_mut_ref(|| None);
    let value_updated: Callback<InputEvent> = {
        let force_update: UseForceUpdateHandle = force_update.clone();
        let value: Rc<RefCell<Option<T>>> = value.clone();
        Callback::from(move |event: InputEvent| {
            let value_string: String = event.target_unchecked_into::<HtmlInputElement>().value();
            match value_string.parse::<T>() {
                Ok(value_) => {
                    *value.borrow_mut() = Some(value_);
                }
                Err(_) => {
                    *value.borrow_mut() = None;
                    if value_string != "-" {
                        force_update.force_update();
                    }
                }
            }
        })
    };

    let jump_updated: Callback<MouseEvent> = {
        let updated: Callback<T> = properties.updated.clone();
        Callback::from(move |_: MouseEvent| match *value.borrow() {
            Some(value) => updated.emit(value),
            None => {}
        })
    };

    html!(
        <div class="field has-addons">
            <div class="control is-expanded">
                <input class="input" oninput={ value_updated } placeholder={ properties.label.clone() } type="text" value="" />
            </div>
            <div class="control">
                <button class="button is-primary" onclick={ jump_updated }>{ "Jump" }</button>
            </div>
        </div>
    )
}
