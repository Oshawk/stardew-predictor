use std::cmp::max;

use anyhow::Result;
use yew::prelude::*;

use crate::components::date::DateJump;
use crate::components::filter::Filter;
use crate::components::message::{Message, MessageColour};
use crate::components::navigation::{Navigation, NavigationDirection};
use crate::components::table::{Table, TableAlign, TableCell, TableValue};
use crate::configuration::Configuration;

pub fn stock_items_table_header() -> Vec<Vec<TableCell>> {
    vec![vec![
        TableCell {
            value: TableValue::String(AttrValue::from("Date")),
            align: TableAlign::MiddleLeft,
            rows: 1,
            columns: 1,
        },
        TableCell {
            value: TableValue::String(AttrValue::from("Item")),
            align: TableAlign::MiddleLeft,
            rows: 1,
            columns: 2,
        },
        TableCell {
            value: TableValue::String(AttrValue::from("Price")),
            align: TableAlign::MiddleLeft,
            rows: 1,
            columns: 1,
        },
        TableCell {
            value: TableValue::String(AttrValue::from("Quantity")),
            align: TableAlign::MiddleLeft,
            rows: 1,
            columns: 1,
        },
    ]]
}

pub trait StockTableTrait {
    fn get_stock(
        configuration: &Configuration,
        date: i32,
        filter: &String,
    ) -> Result<Vec<Vec<TableCell>>>;

    fn get_messages(configuration: &Configuration) -> Html;
}

#[derive(Properties, PartialEq)]
pub struct StockTableProperties {
    pub configuration: Configuration,
    pub navigation_step: i32,
}

#[component]
pub fn StockTable<T: StockTableTrait>(properties: &StockTableProperties) -> Html {
    let date = use_state_eq(|| properties.configuration.date.unwrap_or(1));
    let filter = use_state_eq(|| String::new());

    let date_jump_updated = {
        let date = date.clone();
        Callback::from(move |value: i32| date.set(value))
    };

    let filter_updated = {
        let filter = filter.clone();
        Callback::from(move |value: String| filter.set(value))
    };

    let navigation_step = properties.navigation_step;
    let navigation_updated = {
        let date = date.clone();
        Callback::from(move |direction: NavigationDirection| match direction {
            NavigationDirection::Backward => {
                date.set(max(*date - navigation_step, 1));
            }
            NavigationDirection::Forward => {
                date.set(*date + navigation_step);
            }
        })
    };

    match T::get_stock(&properties.configuration, *date, &filter) {
        Ok(table) => {
            html!(
                <>
                    { T::get_messages(&properties.configuration) }
                    <div class="columns">
                        <div class="column">
                            <DateJump updated={ date_jump_updated } />
                        </div>
                        <div class="column">
                            <Filter updated={ filter_updated } />
                        </div>
                    </div>
                    <Navigation updated={ navigation_updated.clone() } disabled={ !filter.is_empty() } />
                    <Table header={ stock_items_table_header() } body={ table } />
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
