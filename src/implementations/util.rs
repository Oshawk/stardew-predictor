use yew::prelude::*;

use crate::codegen::{BigCraftablesInformation, Furniture, ObjectInformation};
use crate::components::table::{TableAlign, TableCell, TableValue};
use crate::implementations::traveling_cart::TravelingCart;

const OBJECT_INFORMATION_ICON_FILE: &'static str = "springobjects.png";
const OBJECT_INFORMATION_ICON_SIZE: u16 = 16u16;
const OBJECT_INFORMATION_ICONS_PER_ROW: u16 = 24u16;
const OBJECT_INFORMATION_ICON_SHEET_WIDTH: u16 = 384u16;
const OBJECT_INFORMATION_ICON_SHEET_HEIGHT: u16 = 624u16;

const BIG_CRAFTABLES_INFORMATION_ICON_FILE: &'static str = "Craftables.png";
const BIG_CRAFTABLES_INFORMATION_ICON_WIDTH: u16 = 16u16;
const BIG_CRAFTABLES_INFORMATION_ICON_HEIGHT: u16 = 32u16;
const BIG_CRAFTABLES_INFORMATION_ICONS_PER_ROW: u16 = 8u16;
const BIG_CRAFTABLES_INFORMATION_ICON_SHEET_WIDTH: u16 = 128u16;
const BIG_CRAFTABLES_INFORMATION_ICON_SHEET_HEIGHT: u16 = 1152u16;

const FURNITURE_ICON_FILE: &'static str = "furniture.png";
const FURNITURE_ICON_UNIT: u16 = 16u16;
const FURNITURE_ICON_UNITS_PER_ROW: u16 = 32u16;
const FURNITURE_ICON_SHEET_WIDTH: u16 = 512u16;
const FURNITURE_ICON_SHEET_HEIGHT: u16 = 1488u16;

#[derive(Clone, Copy, PartialEq)]
pub enum Implementation {
    TravelingCart,
}

impl ToString for Implementation {
    fn to_string(&self) -> String {
        match self {
            Implementation::TravelingCart => "Traveling Cart",
        }
        .to_string()
    }
}

pub enum Item {
    ObjectInformation(&'static ObjectInformation),
    BigCraftablesInformation(&'static BigCraftablesInformation),
    Furniture(&'static Furniture),
}

impl Item {
    pub fn name(&self) -> &'static str {
        match self {
            Item::ObjectInformation(object) => object.name,
            Item::BigCraftablesInformation(big_craftable) => big_craftable.name,
            Item::Furniture(furniture) => furniture.name,
        }
    }
}

pub struct StockItem {
    pub id: u16,
    pub item: Item,
    pub price: u16,
    pub quantity: u8,
}

pub fn day_number(date: i32) -> u8 {
    ((date - 1i32) % 28i32) as u8
}

pub fn day_name(date: i32) -> &'static str {
    match day_number(date) % 7u8 {
        0u8 => "Monday",
        1u8 => "Tuesday",
        2u8 => "Wednesday",
        3u8 => "Thursday",
        4u8 => "Friday",
        5u8 => "Saturday",
        6u8 => "Sunday",
        _ => panic!(),
    }
}

pub fn season_number(date: i32) -> u8 {
    (((date - 1i32) / 28i32) % 4i32) as u8
}

pub fn season_name(date: i32) -> &'static str {
    match season_number(date) {
        0u8 => "Spring",
        1u8 => "Summer",
        2u8 => "Fall",
        3u8 => "Winder",
        _ => panic!(),
    }
}

pub fn year_number(date: i32) -> u32 {
    ((date - 1i32) / 112i32) as u32
}

pub fn format_date(date: i32) -> String {
    if date <= 0 {
        return "UNEXPECTED".to_string();
    }

    format!(
        "{} {} {}, Year {}",
        day_name(date),
        season_name(date),
        day_number(date) + 1u8,
        year_number(date) + 1u32
    )
}

pub fn stock_items_rows(
    stock_items: &Vec<StockItem>,
    date: i32,
    filter: &String,
) -> Option<Vec<Vec<TableCell>>> {
    let mut rows: Vec<Vec<TableCell>> = stock_items
        .iter()
        .filter(|stock_item: &&StockItem| stock_item.item.name().to_lowercase().contains(filter))
        .map(|stock_item: &StockItem| {
            let mut row: Vec<TableCell> = Vec::new();
            row.push(TableCell {
                value: match stock_item.item {
                    Item::ObjectInformation(_) => TableValue::Sprite(
                        AttrValue::from(OBJECT_INFORMATION_ICON_FILE),
                        (stock_item.id % OBJECT_INFORMATION_ICONS_PER_ROW)
                            * OBJECT_INFORMATION_ICON_SIZE,
                        (stock_item.id / OBJECT_INFORMATION_ICONS_PER_ROW)
                            * OBJECT_INFORMATION_ICON_SIZE,
                        OBJECT_INFORMATION_ICON_SIZE,
                        OBJECT_INFORMATION_ICON_SIZE,
                        OBJECT_INFORMATION_ICON_SHEET_WIDTH,
                        OBJECT_INFORMATION_ICON_SHEET_HEIGHT,
                    ),
                    Item::BigCraftablesInformation(_) => TableValue::Sprite(
                        AttrValue::from(BIG_CRAFTABLES_INFORMATION_ICON_FILE),
                        (stock_item.id % BIG_CRAFTABLES_INFORMATION_ICONS_PER_ROW)
                            * BIG_CRAFTABLES_INFORMATION_ICON_WIDTH,
                        (stock_item.id / BIG_CRAFTABLES_INFORMATION_ICONS_PER_ROW)
                            * BIG_CRAFTABLES_INFORMATION_ICON_HEIGHT,
                        BIG_CRAFTABLES_INFORMATION_ICON_WIDTH,
                        BIG_CRAFTABLES_INFORMATION_ICON_HEIGHT,
                        BIG_CRAFTABLES_INFORMATION_ICON_SHEET_WIDTH,
                        BIG_CRAFTABLES_INFORMATION_ICON_SHEET_HEIGHT,
                    ),
                    Item::Furniture(furniture) => TableValue::Sprite(
                        AttrValue::from(FURNITURE_ICON_FILE),
                        (stock_item.id % FURNITURE_ICON_UNITS_PER_ROW) * FURNITURE_ICON_UNIT,
                        (stock_item.id / FURNITURE_ICON_UNITS_PER_ROW) * FURNITURE_ICON_UNIT,
                        furniture.source_rectangle_width as u16 * FURNITURE_ICON_UNIT,
                        furniture.source_rectangle_height as u16 * FURNITURE_ICON_UNIT,
                        FURNITURE_ICON_SHEET_WIDTH,
                        FURNITURE_ICON_SHEET_WIDTH,
                    ),
                },
                align: TableAlign::MiddleCenter,
                rows: 1u8,
                columns: 1u8,
            });
            row.push(TableCell {
                value: TableValue::String(AttrValue::from(stock_item.item.name())),
                align: TableAlign::MiddleLeft,
                rows: 1u8,
                columns: 1u8,
            });
            row.push(TableCell {
                value: TableValue::String(AttrValue::from(format!("{}g", stock_item.price))),
                align: TableAlign::MiddleLeft,
                rows: 1u8,
                columns: 1u8,
            });
            row.push(TableCell {
                value: TableValue::String(AttrValue::from(format!("x{}", stock_item.quantity))),
                align: TableAlign::MiddleLeft,
                rows: 1u8,
                columns: 1u8,
            });

            row
        })
        .collect();

    let rows_length: u8 = rows.len() as u8;

    if rows_length == 0u8 {
        return None;
    }

    rows[0].insert(
        0,
        TableCell {
            value: TableValue::String(AttrValue::from(format_date(date))),
            align: TableAlign::MiddleLeft,
            rows: rows_length,
            columns: 1u8,
        },
    );

    Some(rows)
}
