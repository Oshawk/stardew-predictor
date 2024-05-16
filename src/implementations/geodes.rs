use crate::codegen::{ObjectInformation, ObjectInformationExtra, HATS, OBJECT_INFORMATION};
use anyhow::{Context, Result};
use std::cmp::max;
use yew::prelude::*;

use crate::components::filter::Filter;
use crate::components::jump::Jump;
use crate::components::message::{Message, MessageColour};
use crate::components::navigation::{Navigation, NavigationDirection};
use crate::components::table::{Table, TableAlign, TableCell, TableValue};
use crate::configuration::Configuration;
use crate::implementations::util::{get_prng, Item};
use crate::prng::Prng;

const NON_FILTER_ITERATIONS: u16 = 28u16;
const FILTER_ITERATIONS: u16 = 1120u16;
const FILTER_GEODES: u8 = 8u8;

struct Geode {
    id: u16,
    item: Item,
    quantity: u8,
}

fn get_geodes(
    configuration: &Configuration,
    geodes_cracked: i32,
    filter: &String,
) -> Result<Vec<Vec<TableCell>>> {
    let iterations: u16 = if filter.is_empty() {
        NON_FILTER_ITERATIONS
    } else {
        FILTER_ITERATIONS
    };
    let mut geodes_generated: u8 = 0u8;
    let mut table: Vec<Vec<TableCell>> = Vec::new();
    for iteration in 0u16..iterations {
        let geodes_cracked: i32 = geodes_cracked + iteration as i32;

        let mut geodes: [Option<Geode>; 6usize] = Default::default();

        for i in 0usize..geodes.len() {
            let mut prng: Box<dyn Prng> = get_prng(
                configuration.platform,
                configuration.seed / 2i32 + geodes_cracked,
            )?;
            for _ in 0i32..prng.gen_range(1i32..10i32)? {
                prng.gen_float()?;
            }
            for _ in 0i32..prng.gen_range(1i32..10i32)? {
                prng.gen_float()?;
            }

            geodes[i] = Some(
                if prng.gen_float()? <= 0.1f64 && configuration.qis_crop.unwrap_or(false) {
                    Geode {
                        id: 890u16,
                        item: Item::ObjectInformation(
                            OBJECT_INFORMATION
                                .get(&890u16)
                                .context("Error getting object information.")?,
                        ),
                        quantity: if prng.gen_float()? < 0.25f64 {
                            5u8
                        } else {
                            1u8
                        },
                    }
                } else {
                    match i {
                        0usize..=4usize => {
                            if i == 4usize || prng.gen_float()? >= 0.5f64 {
                                // Artefact trove
                                let object_information: &ObjectInformation = OBJECT_INFORMATION
                                    .get(&match i {
                                        0usize => 535u16,
                                        1usize => 536u16,
                                        2usize => 537u16,
                                        3usize => 749u16,
                                        4usize => 275u16,
                                        _ => panic!(),
                                    })
                                    .unwrap();

                                let treasure: &[u16] = match object_information.extra {
                                    ObjectInformationExtra::Treasure(treasure) => treasure,
                                    _ => panic!(),
                                };

                                let mut id: u16 =
                                    treasure[prng.gen_range(0i32..treasure.len() as i32)? as usize];

                                if i == 3usize
                                    && prng.gen_float()? < 0.008f64
                                    && geodes_cracked > 15i32
                                {
                                    // Omni geode
                                    id = 74u16;
                                }

                                Geode {
                                    id,
                                    item: Item::ObjectInformation(
                                        OBJECT_INFORMATION
                                            .get(&id)
                                            .context("Error getting object information.")?,
                                    ),
                                    quantity: 1u8,
                                }
                            } else {
                                let mut quantity: u8 =
                                    (prng.gen_range(0i32..3i32)? as u8) * 2u8 + 1u8;
                                if prng.gen_float()? < 0.1f64 {
                                    quantity = 10u8;
                                }
                                if prng.gen_float()? < 0.01f64 {
                                    quantity = 20u8;
                                }

                                let (id, quantity): (u16, u8) = if prng.gen_float()? < 0.5f64 {
                                    match prng.gen_range(0i32..4i32)? {
                                        0i32 | 1i32 => (390u16, quantity),
                                        2i32 => (330u16, 1u8),
                                        3i32 => match i {
                                            0usize => (86u16, 1u8), // Geode
                                            1usize => (84u16, 1u8), // Frozen geode
                                            2usize => (82u16, 1u8), // Magma geode
                                            3usize => (
                                                82u16 + (prng.gen_range(0i32..3i32)? as u16) * 2u16,
                                                1u8,
                                            ), // Omni geode
                                            _ => panic!(),
                                        },
                                        _ => panic!(),
                                    }
                                } else {
                                    match i {
                                        0usize => match prng.gen_range(0i32..3i32)? {
                                            // Geode
                                            0i32 => (378u16, quantity),
                                            1i32 => (
                                                if configuration.mine_level.unwrap_or(120u8) > 25u8
                                                {
                                                    380u16
                                                } else {
                                                    378u16
                                                },
                                                quantity,
                                            ),
                                            2i32 => (382u16, quantity),
                                            _ => panic!(),
                                        },
                                        1usize => match prng.gen_range(0i32..4i32)? {
                                            // Frozen geode
                                            0i32 => (378u16, quantity),
                                            1i32 => (380u16, quantity),
                                            2i32 => (382u16, quantity),
                                            3i32 => (
                                                if configuration.mine_level.unwrap_or(120u8) > 75u8
                                                {
                                                    384u16
                                                } else {
                                                    380u16
                                                },
                                                quantity,
                                            ),
                                            _ => panic!(),
                                        },
                                        2usize | 3usize => match prng.gen_range(0i32..5i32)? {
                                            // Magma or omni geode
                                            0i32 => (378u16, quantity),
                                            1i32 => (380u16, quantity),
                                            2i32 => (382u16, quantity),
                                            3i32 => (384u16, quantity),
                                            4i32 => (386u16, quantity / 2u8 + 1u8),
                                            _ => panic!(),
                                        },
                                        _ => panic!(),
                                    }
                                };

                                Geode {
                                    id,
                                    item: Item::ObjectInformation(
                                        OBJECT_INFORMATION
                                            .get(&id)
                                            .context("Error getting object information.")?,
                                    ),
                                    quantity,
                                }
                            }
                        }
                        5usize => {
                            // Golden coconut
                            if prng.gen_float()? < 0.05f64
                                && !configuration.golden_helmet.unwrap_or(true)
                            {
                                Geode {
                                    id: 75u16,
                                    item: Item::Hats(
                                        HATS.get(&75u16).context("Error getting hats.")?,
                                    ),
                                    quantity: 1u8,
                                }
                            } else {
                                let (id, quantity): (u16, u8) = match prng.gen_range(0i32..7i32)? {
                                    0i32 => (69u16, 1u8),
                                    1i32 => (835u16, 1u8),
                                    2i32 => (833u16, 5u8),
                                    3i32 => (831u16, 5u8),
                                    4i32 => (820u16, 1u8),
                                    5i32 => (292u16, 1u8),
                                    6i32 => (386u16, 5u8),
                                    _ => panic!(),
                                };

                                Geode {
                                    id,
                                    item: Item::ObjectInformation(
                                        OBJECT_INFORMATION
                                            .get(&id)
                                            .context("Error getting object information.")?,
                                    ),
                                    quantity,
                                }
                            }
                        }
                        _ => panic!(),
                    }
                },
            );
        }

        // Filter after generating everything.
        for i in 0usize..geodes.len() {
            match &geodes[i] {
                Some(geode) => {
                    if !geode.item.name(geode.id).to_lowercase().contains(filter) {
                        geodes[i] = None;
                    }
                }
                None => {}
            }
        }

        if geodes.iter().any(|geode| geode.is_some()) {
            let mut row: Vec<TableCell> = vec![TableCell {
                value: TableValue::String(AttrValue::from(format!("{}", geodes_cracked))),
                align: TableAlign::MiddleCenter,
                rows: 1u8,
                columns: 1u8,
            }];

            for geode in &geodes {
                row.push(TableCell {
                    value: match geode {
                        Some(geode) => geode.item.sprite(geode.id),
                        None => TableValue::None,
                    },
                    align: TableAlign::MiddleCenter,
                    rows: 1u8,
                    columns: 1u8,
                });

                row.push(TableCell {
                    value: match geode {
                        Some(geode) => {
                            TableValue::String(AttrValue::from(geode.item.name(geode.id)))
                        }
                        None => TableValue::None,
                    },
                    align: TableAlign::MiddleLeft,
                    rows: 1u8,
                    columns: 1u8,
                });

                row.push(TableCell {
                    value: match geode {
                        Some(geode) => {
                            TableValue::String(AttrValue::from(format!("x{}", geode.quantity)))
                        }
                        None => TableValue::None,
                    },
                    align: TableAlign::MiddleLeft,
                    rows: 1u8,
                    columns: 1u8,
                });
            }

            table.push(row);

            geodes_generated += 1u8;
            if !filter.is_empty() && geodes_generated >= FILTER_GEODES {
                break;
            }
        }
    }

    Ok(table)
}

fn get_messages(configuration: &Configuration) -> Html {
    let mut warnings: Vec<&str> = Vec::<&str>::new();

    if configuration.geodes_cracked.is_none() {
        warnings.push(
            "Use the geodes cracked optional configuration parameter to always display from there.",
        );
    }

    if configuration.mine_level.is_none() {
        warnings.push("The deepest mine level you have reached can be specified using the optional configuration parameter. We assume 120 by default.");
    }

    if configuration.qis_crop.is_none() {
        warnings.push("Whether you are on a Qi's crop quest can be specified using the optional configuration parameter. We assume not by default.");
    }

    if configuration.golden_helmet.is_none() {
        warnings.push("Whether you have received the golden helmet can be specified using the optional configuration parameter. We assume so by default.");
    }

    html!(
        <>
            <Message colour={ MessageColour::Info } body="Items from geodes." />
            {
                warnings.into_iter().map(|warning|{
                    html!{
                        <Message colour={ MessageColour::Warning } body={ warning } />
                    }
                }).collect::<Html>()
            }
        </>
    )
}

pub fn geodes_table_header() -> Vec<Vec<TableCell>> {
    vec![
        vec![
            TableCell {
                value: TableValue::String(AttrValue::from("Cracked")),
                align: TableAlign::MiddleLeft,
                rows: 2u8,
                columns: 1u8,
            },
            TableCell {
                value: TableValue::String(AttrValue::from("Geode")),
                align: TableAlign::MiddleLeft,
                rows: 1u8,
                columns: 3u8,
            },
            TableCell {
                value: TableValue::String(AttrValue::from("Frozen Geode")),
                align: TableAlign::MiddleLeft,
                rows: 1u8,
                columns: 3u8,
            },
            TableCell {
                value: TableValue::String(AttrValue::from("Magma Geode")),
                align: TableAlign::MiddleLeft,
                rows: 1u8,
                columns: 3u8,
            },
            TableCell {
                value: TableValue::String(AttrValue::from("Omni Geode")),
                align: TableAlign::MiddleLeft,
                rows: 1u8,
                columns: 3u8,
            },
            TableCell {
                value: TableValue::String(AttrValue::from("Artifact Trove")),
                align: TableAlign::MiddleLeft,
                rows: 1u8,
                columns: 3u8,
            },
            TableCell {
                value: TableValue::String(AttrValue::from("Golden Coconut")),
                align: TableAlign::MiddleLeft,
                rows: 1u8,
                columns: 3u8,
            },
        ],
        {
            let mut second_row: Vec<TableCell> = Vec::new();
            for _ in 0u8..6u8 {
                second_row.push(TableCell {
                    value: TableValue::String(AttrValue::from("Item")),
                    align: TableAlign::MiddleLeft,
                    rows: 1u8,
                    columns: 2u8,
                });
                second_row.push(TableCell {
                    value: TableValue::String(AttrValue::from("Quantity")),
                    align: TableAlign::MiddleLeft,
                    rows: 1u8,
                    columns: 1u8,
                });
            }
            second_row
        },
    ]
}

#[derive(Properties, PartialEq)]
pub struct GeodesProperties {
    pub configuration: Configuration,
}

#[function_component]
pub fn Geodes(properties: &GeodesProperties) -> Html {
    let geodes_cracked: UseStateHandle<i32> =
        use_state_eq(|| properties.configuration.geodes_cracked.unwrap_or(0u16) as i32);
    let filter: UseStateHandle<String> = use_state_eq(|| "".to_string());

    let jump_updated: Callback<u16> = use_callback(
        geodes_cracked.clone(),
        |geodes_cracked_: u16, geodes_cracked: &UseStateHandle<i32>| {
            geodes_cracked.set(geodes_cracked_ as i32);
        },
    );

    let filter_updated: Callback<String> = use_callback(
        filter.clone(),
        |filter_: String, filter: &UseStateHandle<String>| {
            filter.set(filter_);
        },
    );

    let navigation_updated: Callback<NavigationDirection> = use_callback(
        geodes_cracked.clone(),
        move |direction: NavigationDirection, geodes_cracked: &UseStateHandle<i32>| {
            match direction {
                NavigationDirection::Backward => {
                    geodes_cracked.set(max(**geodes_cracked - NON_FILTER_ITERATIONS as i32, 0i32));
                }
                NavigationDirection::Forward => {
                    geodes_cracked.set(**geodes_cracked + NON_FILTER_ITERATIONS as i32);
                }
            };
        },
    );

    match get_geodes(&properties.configuration, *geodes_cracked, &*filter) {
        Ok(table) => {
            html!(
                <>
                    { get_messages(&properties.configuration) }
                    <div class="columns">
                        <div class="column">
                            <Jump<u16> updated={ jump_updated } />
                        </div>
                        <div class="column">
                            <Filter updated={ filter_updated } />
                        </div>
                    </div>
                    <Navigation updated={ navigation_updated.clone() } disabled={ !filter.is_empty() } />
                    <Table header={ geodes_table_header() } body={ table } />
                    <Navigation updated={ navigation_updated } disabled={ !filter.is_empty() } />
                </>
            )
        }
        Err(error) => {
            html! {
                <Message colour={ MessageColour::Danger } body={ error.to_string() } />
            }
        }
    }
}
