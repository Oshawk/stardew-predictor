use yew::prelude::*;

use crate::components::table::{TableAlign, TableCell, TableValue};

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
