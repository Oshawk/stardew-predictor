use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

trait FromValueSplit {
    fn from_value_split(value_split: &Vec<&str>) -> Self;
}

#[derive(Debug)]
struct ObjectInformation {
    pub name: String,
    pub price: u16,
    pub edibility: i16,
    pub type_and_category: String,
    pub display_name: String,
    pub description: String,
}

impl FromValueSplit for ObjectInformation {
    fn from_value_split(value_split: &Vec<&str>) -> Self {
        ObjectInformation {
            name: value_split[0usize].to_string(),
            price: value_split[1usize].parse::<u16>().unwrap(),
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
    pub price: u16,
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
        }
    }
}

fn load<T: Debug + FromValueSplit>(out_file: &mut BufWriter<File>, path: &Path, constant_name: &str, struct_name: &str) {
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
    ).unwrap();
}
fn main() {
    let out_path: PathBuf = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut out_file: BufWriter<File> = BufWriter::new(File::create(&out_path).unwrap());

    let object_information_path: &Path = Path::new("assets/ObjectInformation.json");
    load::<ObjectInformation>(&mut out_file, object_information_path, "OBJECT_INFORMATION", "ObjectInformation");

    let big_craftables_information_path: &Path = Path::new("assets/BigCraftablesInformation.json");
    load::<BigCraftablesInformation>(&mut out_file, big_craftables_information_path, "BIG_CRAFTABLES_INFORMATION", "BigCraftablesInformation");
}
