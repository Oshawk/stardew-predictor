use anyhow::{Context, Result};
use yew::prelude::*;

use crate::codegen::{CLOTHING_INFORMATION, FURNITURE, OBJECT_INFORMATION};
use crate::components::message::{Message, MessageColour};
use crate::components::stock_table::{StockTable, StockTableTrait};
use crate::components::table::TableCell;
use crate::configuration::Configuration;
use crate::implementations::util::Item::ObjectInformation;
use crate::implementations::util::{
    day_number, get_clothing_information, get_prng, stock_items_rows, Item, StockItem,
};
use crate::prng::Prng;

const NON_FILTER_ITERATIONS: u16 = 28u16;
const FILTER_ITERATIONS: u16 = 1120u16;
const FILTER_DAYS: u8 = 8u8;

pub struct SandyImpl {}

impl StockTableTrait for SandyImpl {
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

            let mut prng: Box<dyn Prng> =
                get_prng(configuration.platform, configuration.seed / 2i32 + date)?;

            let mut stock_items: Vec<StockItem> = vec![{
                let id: u16 = 1000u16 + prng.gen_range(0i32..127i32)? as u16;
                StockItem {
                    id,
                    item: Item::ClothingInformation(get_clothing_information(id)?),
                    price: 700u32,
                    quantity: 1u8,
                }
            }];

            if day_number(date) % 7u8 == 1u8 {
                let id: u16 = 2734u16 + prng.gen_range(0i32..4i32)? as u16 * 2u16;
                stock_items.push(StockItem {
                    id,
                    item: Item::Furniture(FURNITURE.get(&id).context("Error getting furniture.")?),
                    price: 500u32,
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

        Ok(table)
    }

    fn get_messages(configuration: &Configuration) -> Html {
        html!(
            <>
                <Message colour={ MessageColour::Info } body="Random stock from Sandy." />
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
pub struct SandyProperties {
    pub configuration: Configuration,
}

#[function_component]
pub fn Sandy(properties: &SandyProperties) -> Html {
    html! {
        <StockTable<SandyImpl> configuration={ properties.configuration.clone() } navigation_step={ NON_FILTER_ITERATIONS as i32 } />
    }
}
