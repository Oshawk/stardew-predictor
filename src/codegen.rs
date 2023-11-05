pub struct ObjectInformation {
    pub name: &'static str,
    pub price: u16,
    pub edibility: i16,
    pub type_and_category: &'static str,
    pub display_name: &'static str,
    pub description: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub static OFF_LIMIT: phf::Set<u16> = phf::phf_set!(
    69u16, 73u16, 79u16, 91u16, 158u16, 159u16, 160u16, 161u16, 162u16, 163u16, 261u16, 277u16,
    279u16, 289u16, 292u16, 305u16, 308u16, 326u16, 341u16, 413u16, 417u16, 437u16, 439u16, 447u16,
    454u16, 460u16, 645u16, 680u16, 681u16, 682u16, 688u16, 689u16, 690u16, 774u16, 775u16, 797u16,
    798u16, 799u16, 800u16, 801u16, 802u16, 803u16, 807u16, 812u16
);
