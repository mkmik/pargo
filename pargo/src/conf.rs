use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Program {
    pub name: String,
    pub version: String,
    pub platform: Platform,
}

#[derive(Serialize, Deserialize)]
pub enum Platform {
    PDP11,
}
