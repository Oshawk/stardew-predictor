use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

use crate::configuration::{Configuration, Platform};
use crate::elements::button::Button;
use crate::elements::dropdown::Dropdown;
use crate::elements::input::Input;
use crate::elements::message::{Message, MessageColour};

#[derive(Properties, PartialEq)]
pub struct ConfigurationFormProperties {
    pub updated: Callback<Configuration>,
}

#[function_component]
pub fn ConfigurationForm(properties: &ConfigurationFormProperties) -> Html {
    let platform: Rc<RefCell<Option<Platform>>> = use_mut_ref(|| None);
    let platform_updated: Callback<Option<Platform>>= use_callback(
        platform.clone(),
        move |platform_: Option<Platform>, platform: &Rc<RefCell<Option<Platform>>>| {
            *platform.borrow_mut() = platform_;
        },
    );

    let seed: Rc<RefCell<Option<i32>>> = use_mut_ref(|| None);
    let seed_updated: Callback<Option<i32>> = use_callback(
        seed.clone(),
        move |seed_: Option<i32>, seed: &Rc<RefCell<Option<i32>>>| {
            *seed.borrow_mut() = seed_;
        },
    );

    let message: UseStateHandle<Option<String>> = use_state_eq(|| None);
    let button_updated: Callback<()> = use_callback(
        (
            platform.clone(),
            seed.clone(),
            message.clone(),
            properties.updated.clone(),
        ),
        move |_: (), (platform, seed, message, updated): &(
            Rc<RefCell<Option<Platform>>>,
            Rc<RefCell<Option<i32>>>,
            UseStateHandle<Option<String>>,
            Callback<Configuration>,
        )| {
            match (*platform.borrow(), *seed.borrow()) {
                (Some(platform), Some(seed)) => {
                    updated.emit(Configuration { platform, seed, date: None });
                    message.set(None);
                }
                _ => {
                    message.set(Some("Platform and seed must be set.".to_string()));
                }
            }
        },
    );

    html!(
        <section class="section">
            <h1 class="title">{ "Configuration" }</h1>
            <div class="container">
                    <div class="columns">
                        <div class="column">
                            <Dropdown<Platform> items={ vec![Platform::PC, Platform::Switch] } updated={ platform_updated } label="Platform" />
                            <Input<i32> updated={ seed_updated } label="Seed" />
                            <Button updated={ button_updated } label="Go" />
                        </div>
                        <div class="column"></div>
                    </div>
                    <Message colour={ MessageColour::Danger } body={ (*message).clone() } />
            </div>
        </section>
    )
}
