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
    let platform_updated: Callback<Option<Platform>> = {
        let platform: Rc<RefCell<Option<Platform>>> = platform.clone();
        Callback::from(move |platform_: Option<Platform>| {
            *platform.borrow_mut() = platform_;
            // platform.replace(platform_);
        })
    };

    let seed: Rc<RefCell<Option<i32>>> = use_mut_ref(|| None);
    let seed_updated: Callback<Option<i32>> = {
        let seed: Rc<RefCell<Option<i32>>> = seed.clone();
        Callback::from(move |seed_: Option<i32>| {
            *seed.borrow_mut() = seed_;
            // seed.replace(seed_);
        })
    };

    let message: UseStateHandle<Option<String>> = use_state_eq(|| None);
    let button_updated: Callback<()> = {
        let platform: Rc<RefCell<Option<Platform>>> = platform.clone();
        let seed: Rc<RefCell<Option<i32>>> = seed.clone();
        let message: UseStateHandle<Option<String>> = message.clone();
        let updated: Callback<Configuration> = properties.updated.clone();
        Callback::from(move |_: ()| {
            match (*platform.borrow(), *seed.borrow()) {
                (Some(platform), Some(seed)) => {
                    updated.emit(Configuration { platform, seed });
                    message.set(None);
                }
                _ => {
                    message.set(Some("Platform and seed must be set.".to_string()));
                }
            }
        })
    };

    html!(
        <div class="section">
            <section class="section">
                <h1 class="title">{ "Configuration" }</h1>
                <div class="columns">
                    <div class="column">
                        <Dropdown<Platform> items={ vec![Platform::SWITCH] } updated={ platform_updated } label="Platform"/>
                        <Input<i32> updated={ seed_updated } label="Seed"/>
                        <Button updated={ button_updated } label="Go"/>
                    </div>
                    <div class="column"></div>
                </div>
                <Message colour={ MessageColour::DANGER } body={ (*message).clone() }/>
            </section>
        </div>
    )
}
