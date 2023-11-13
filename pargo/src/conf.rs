use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Program {
    pub name: String,
    pub version: String,
    pub platform: Platform,
}

#[derive(Serialize, Deserialize)]
pub enum Platform {
    PDP11,
}
