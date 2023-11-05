use yew::AttrValue;

use crate::codegen::ObjectInformation;
use crate::compounds::table::{TableAlign, TableCell, TableValue};

const ICON_SIZE: u16 = 16u16;
const ICONS_PER_ROW: u16 = 24u16;

pub struct StockItem {
    pub object_id: u16,
    pub object_information: &'static ObjectInformation,
    pub price: u16,
    pub quantity: u8,
}

pub fn format_date(date: i32) -> String {
    if date <= 0 {
        return "UNEXPECTED".to_string();
    }

    let day: u8 = ((date - 1i32) % 28i32) as u8 + 1;
    let day_: &str = match (day - 1u8) % 7u8 {
        0u8 => "Monday",
        1u8 => "Tuesday",
        2u8 => "Wednesday",
        3u8 => "Thursday",
        4u8 => "Friday",
        5u8 => "Saturday",
        6u8 => "Sunday",
        _ => panic!(),
    };
    let season: &str = match ((date - 1i32) / 28i32) % 4i32 {
        0i32 => "Spring",
        1i32 => "Summer",
        2i32 => "Fall",
        3i32 => "Winder",
        _ => panic!(),
    };
    let year: u32 = ((date - 1i32) / 112i32) as u32 + 1u32;

    format!("{} {} {}, Year {}", day_, season, day, year)
}

pub fn stock_items_rows(stock_items: &Vec<StockItem>, date: i32, filter: &String) -> Option<Vec<Vec<TableCell>>> {
    let mut rows: Vec<Vec<TableCell>> = stock_items.iter().filter(|stock_item: &&StockItem| stock_item.object_information.name.contains(filter)).map(|stock_item: &StockItem| {
        let mut row: Vec<TableCell> = Vec::new();
        row.push(TableCell {
            value: TableValue::Sprite(AttrValue::from("springobjects.png"), (stock_item.object_id % ICONS_PER_ROW) * ICON_SIZE, (stock_item.object_id / ICONS_PER_ROW) * ICON_SIZE, ICON_SIZE, ICON_SIZE),
            align: TableAlign::MiddleCenter,
            rows: 1u8,
            columns: 1u8,
        });
        row.push(TableCell {
            value: TableValue::String(AttrValue::from(stock_item.object_information.name)),
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
    }).collect();

    let rows_length: u8 = rows.len() as u8;

    if rows_length == 0u8 {
        return None;
    }

    rows[0].insert(0, TableCell {
        value: TableValue::String(AttrValue::from(format_date(date))),
        align: TableAlign::MiddleLeft,
        rows: rows_length,
        columns: 1u8,
    });

    Some(rows)
}