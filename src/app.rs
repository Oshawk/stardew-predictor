use crate::dropdown::Dropdown;
use yew::prelude::*;

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
    html!(
        <Dropdown<Test> items={ vec![Test::A, Test::B] } updated={ Callback::<Test>::from(|_| {}) }/>
    )
}
