use std::cmp::max;
use std::collections::HashSet;

use anyhow::{Context, Result};
use yew::prelude::*;

use crate::codegen::{
    ObjectInformation, BIG_CRAFTABLES_INFORMATION, FURNITURE, OBJECT_INFORMATION,
    OBJECT_INFORMATION_OFF_LIMIT,
};
use crate::components::message::{Message, MessageColour};
use crate::components::stock_table::{StockTable, StockTableTrait};
use crate::components::table::TableCell;
use crate::configuration::{Configuration, Platform};
use crate::implementations::util::{
    day_number, get_prng, get_random_furniture, season_number, stock_items_rows, Item, StockItem,
};
use crate::prng::Prng;

const NON_FILTER_ITERATIONS: u16 = 28u16;
const FILTER_ITERATIONS: u16 = 1120u16;
const FILTER_DAYS: u8 = 8u8;

macro_rules! second_check {
    ($object_information:ident) => {
        if !$object_information.type_and_category.contains("-")
            || $object_information.price <= 0
            || $object_information.type_and_category.contains("-13")
            || $object_information.type_and_category == "Quest"
            || $object_information.name == "Weeds"
            || $object_information.type_and_category.contains("Minerals")
            || $object_information.type_and_category.contains("Arch")
        {
            continue;
        }
    };
}

macro_rules! second_rng {
    ($prng:ident, $constant_multiplier:ident, $variable_multiplier:ident, $quantity_decider:ident) => {
        $constant_multiplier = $prng.gen_range(1i32..11i32)? as u32;
        $variable_multiplier = $prng.gen_range(3i32..6i32)? as u32;
        $quantity_decider = $prng.gen_float()?;
    };
}

macro_rules! gen_furniture_id {
    ($prng:ident, $furniture_id:ident) => {
        $furniture_id = get_random_furniture(&mut $prng, 0u16, 1613u16)?;
    };
}

macro_rules! gen_furniture_price {
    ($prng:ident, $furniture_price:ident) => {
        $furniture_price = $prng.gen_range(1i32..11i32)? as u32 * 250u32;
    };
}

pub struct TravelingCartImpl {}

impl StockTableTrait for TravelingCartImpl {
    fn get_stock(
        configuration: &Configuration,
        date: i32,
        filter: &String,
    ) -> Result<Vec<Vec<TableCell>>> {
        let iterations: u16 = if filter.is_empty() {
            NON_FILTER_ITERATIONS
        } else {
            FILTER_ITERATIONS
        };
        let mut days_generated: u8 = 0u8;
        let mut table: Vec<Vec<TableCell>> = Vec::new();
        for iteration in 0u16..iterations {
            let date: i32 = date + iteration as i32;
            match (
                day_number(date) % 7u8,
                day_number(date),
                season_number(date),
            ) {
                // Dates start at 1, hence the subtraction.
                (4u8 | 6u8, _, _) => {}            // Friday or sunday.
                (_, 14u8 | 15u8 | 16u8, 3u8) => {} // Night market.
                (_, _, _) => continue,
            }

            let mut prng: Box<dyn Prng> =
                get_prng(configuration.platform, configuration.seed + date)?;

            // TODO: Year one completable.

            let mut stock_items: Vec<StockItem> = Vec::<StockItem>::new();

            let mut used_indexes: HashSet<u16> = HashSet::<u16>::new();
            for _ in 0u8..10u8 {
                let mut id: u16 = prng.gen_range(2i32..790i32)? as u16;
                stock_items.push(loop {
                    id += 1u16;
                    id %= 790u16;

                    if !OBJECT_INFORMATION.contains_key(&id)
                        || OBJECT_INFORMATION_OFF_LIMIT.contains(&id)
                    {
                        continue;
                    }

                    let object_information: &ObjectInformation = OBJECT_INFORMATION
                        .get(&id)
                        .context("Error getting object information.")?;

                    // PC does the second check before the second RNG generation, Switch does the reverse.
                    let constant_multiplier: u32;
                    let variable_multiplier: u32;
                    let quantity_decider: f64;
                    match configuration.platform {
                        Platform::PC => {
                            second_check!(object_information);
                            second_rng!(
                                prng,
                                constant_multiplier,
                                variable_multiplier,
                                quantity_decider
                            );
                        }
                        Platform::Switch => {
                            second_rng!(
                                prng,
                                constant_multiplier,
                                variable_multiplier,
                                quantity_decider
                            );
                            second_check!(object_information);
                        }
                    }

                    if !used_indexes.insert(id) {
                        continue;
                    }

                    break StockItem {
                        id,
                        item: Item::ObjectInformation(object_information),
                        price: max(
                            100u32 * constant_multiplier,
                            object_information.price * variable_multiplier,
                        ),
                        quantity: if quantity_decider < 0.1f64 { 5u8 } else { 1u8 },
                    };
                });
            }

            let furniture_id: u16;
            let furniture_price: u32;
            match configuration.platform {
                Platform::PC => {
                    gen_furniture_id!(prng, furniture_id);
                    gen_furniture_price!(prng, furniture_price);
                }
                Platform::Switch => {
                    gen_furniture_price!(prng, furniture_price);
                    gen_furniture_id!(prng, furniture_id);
                }
            }

            stock_items.push(StockItem {
                id: furniture_id,
                item: Item::Furniture(
                    FURNITURE
                        .get(&furniture_id)
                        .context("Error getting furniture.")?,
                ),
                price: furniture_price,
                quantity: 1u8,
            });

            if season_number(date) < 2 {
                stock_items.push(StockItem {
                    id: 347u16,
                    item: Item::ObjectInformation(
                        OBJECT_INFORMATION
                            .get(&347u16)
                            .context("Error getting object information.")?,
                    ),
                    price: 1000u32,
                    quantity: if prng.gen_float()? < 0.1f64 { 5u8 } else { 1u8 },
                });
            } else if prng.gen_float()? < 0.4f64 {
                stock_items.push(StockItem {
                    id: 136u16,
                    item: Item::BigCraftablesInformation(
                        BIG_CRAFTABLES_INFORMATION
                            .get(&136u16)
                            .context("Error getting big craftables information.")?,
                    ),
                    price: 4000u32,
                    quantity: 1u8,
                });
            }

            if prng.gen_float()? < 0.25f64 {
                stock_items.push(StockItem {
                    id: 433u16,
                    item: Item::ObjectInformation(
                        OBJECT_INFORMATION
                            .get(&433u16)
                            .context("Error getting object information.")?,
                    ),
                    price: 2500u32,
                    quantity: 1u8,
                });
            }

            match stock_items_rows(&stock_items, date, filter) {
                Some(rows) => {
                    table.extend(rows);

                    days_generated += 1u8;
                    if !filter.is_empty() && days_generated >= FILTER_DAYS {
                        break;
                    }
                }
                None => {}
            }
        }

        return Ok(table);
    }

    fn get_messages(configuration: &Configuration) -> Html {
        html!(
            <>
                <Message colour={ MessageColour::Info } body="All stock available from the traveling cart (and night market boat)." />
                {
                    match configuration.date {
                        Some(_) => html!(),
                        None => html!(<Message colour={ MessageColour::Warning } body="Use the date optional configuration parameter to always display from that date." />),
                    }
                }
            </>
        )
    }
}

#[derive(Properties, PartialEq)]
pub struct TravelingCartProperties {
    pub configuration: Configuration,
}

#[function_component]
pub fn TravelingCart(properties: &TravelingCartProperties) -> Html {
    html! {
        <StockTable<TravelingCartImpl> configuration={ properties.configuration.clone() } navigation_step={ NON_FILTER_ITERATIONS as i32 } />
    }
}
