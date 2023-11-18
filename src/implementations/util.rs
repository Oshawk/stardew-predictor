use anyhow::{Context, Result};
use yew::prelude::*;

use crate::codegen::{
    BigCraftablesInformation, ClothingInformation, Furniture, ObjectInformation,
    CLOTHING_INFORMATION, FURNITURE, FURNITURE_OFF_LIMIT,
};
use crate::components::table::{TableAlign, TableCell, TableValue};
use crate::configuration::Platform;
use crate::prng::{Jkiss, MsCorLibRandom, Prng};

#[derive(Clone, Copy, PartialEq)]
pub enum Implementation {
    TravelingCart,
    Krobus,
    Sandy,
    Pierre,
    Joja,
}

impl ToString for Implementation {
    fn to_string(&self) -> String {
        match self {
            Self::TravelingCart => "Traveling Cart",
            Self::Krobus => "Krobus",
            Self::Sandy => "Sandy",
            Self::Pierre => "Pierre",
            Self::Joja => "Joja",
        }
        .to_string()
    }
}

pub enum Item {
    ObjectInformation(&'static ObjectInformation),
    BigCraftablesInformation(&'static BigCraftablesInformation),
    Furniture(&'static Furniture),
    ClothingInformation(&'static ClothingInformation),
    Wallpaper(bool),
}

impl Item {
    pub fn name(&self, id: u16) -> String {
        match self {
            Self::ObjectInformation(object_information) => object_information.name.to_string(),
            Self::BigCraftablesInformation(big_craftable_information) => {
                big_craftable_information.name.to_string()
            }
            Self::Furniture(furniture) => furniture.name.to_string(),
            Self::ClothingInformation(clothing_information) => format!(
                "{} ({})",
                clothing_information.name,
                if id >= 1000u16 { id - 1000u16 } else { id }
            ),
            Self::Wallpaper(flooring) => format!(
                "{} ({})",
                if *flooring { "Flooring" } else { "Wallpaper" },
                id,
            ),
        }
    }

    pub fn sprite(&self, id: u16) -> TableValue {
        match self {
            Self::ObjectInformation(_) => TableValue::Sprite(
                AttrValue::from("springobjects.png"),
                (id % 24u16) * 16u16,
                (id / 24u16) * 16u16,
                16u16,
                16u16,
                384u16,
                624u16,
            ),
            Self::BigCraftablesInformation(_) => TableValue::Sprite(
                AttrValue::from("Craftables.png"),
                (id % 8u16) * 16u16,
                (id / 8u16) * 32u16,
                16u16,
                32u16,
                128u16,
                1152u16,
            ),
            Self::Furniture(furniture) => TableValue::Sprite(
                AttrValue::from("furniture.png"),
                (id % 32u16) * 16u16,
                (id / 32u16) * 16u16,
                furniture.source_rectangle_width as u16 * 16u16,
                furniture.source_rectangle_height as u16 * 16u16,
                512u16,
                1488u16,
            ),
            Self::ClothingInformation(clothing_information) => {
                let index: u16 = match clothing_information.name {
                    "Shirt" => id - 1000u16, // The generic shirt.
                    _ => clothing_information.male_index,
                };

                TableValue::Sprite(
                    AttrValue::from("shirts.png"),
                    (index % 16u16) * 8u16,
                    (index / 16u16) * 8u16 * 4u16,
                    8u16,
                    8u16,
                    256u16,
                    608u16,
                )
            }
            Self::Wallpaper(flooring) => match flooring {
                true => TableValue::Sprite(
                    AttrValue::from("walls_and_floors.png"),
                    (id % 8u16) * 32u16,
                    (id / 8u16) * 32u16 + 336u16,
                    28u16,
                    26u16,
                    256u16,
                    560u16,
                ),
                false => TableValue::Sprite(
                    AttrValue::from("walls_and_floors.png"),
                    (id % 16u16) * 16u16,
                    (id / 16u16) * 48u16 + 8u16,
                    16u16,
                    28u16,
                    256u16,
                    560u16,
                ),
            },
        }
    }
}

pub struct StockItem {
    pub id: u16,
    pub item: Item,
    pub price: u32,
    pub quantity: u8,
}

pub fn get_prng(platform: Platform, seed: i32) -> Result<Box<dyn Prng>> {
    Ok(match platform {
        Platform::Switch => Box::new(Jkiss::from_seed(seed)?),
        Platform::PC => Box::new(MsCorLibRandom::from_seed(seed)?),
    })
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
        .filter(|stock_item: &&StockItem| {
            stock_item
                .item
                .name(stock_item.id)
                .to_lowercase()
                .contains(filter)
        })
        .map(|stock_item: &StockItem| {
            let mut row: Vec<TableCell> = Vec::new();
            row.push(TableCell {
                value: stock_item.item.sprite(stock_item.id),
                align: TableAlign::MiddleCenter,
                rows: 1u8,
                columns: 1u8,
            });
            row.push(TableCell {
                value: TableValue::String(AttrValue::from(stock_item.item.name(stock_item.id))),
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

pub fn get_random_furniture(
    prng: &mut Box<dyn Prng>,
    lower_bound: u16,
    upper_bound: u16,
) -> Result<u16> {
    loop {
        let id: u16 = prng.gen_range((lower_bound as i32)..(upper_bound as i32))? as u16;
        if !FURNITURE.contains_key(&id) || FURNITURE_OFF_LIMIT.contains(&id) {
            continue;
        }
        return Ok(id);
    }
}

pub fn get_clothing_information(id: u16) -> Result<&'static ClothingInformation> {
    Ok(match CLOTHING_INFORMATION.get(&id) {
        Some(item) => item,
        None => {
            if id >= 1000u16 {
                CLOTHING_INFORMATION
                    .get(&(u16::MAX - 2u16))
                    .context("Error getting clothing information.")?
            } else {
                CLOTHING_INFORMATION
                    .get(&(u16::MAX - 1u16))
                    .context("Error getting clothing information.")?
            }
        }
    })
}
