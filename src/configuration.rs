#[derive(Clone, Copy, PartialEq)]
pub enum Platform {
    PC,
    Switch,
}

impl ToString for Platform {
    fn to_string(&self) -> String {
        match self {
            Platform::PC => "PC",
            Platform::Switch => "Switch",
        }
        .to_string()
    }
}

#[derive(Clone, PartialEq)]
pub struct Configuration {
    pub platform: Platform,
    pub seed: i32,
    pub date: Option<i32>,
}
