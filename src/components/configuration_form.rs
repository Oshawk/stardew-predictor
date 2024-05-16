use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use yew::prelude::*;

use crate::components::button::{Button, ButtonColour};
use crate::components::date::Date;
use crate::components::dropdown::Dropdown;
use crate::components::input::Input;
use crate::components::message::{Message, MessageColour};
use crate::configuration::{Configuration, Platform};

#[derive(Clone, Copy, PartialEq)]
pub enum YesNo {
    Yes,
    No,
}

impl Display for YesNo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                YesNo::Yes => "Yes",
                YesNo::No => "No",
            }
        )
    }
}

#[derive(Properties, PartialEq)]
pub struct ConfigurationFormProperties {
    pub updated: Callback<Configuration>,
}

#[function_component]
pub fn ConfigurationForm(properties: &ConfigurationFormProperties) -> Html {
    let platform: Rc<RefCell<Option<Platform>>> = use_mut_ref(|| None);
    let platform_updated: Callback<Option<Platform>> = use_callback(
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

    let show_optional: UseStateHandle<bool> = use_state_eq(|| false);
    let show_optional_updated: Callback<()> = use_callback(
        show_optional.clone(),
        move |_: (), show_optional: &UseStateHandle<bool>| {
            show_optional.set(!**show_optional);
        },
    );

    let date: Rc<RefCell<Option<i32>>> = use_mut_ref(|| None);
    let date_updated: Callback<Option<i32>> = use_callback(
        date.clone(),
        move |date_: Option<i32>, date: &Rc<RefCell<Option<i32>>>| {
            *date.borrow_mut() = date_;
        },
    );

    let geodes_cracked: Rc<RefCell<Option<u16>>> = use_mut_ref(|| None);
    let geodes_cracked_updated: Callback<Option<u16>> = use_callback(
        geodes_cracked.clone(),
        move |geodes_cracked_: Option<u16>, geodes_cracked: &Rc<RefCell<Option<u16>>>| {
            *geodes_cracked.borrow_mut() = geodes_cracked_;
        },
    );

    let mine_level: Rc<RefCell<Option<u8>>> = use_mut_ref(|| None);
    let mine_level_updated: Callback<Option<u8>> = use_callback(
        mine_level.clone(),
        move |mine_level_: Option<u8>, mine_level: &Rc<RefCell<Option<u8>>>| {
            *mine_level.borrow_mut() = mine_level_;
        },
    );

    let qis_crop: Rc<RefCell<Option<bool>>> = use_mut_ref(|| None);
    let qis_crop_updated: Callback<Option<YesNo>> = use_callback(
        qis_crop.clone(),
        move |qis_crop_: Option<YesNo>, qis_crop: &Rc<RefCell<Option<bool>>>| {
            *qis_crop.borrow_mut() = match qis_crop_ {
                Some(YesNo::Yes) => Some(true),
                Some(YesNo::No) => Some(false),
                None => None,
            };
        },
    );

    let golden_helmet: Rc<RefCell<Option<bool>>> = use_mut_ref(|| None);
    let golden_helmet_updated: Callback<Option<YesNo>> = use_callback(
        golden_helmet.clone(),
        move |golden_helmet_: Option<YesNo>, golden_helmet: &Rc<RefCell<Option<bool>>>| {
            *golden_helmet.borrow_mut() = match golden_helmet_ {
                Some(YesNo::Yes) => Some(true),
                Some(YesNo::No) => Some(false),
                None => None,
            };
        },
    );

    let message: UseStateHandle<Option<String>> = use_state_eq(|| None);

    let go_updated: Callback<()> = use_callback(
        (
            platform.clone(),
            seed.clone(),
            date.clone(),
            geodes_cracked.clone(),
            mine_level.clone(),
            qis_crop.clone(),
            golden_helmet.clone(),
            message.clone(),
            properties.updated.clone(),
        ),
        move |_: (),
              (
            platform,
            seed,
            date,
            geodes_cracked,
            mine_level,
            qis_crop,
            golden_helmet,
            message,
            updated,
        ): &(
            Rc<RefCell<Option<Platform>>>,
            Rc<RefCell<Option<i32>>>,
            Rc<RefCell<Option<i32>>>,
            Rc<RefCell<Option<u16>>>,
            Rc<RefCell<Option<u8>>>,
            Rc<RefCell<Option<bool>>>,
            Rc<RefCell<Option<bool>>>,
            UseStateHandle<Option<String>>,
            Callback<Configuration>,
        )| {
            match (*platform.borrow(), *seed.borrow()) {
                (Some(platform), Some(seed)) => {
                    updated.emit(Configuration {
                        platform,
                        seed,
                        date: *date.borrow(),
                        geodes_cracked: *geodes_cracked.borrow(),
                        mine_level: *mine_level.borrow(),
                        qis_crop: *qis_crop.borrow(),
                        golden_helmet: *golden_helmet.borrow(),
                    });
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
                        <Dropdown<Platform> updated={ platform_updated } items={ vec![Platform::PC, Platform::Switch] } label="Platform" />
                        <Input<i32> updated={ seed_updated } label="Seed" />
                        <div class={ if *show_optional { "mb-3" } else { "is-hidden" } }>
                            <Date updated={ date_updated } label="Date (optional)" />
                            <Input<u16> updated={ geodes_cracked_updated } label="Geodes cracked (optional)" />
                            <Input<u8> updated={ mine_level_updated } label="Deepest mine level (optional)" />
                            <Dropdown<YesNo> updated={ qis_crop_updated } items={ vec![YesNo::Yes, YesNo::No] } label="Qi's crop quest (optional)" />
                            <Dropdown<YesNo> updated={ golden_helmet_updated } items={ vec![YesNo::Yes, YesNo::No] } label="Golden helmet received (optional)" />
                        </div>
                        <Button updated={ show_optional_updated } colour={ ButtonColour::Default } label={ format!("{} optional parameters", if *show_optional { "Hide" } else { "Show" }) } />
                        <Button updated={ go_updated } colour={ ButtonColour::Primary } label="Go" />
                    </div>
                    <div class="column"></div>
                </div>
                <Message colour={ MessageColour::Danger } body={ (*message).clone() } />
            </div>
        </section>
    )
}
