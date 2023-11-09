use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct ObjectInformation {
    pub name: String,
    pub price: u16,
    pub edibility: i16,
    pub type_and_category: String,
    pub display_name: String,
    pub description: String,
}

#[derive(Debug)]
struct BigCraftablesInformation {
    pub name: String,
    pub price: u16,
    pub edibility: i16,
    pub type_and_category: String,
    pub description: String,
    pub can_be_set_outdoors: bool,
    pub can_be_set_indoors: bool,
    pub fragility: u8,
    pub display_name: String,
}

fn main() {
    let out_path: PathBuf = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut out_file: BufWriter<File> = BufWriter::new(File::create(&out_path).unwrap());

    let object_information_path: &Path = Path::new("assets/ObjectInformation.json");
    let object_information_file: File = File::open(&object_information_path).unwrap();
    let object_information_json: serde_json::Value =
        serde_json::from_reader(object_information_file).unwrap();

    let mut object_information_map: HashMap<u16, ObjectInformation> = HashMap::new();
    for (key, value) in object_information_json
        .get("content")
        .unwrap()
        .as_object()
        .unwrap()
    {
        let value_split: Vec<&str> = value.as_str().unwrap().split("/").collect();
        object_information_map.insert(
            key.parse::<u16>().unwrap(),
            ObjectInformation {
                name: value_split[0usize].to_string(),
                price: value_split[1usize].parse::<u16>().unwrap(),
                edibility: value_split[2usize].parse::<i16>().unwrap(),
                type_and_category: value_split[3usize].to_string(),
                display_name: value_split[4usize].to_string(),
                description: value_split[5usize].to_string(),
            },
        );
    }

    let mut object_information_builder: phf_codegen::Map<u16> = phf_codegen::Map::new();
    for (key, value) in &object_information_map {
        object_information_builder.entry(key.clone(), format!("{:?}", value).as_str());
    }

    writeln!(
        &mut out_file,
        "pub static OBJECT_INFORMATION: phf::Map<u16, ObjectInformation> = {};",
        object_information_builder.build()
    )
    .unwrap();

    let big_craftables_information_path: &Path = Path::new("assets/BigCraftablesInformation.json");
    let big_craftables_information_file: File =
        File::open(&big_craftables_information_path).unwrap();
    let big_craftables_information_json: serde_json::Value =
        serde_json::from_reader(big_craftables_information_file).unwrap();

    let mut big_craftables_information_map: HashMap<u16, BigCraftablesInformation> = HashMap::new();
    for (key, value) in big_craftables_information_json
        .get("content")
        .unwrap()
        .as_object()
        .unwrap()
    {
        let value_split: Vec<&str> = value.as_str().unwrap().split("/").collect();
        big_craftables_information_map.insert(
            key.parse::<u16>().unwrap(),
            BigCraftablesInformation {
                name: value_split[0usize].to_string(),
                price: value_split[1usize].parse::<u16>().unwrap(),
                edibility: value_split[2usize].parse::<i16>().unwrap(),
                type_and_category: value_split[3usize].to_string(),
                description: value_split[4usize].to_string(),
                can_be_set_outdoors: value_split[5usize].parse::<bool>().unwrap(),
                can_be_set_indoors: value_split[6usize].parse::<bool>().unwrap(),
                fragility: value_split[7usize].parse::<u8>().unwrap(),
                display_name: value_split[8usize].to_string(),
            },
        );
    }

    let mut big_craftables_information_builder: phf_codegen::Map<u16> = phf_codegen::Map::new();
    for (key, value) in &big_craftables_information_map {
        big_craftables_information_builder.entry(key.clone(), format!("{:?}", value).as_str());
    }

    writeln!(
        &mut out_file,
        "pub static BIG_CRAFTABLES_INFORMATION: phf::Map<u16, BigCraftablesInformation> = {};",
        big_craftables_information_builder.build()
    )
    .unwrap();
}
