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
use std::fmt::format;
use anyhow::{bail, Result};
use yew::AttrValue;

use crate::codegen::{OBJECT_INFORMATION, ObjectInformation, OFF_LIMIT};
use crate::configuration::{Configuration, Platform};
use crate::compounds::table::{TableAlign, TableCell, TableProperties, TableValue};
use crate::implementations::util::{stock_items_rows, StockItem};
use crate::prng::{Jkiss, Prng};

const MAXIMUM_ITERATIONS: u32 = 1000u32;
const MAXIMUM_DAYS: u8 = 4u8;

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
    }
}

macro_rules! second_rng {
    ($prng:ident, $constant_multiplier:ident, $variable_multiplier:ident, $quantity_decider:ident) => {
        $constant_multiplier = $prng.gen_range(1i32..11i32)? as u16;
        $variable_multiplier = $prng.gen_range(3i32..6i32)? as u16;
        $quantity_decider = $prng.gen_float()?;
    }
}

pub fn traveling_cart(configuration: &Configuration, date: Option<i32>, filter: &String) -> Result<Vec<Vec<TableCell>>> {
    let date: i32 = date.or(configuration.date).unwrap_or(1i32);

    let mut days_generated: u8 = 0u8;
    let mut table: Vec<Vec<TableCell>> = Vec::new();
    for iteration in 0u32..MAXIMUM_ITERATIONS {
        let date: i32 = date + iteration as i32;
        match (((date - 1i32) % 7i32), ((date - 1i32) % 28i32), ((date - 1i32) / 28i32) % 4i32) {  // Dates start at 1, hence the subtraction.
            (4i32 | 6i32 , _, _) => {},  // Friday or sunday.
            (_, 14i32 | 15i32 | 16i32, 3i32) => {},  // Night market.
            (_, _, _) => continue,
        }

        let seed: i32 = configuration.seed + date;
        let mut prng: Box<dyn Prng> = match configuration.platform {
            Platform::Switch | Platform::PC => Box::new(Jkiss::from_seed(seed)?),  // TODO
        };

        // TODO: Year one completable.

        let mut stock_items: Vec<StockItem> = Vec::<StockItem>::new();

        let mut used_indexes: HashSet<u16> = HashSet::<u16>::new();
        for _ in 0u8..10u8 {
            let mut object_id: u16 = prng.gen_range(2i32..790i32)? as u16;
            stock_items.push(loop {
                object_id += 1u16;
                object_id %= 790u16;

                if !OBJECT_INFORMATION.contains_key(&object_id) || OFF_LIMIT.contains(&object_id) {
                    continue
                }

                let object_information: &ObjectInformation = OBJECT_INFORMATION.get(&object_id).unwrap();

                // PC does the second check before the second RNG generation, Switch does the reverse.
                let constant_multiplier: u16;
                let variable_multiplier: u16;
                let quantity_decider: f64;
                match configuration.platform {
                    Platform::PC => {
                        second_check!(object_information);
                        second_rng!(prng, constant_multiplier, variable_multiplier, quantity_decider);
                    }
                    Platform::Switch => {
                        second_rng!(prng, constant_multiplier, variable_multiplier, quantity_decider);
                        second_check!(object_information);
                    }

                }

                if !used_indexes.insert(object_id) {
                    continue;
                }

                break StockItem {
                    object_id,
                    object_information,
                    price: max(100u16 * constant_multiplier, object_information.price * variable_multiplier),
                    quantity: if quantity_decider < 0.1f64 { 5u8 } else { 1u8 },
                };
            });
        }

        // TODO: Other items

        match stock_items_rows(&stock_items, date, filter) {
            Some(rows) => {
                table.extend(rows);

                days_generated += 1u8;
                if days_generated >= MAXIMUM_DAYS {
                    break
                }
            }
            None => {}
        }
    }

    return Ok(table)
}