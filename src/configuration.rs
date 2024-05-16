use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
pub enum Platform {
    PC,
    Switch,
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Platform::PC => "PC",
                Platform::Switch => "Switch",
            }
        )
    }
}

#[derive(Clone, PartialEq)]
pub struct Configuration {
    pub platform: Platform,
    pub seed: i32,
    pub date: Option<i32>,
    pub geodes_cracked: Option<u16>,
    pub mine_level: Option<u8>,
    pub qis_crop: Option<bool>,
    pub golden_helmet: Option<bool>,
}
