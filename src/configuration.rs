#[derive(Clone, Copy, PartialEq)]
pub enum Platform {
    SWITCH,
}

impl ToString for Platform {
    fn to_string(&self) -> String {
        match self {
            Platform::SWITCH => "Switch",
        }.to_string()
    }
}

pub struct Configuration {
    pub platform: Platform,
    pub seed: i32,
}
