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

pub struct ClothingInformation {
    pub name: &'static str,
    pub display_name: &'static str,
    pub description: &'static str,
    pub male_index: u16,
    pub female_index: u16,
    pub price: u32,
    pub rgb: (u8, u8, u8),
    pub dyeable: bool,
    pub type_: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub static OBJECT_INFORMATION_OFF_LIMIT: phf::Set<u16> = phf::phf_set!(
    69u16, 73u16, 79u16, 91u16, 158u16, 159u16, 160u16, 161u16, 162u16, 163u16, 261u16, 277u16,
    279u16, 289u16, 292u16, 305u16, 308u16, 326u16, 341u16, 413u16, 417u16, 437u16, 439u16, 447u16,
    454u16, 460u16, 645u16, 680u16, 681u16, 682u16, 688u16, 689u16, 690u16, 774u16, 775u16, 797u16,
    798u16, 799u16, 800u16, 801u16, 802u16, 803u16, 807u16, 812u16
);

pub static FURNITURE_OFF_LIMIT: phf::Set<u16> = phf::phf_set!(
    131u16, 134u16, 984u16, 985u16, 986u16, 989u16, 1226u16, 1298u16, 1299u16, 1300u16, 1301u16,
    1302u16, 1303u16, 1304u16, 1305u16, 1306u16, 1307u16, 1308u16, 1309u16, 1371u16, 1373u16,
    1375u16, 1402u16, 1466u16, 1468u16, 1471u16, 1541u16, 1545u16, 1554u16, 1669u16, 1671u16,
    1680u16, 1687u16, 1692u16, 1733u16, 1760u16, 1761u16, 1762u16, 1763u16, 1764u16, 1796u16,
    1798u16, 1800u16, 1802u16, 1838u16, 1840u16, 1842u16, 1844u16, 1846u16, 1848u16, 1850u16,
    1852u16, 1854u16, 1900u16, 1902u16, 1907u16, 1909u16, 1914u16, 1915u16, 1916u16, 1917u16,
    1918u16, 1952u16, 1953u16, 1954u16, 1955u16, 1956u16, 1957u16, 1958u16, 1959u16, 1960u16,
    1961u16, 1971u16, 2186u16, 2326u16, 2329u16, 2331u16, 2332u16, 2334u16, 2393u16, 2396u16,
    2400u16, 2418u16, 2419u16, 2421u16, 2423u16, 2425u16, 2426u16, 2428u16, 2496u16, 2502u16,
    2508u16, 2514u16, 2624u16, 2625u16, 2626u16, 2653u16, 2732u16, 2814u16
);
