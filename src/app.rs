use crate::dropdown::Dropdown;
use crate::input::Input;
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
        <>
            <Dropdown<Test> items={ vec![Test::A, Test::B] } updated={ Callback::<Option<Test>>::from(|_| {}) }/>
            <Input<i32> updated={ Callback::<Option<i32>>::from(|_| {}) }/>
        </>
    )
}
