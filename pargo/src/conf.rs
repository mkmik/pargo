use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub program: Program,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Program {
    pub name: String,
    pub version: String,
    pub platform: Platform,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Platform {
    #[serde(rename = "pdp11")]
    Pdp11,
}
