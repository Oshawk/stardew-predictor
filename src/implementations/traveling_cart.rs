// Need a function that takes:
// - Config
// - Day
// - Search
// And returns a table body.

// A component to select the date.

// A component to set the search.

// A component to change the page.

use std::cmp::max;
use std::collections::HashSet;

use anyhow::Result;
use yew::prelude::*;

use crate::codegen::{ObjectInformation, BIG_CRAFTABLES_INFORMATION, OBJECT_INFORMATION, OBJECT_INFORMATION_OFF_LIMIT, Furniture, FURNITURE};
use crate::components::stock_table::{StockTable, StockTableTrait};
use crate::components::table::TableCell;
use crate::configuration::{Configuration, Platform};
use crate::implementations::util::{day_number, season_number, stock_items_rows, Item, StockItem, get_random_furniture};
use crate::prng::{Jkiss, Prng};

const NON_FILTER_ITERATIONS: u16 = 28u16;
const FILTER_ITERATIONS: u16 = 1000u16;
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

            let seed: i32 = configuration.seed + date;
            let mut prng: Box<dyn Prng> = match configuration.platform {
                Platform::Switch | Platform::PC => Box::new(Jkiss::from_seed(seed)?), // TODO
            };

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

                    let object_information: &ObjectInformation =
                        OBJECT_INFORMATION.get(&id).unwrap();

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

            // This should be different on PC.
            let furniture_price: u32 = prng.gen_range(1i32..11i32)? as u32 * 250u32;
            let furniture_id: u16 = get_random_furniture(&mut prng, 0u16, 1613u16)?;
            stock_items.push(StockItem {
                id: furniture_id,
                item: Item::Furniture(FURNITURE.get(&furniture_id).unwrap()),
                price: furniture_price,
                quantity: 1u8,
            });

            if season_number(date) < 2 {
                stock_items.push(StockItem {
                    id: 347u16,
                    item: Item::ObjectInformation(OBJECT_INFORMATION.get(&347u16).unwrap()),
                    price: 1000u32,
                    quantity: if prng.gen_float()? < 0.1f64 { 5u8 } else { 1u8 },
                });
            } else if prng.gen_float()? < 0.4f64 {
                stock_items.push(StockItem {
                    id: 136u16,
                    item: Item::BigCraftablesInformation(
                        BIG_CRAFTABLES_INFORMATION.get(&136u16).unwrap(),
                    ),
                    price: 4000u32,
                    quantity: 1u8,
                });
            }

            if prng.gen_float()? < 0.25f64 {
                stock_items.push(StockItem {
                    id: 433u16,
                    item: Item::ObjectInformation(OBJECT_INFORMATION.get(&433u16).unwrap()),
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
