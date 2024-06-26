use std::cmp::max;

use anyhow::Result;
use yew::prelude::*;

use crate::components::date_jump::DateJump;
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
            rows: 1u8,
            columns: 1u8,
        },
        TableCell {
            value: TableValue::String(AttrValue::from("Item")),
            align: TableAlign::MiddleLeft,
            rows: 1u8,
            columns: 2u8,
        },
        TableCell {
            value: TableValue::String(AttrValue::from("Price")),
            align: TableAlign::MiddleLeft,
            rows: 1u8,
            columns: 1u8,
        },
        TableCell {
            value: TableValue::String(AttrValue::from("Quantity")),
            align: TableAlign::MiddleLeft,
            rows: 1u8,
            columns: 1u8,
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

#[function_component]
pub fn StockTable<T: StockTableTrait>(properties: &StockTableProperties) -> Html {
    let date: UseStateHandle<i32> = use_state_eq(|| properties.configuration.date.unwrap_or(1i32));
    let filter: UseStateHandle<String> = use_state_eq(|| "".to_string());

    let date_jump_updated: Callback<i32> =
        use_callback(date.clone(), |date_: i32, date: &UseStateHandle<i32>| {
            date.set(date_);
        });

    let filter_updated: Callback<String> = use_callback(
        filter.clone(),
        |filter_: String, filter: &UseStateHandle<String>| {
            filter.set(filter_);
        },
    );

    let navigation_step: i32 = properties.navigation_step;
    let navigation_updated: Callback<NavigationDirection> = use_callback(
        date.clone(),
        move |direction: NavigationDirection, date: &UseStateHandle<i32>| match direction {
            NavigationDirection::Backward => {
                date.set(max(**date - navigation_step, 1i32));
            }
            NavigationDirection::Forward => {
                date.set(**date + navigation_step);
            }
        },
    );

    match T::get_stock(&properties.configuration, *date, &*filter) {
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
