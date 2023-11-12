use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProperties {
    pub updated: Callback<()>,
    #[prop_or(AttrValue::from("Button"))]
    pub label: AttrValue,
}

#[function_component]
pub fn Button(properties: &ButtonProperties) -> Html {
    let button_updated: Callback<MouseEvent> = {
        let updated: Callback<()> = properties.updated.clone();
        Callback::from(move |_: MouseEvent| {
            updated.emit(());
        })
    };

    html!(
        <div class="field">
            <div class="control">
                <button class="button" onclick={ button_updated }>{ properties.label.clone() }</button>
            </div>
        </div>
    )
}
