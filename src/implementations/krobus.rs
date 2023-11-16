use anyhow::{Context, Result};
use yew::prelude::*;

use crate::codegen::OBJECT_INFORMATION;
use crate::components::message::{Message, MessageColour};
use crate::components::stock_table::{StockTable, StockTableTrait};
use crate::components::table::TableCell;
use crate::configuration::Configuration;
use crate::implementations::util::Item::ObjectInformation;
use crate::implementations::util::{day_number, get_prng, stock_items_rows, StockItem};
use crate::prng::Prng;

const NON_FILTER_ITERATIONS: u16 = 112u16;
const FILTER_ITERATIONS: u16 = 1120u16;
const FILTER_DAYS: u8 = 8u8;

pub struct KrobusImpl {}

impl StockTableTrait for KrobusImpl {
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
            match day_number(date) % 7u8 {
                2u8 | 5u8 => {}
                _ => continue,
            }

            let mut prng: Box<dyn Prng> =
                get_prng(configuration.platform, configuration.seed + date / 2i32)?;

            let stock_items: Vec<StockItem> = vec![match day_number(date) % 7u8 {
                2u8 => {
                    let id = prng.gen_range(698i32..709i32)? as u16;
                    StockItem {
                        id,
                        item: ObjectInformation(
                            OBJECT_INFORMATION
                                .get(&id)
                                .context("Error getting object information.")?,
                        ),
                        price: 200u32,
                        quantity: 5u8,
                    }
                }
                5u8 => {
                    let mut id = prng.gen_range(194i32..245i32)? as u16;
                    if id == 217u16 {
                        id = 216u16;
                    }
                    StockItem {
                        id,
                        item: ObjectInformation(
                            OBJECT_INFORMATION
                                .get(&id)
                                .context("Error getting object information.")?,
                        ),
                        price: prng.gen_range(5i32..51i32)? as u32 * 10u32,
                        quantity: 5u8,
                    }
                }
                _ => panic!(),
            }];

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

        Ok(table)
    }

    fn get_messages(configuration: &Configuration) -> Html {
        html!(
            <>
                <Message colour={ MessageColour::Info } body="Random stock from Krobus." />
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
pub struct KrobusProperties {
    pub configuration: Configuration,
}

#[function_component]
pub fn Krobus(properties: &KrobusProperties) -> Html {
    html! {
        <StockTable<KrobusImpl> configuration={ properties.configuration.clone() } navigation_step={ NON_FILTER_ITERATIONS as i32 } />
    }
}
