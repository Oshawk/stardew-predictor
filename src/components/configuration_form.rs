use std::fmt::{Display, Formatter};
use yew::prelude::*;

use crate::components::button::{Button, ButtonColour};
use crate::components::date::DatePicker;
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

#[component]
pub fn ConfigurationForm(properties: &ConfigurationFormProperties) -> Html {
    let platform = use_state(|| None::<Platform>);
    let seed = use_state(|| None::<i32>);
    let show_optional = use_state(|| false);
    let date = use_state(|| None::<i32>);
    let geodes_cracked = use_state(|| None::<u16>);
    let mine_level = use_state(|| None::<u8>);
    let qis_crop = use_state(|| None::<bool>);
    let golden_helmet = use_state(|| None::<bool>);
    let message = use_state(|| None::<String>);

    let platform_updated = {
        let platform = platform.clone();
        Callback::from(move |value: Option<Platform>| platform.set(value))
    };

    let seed_updated = {
        let seed = seed.clone();
        Callback::from(move |value: Option<i32>| seed.set(value))
    };

    let show_optional_updated = {
        let show_optional = show_optional.clone();
        Callback::from(move |_: ()| show_optional.set(!*show_optional))
    };

    let date_updated = {
        let date = date.clone();
        Callback::from(move |value: Option<i32>| date.set(value))
    };

    let geodes_cracked_updated = {
        let geodes_cracked = geodes_cracked.clone();
        Callback::from(move |value: Option<u16>| geodes_cracked.set(value))
    };

    let mine_level_updated = {
        let mine_level = mine_level.clone();
        Callback::from(move |value: Option<u8>| mine_level.set(value))
    };

    let qis_crop_updated = {
        let qis_crop = qis_crop.clone();
        Callback::from(move |value: Option<YesNo>| {
            qis_crop.set(match value {
                Some(YesNo::Yes) => Some(true),
                Some(YesNo::No) => Some(false),
                None => None,
            });
        })
    };

    let golden_helmet_updated = {
        let golden_helmet = golden_helmet.clone();
        Callback::from(move |value: Option<YesNo>| {
            golden_helmet.set(match value {
                Some(YesNo::Yes) => Some(true),
                Some(YesNo::No) => Some(false),
                None => None,
            });
        })
    };

    let go_updated = {
        let platform = platform.clone();
        let seed = seed.clone();
        let date = date.clone();
        let geodes_cracked = geodes_cracked.clone();
        let mine_level = mine_level.clone();
        let qis_crop = qis_crop.clone();
        let golden_helmet = golden_helmet.clone();
        let message = message.clone();
        let updated = properties.updated.clone();
        Callback::from(move |_: ()| {
            match (*platform, *seed) {
                (Some(platform), Some(seed)) => {
                    updated.emit(Configuration {
                        platform,
                        seed,
                        date: *date,
                        geodes_cracked: *geodes_cracked,
                        mine_level: *mine_level,
                        qis_crop: *qis_crop,
                        golden_helmet: *golden_helmet,
                    });
                    message.set(None);
                }
                _ => {
                    message.set(Some("Platform and seed must be set.".to_string()));
                }
            }
        })
    };

    html!(
        <section class="section">
            <h1 class="title">{ "Configuration" }</h1>
            <div class="container">
                <div class="columns">
                    <div class="column">
                        <Dropdown<Platform> updated={ platform_updated } items={ vec![Platform::PC, Platform::Switch] } label="Platform" />
                        <Input<i32> updated={ seed_updated } label="Seed" />
                        <div class={ if *show_optional { "mb-3" } else { "is-hidden" } }>
                            <DatePicker updated={ date_updated } label="Date (optional)" />
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
