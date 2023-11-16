use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::str::Split;

trait FromValueSplit {
    fn from_value_split(value_split: &Vec<&str>) -> Self;
}

#[derive(Debug)]
struct ObjectInformation {
    pub name: String,
    pub price: u32,
    pub edibility: i16,
    pub type_and_category: String,
    pub display_name: String,
    pub description: String,
}

impl FromValueSplit for ObjectInformation {
    fn from_value_split(value_split: &Vec<&str>) -> Self {
        Self {
            name: value_split[0usize].to_string(),
            price: value_split[1usize].parse::<u32>().unwrap(),
            edibility: value_split[2usize].parse::<i16>().unwrap(),
            type_and_category: value_split[3usize].to_string(),
            display_name: value_split[4usize].to_string(),
            description: value_split[5usize].to_string(),
        }
    }
}

#[derive(Debug)]
struct BigCraftablesInformation {
    pub name: String,
    pub price: u32,
    pub edibility: i16,
    pub type_and_category: String,
    pub description: String,
    pub can_be_set_outdoors: bool,
    pub can_be_set_indoors: bool,
    pub fragility: u8,
    pub display_name: String,
}

impl FromValueSplit for BigCraftablesInformation {
    fn from_value_split(value_split: &Vec<&str>) -> Self {
        Self {
            name: value_split[0usize].to_string(),
            price: value_split[1usize].parse::<u32>().unwrap(),
            edibility: value_split[2usize].parse::<i16>().unwrap(),
            type_and_category: value_split[3usize].to_string(),
            description: value_split[4usize].to_string(),
            can_be_set_outdoors: value_split[5usize].parse::<bool>().unwrap(),
            can_be_set_indoors: value_split[6usize].parse::<bool>().unwrap(),
            fragility: value_split[7usize].parse::<u8>().unwrap(),
            display_name: value_split[8usize].to_string(),
        }
    }
}

#[derive(Debug)]
struct Furniture {
    pub name: String,
    pub type_: String,
    pub source_rectangle_width: u8,
    pub source_rectangle_height: u8,
    pub bounding_box_width: u8,
    pub bounding_box_height: u8,
    pub rotations: u8,
    pub price: u32,
}

impl FromValueSplit for Furniture {
    // Integer types (Furniture.getTypeNumberFromName):
    // 0 = chair
    // 1 = bench
    // 2 = couch
    // 3 = armchair
    // 4 = dresser
    // 5 = long table
    // 6 = painting
    // 7 = lamp
    // 8 = decor
    // 9 = [default]
    // 10 = bookcase
    // 11 = table
    // 12 = rug
    // 13 = window
    // 14 = fireplace
    // 15 = bed...
    // 16 = torch
    // 17 = sconce
    fn from_value_split(value_split: &Vec<&str>) -> Self {
        let type_: &str = value_split[1usize];

        let (source_rectangle_width, source_rectangle_height): (u8, u8) = match value_split[2usize]
        {
            "-1" => {
                // Furniture.getDefaultSourceRectForType
                match type_ {
                    "chair" | "decor" | "window" | "torch" | "sconce" => (1u8, 2u8),
                    "bench" | "armchair" | "dresser" | "painting" => (2u8, 2u8),
                    "couch" | "rug" => (3u8, 2u8),
                    "long table" => (5u8, 3u8),
                    "lamp" => (1u8, 3u8),
                    "bookcase" | "table" => (2u8, 3u8),
                    "fireplace" => (2u8, 5u8),
                    _ => panic!("{}", type_),
                }
            }
            source_rectangle => {
                let mut source_rectangle_split: Split<&str> = source_rectangle.split(" ");
                (
                    source_rectangle_split
                        .next()
                        .unwrap()
                        .parse::<u8>()
                        .unwrap(),
                    source_rectangle_split
                        .next()
                        .unwrap()
                        .parse::<u8>()
                        .unwrap(),
                )
            }
        };

        let (bounding_box_width, bounding_box_height): (u8, u8) = match value_split[2usize] {
            "-1" => {
                // Furniture.getDefaultBoundingBoxForType
                match type_ {
                    "chair" | "lamp" | "decor" | "torch" => (1u8, 1u8),
                    "bench" | "armchair" | "dresser" | "bookcase" | "fireplace" => (2u8, 1u8),
                    "couch" => (3u8, 1u8),
                    "long table" => (5u8, 2u8),
                    "painting" | "table" => (2u8, 2u8),
                    "rug" => (3u8, 2u8),
                    "window" | "sconce" => (1u8, 2u8),
                    _ => panic!("{}", type_),
                }
            }
            bounding_box => {
                let mut bounding_box_split: Split<&str> = bounding_box.split(" ");
                (
                    bounding_box_split.next().unwrap().parse::<u8>().unwrap(),
                    bounding_box_split.next().unwrap().parse::<u8>().unwrap(),
                )
            }
        };

        Self {
            name: value_split[0usize].to_string(),
            type_: type_.to_string(),
            source_rectangle_width,
            source_rectangle_height,
            bounding_box_width,
            bounding_box_height,
            rotations: value_split[4usize].parse::<u8>().unwrap(),
            price: value_split[5usize].parse::<u32>().unwrap(),
        }
    }
}

fn load<T: Debug + FromValueSplit>(
    out_file: &mut BufWriter<File>,
    path: &Path,
    constant_name: &str,
    struct_name: &str,
) {
    let file: File = File::open(path).unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();

    let mut map: HashMap<u16, T> = HashMap::new();

    for (key, value) in json.get("content").unwrap().as_object().unwrap() {
        let value_split: Vec<&str> = value.as_str().unwrap().split("/").collect();
        map.insert(
            key.parse::<u16>().unwrap(),
            T::from_value_split(&value_split),
        );
    }

    let mut builder: phf_codegen::Map<u16> = phf_codegen::Map::new();
    for (key, value) in &map {
        builder.entry(key.clone(), format!("{:?}", value).as_str());
    }

    writeln!(
        out_file,
        "pub static {}: phf::Map<u16, {}> = {};",
        constant_name,
        struct_name,
        builder.build(),
    )
    .unwrap();
}
fn main() {
    let out_path: PathBuf = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut out_file: BufWriter<File> = BufWriter::new(File::create(&out_path).unwrap());

    let object_information_path: &Path = Path::new("assets/ObjectInformation.json");
    load::<ObjectInformation>(
        &mut out_file,
        object_information_path,
        "OBJECT_INFORMATION",
        "ObjectInformation",
    );

    let big_craftables_information_path: &Path = Path::new("assets/BigCraftablesInformation.json");
    load::<BigCraftablesInformation>(
        &mut out_file,
        big_craftables_information_path,
        "BIG_CRAFTABLES_INFORMATION",
        "BigCraftablesInformation",
    );

    let furniture_path: &Path = Path::new("assets/Furniture.json");
    load::<Furniture>(&mut out_file, furniture_path, "FURNITURE", "Furniture");
}
