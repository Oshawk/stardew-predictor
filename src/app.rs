use yew::prelude::*;

use crate::components::configuration_form::ConfigurationForm;
use crate::configuration::Configuration;
use crate::implementations::traveling_cart::TravelingCart;

#[derive(Clone, Copy, PartialEq)]
pub enum Test {
    A,
    B,
}

impl ToString for Test {
    fn to_string(&self) -> String {
        match self {
            Test::A => "A".to_string(),
            Test::B => "B".to_string(),
        }
    }
}

#[function_component]
pub fn App() -> Html {
    let configuration: UseStateHandle<Option<Configuration>> = use_state_eq(|| None);
    let configuration_form_updated = use_callback(
        configuration.clone(),
        move |configuration_: Configuration,
              configuration: &UseStateHandle<Option<Configuration>>| {
            configuration.set(Some(configuration_));
        },
    );

    html!(
        <>
            // <Dropdown<Test> items={ vec![Test::A, Test::B] } updated={ Callback::<Option<Test>>::from(|_| {}) } />
            // <Input<i32> updated={ Callback::<Option<i32>>::from(|_| {}) } />
            // <Button updated={ Callback::<()>::from(|_| {}) } />
            // <Message colour={ MessageColour::DANGER } header="Header" body="Body" />
            // <Table header={ vec!(
            //     vec!(TableCell{value: TableValue::String(AttrValue::from("1")), rows: 1, columns: 1}, TableCell{value: TableValue::String(AttrValue::from("2")), rows: 2, columns: 1}),
            //     vec!(TableCell{value: TableValue::String(AttrValue::from("3")), rows: 1, columns: 1}, TableCell{value: TableValue::String(AttrValue::from("4")), rows: 1, columns: 1}, TableCell{value: TableValue::String(AttrValue::from("5")), rows: 1, columns: 1})
            // ) } body={ vec!(vec!()) } />
            <ConfigurationForm updated={ configuration_form_updated } />
            {
                match (*configuration).clone() {
                    Some(configuration) => html!(
                        <section class="section">
                            <h1 class="title">{ "Results" }</h1>
                            <div class="container">
                                <TravelingCart configuration={ configuration.clone() } />
                            </div>
                        </section>
                    ),
                    None => html!(),
                }
            }
        </>
    )
}
