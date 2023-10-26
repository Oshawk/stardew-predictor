use crate::elements::button::Button;
use crate::elements::dropdown::Dropdown;
use crate::elements::input::Input;
use crate::elements::message::{Message, MessageColour};
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
            <Button updated={ Callback::<()>::from(|_| {}) }/>
            <Message colour={ MessageColour::DANGER } header="Header" body="Body"/>
        </>
    )
}
