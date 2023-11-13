pub struct ObjectInformation {
    pub name: &'static str,
    pub price: u32,
    pub edibility: i16,
    pub type_and_category: &'static str,
    pub display_name: &'static str,
    pub description: &'static str,
}

pub struct BigCraftablesInformation {
    pub name: &'static str,
    pub price: u32,
    pub edibility: i16,
    pub type_and_category: &'static str,
    pub description: &'static str,
    pub can_be_set_outdoors: bool,
    pub can_be_set_indoors: bool,
    pub fragility: u8,
    pub display_name: &'static str,
}

pub struct Furniture {
    pub name: &'static str,
    pub type_: &'static str,
    pub source_rectangle_width: u8,
    pub source_rectangle_height: u8,
    pub bounding_box_width: u8,
    pub bounding_box_height: u8,
    pub rotations: u8,
    pub price: u32,
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub static OFF_LIMIT: phf::Set<u16> = phf::phf_set!(
    69u16, 73u16, 79u16, 91u16, 158u16, 159u16, 160u16, 161u16, 162u16, 163u16, 261u16, 277u16,
    279u16, 289u16, 292u16, 305u16, 308u16, 326u16, 341u16, 413u16, 417u16, 437u16, 439u16, 447u16,
    454u16, 460u16, 645u16, 680u16, 681u16, 682u16, 688u16, 689u16, 690u16, 774u16, 775u16, 797u16,
    798u16, 799u16, 800u16, 801u16, 802u16, 803u16, 807u16, 812u16
);
