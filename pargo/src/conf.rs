use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub program: Program,
    pub pdp11: Option<Pdp11>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Program {
    pub name: String,
    pub version: String,
    pub platform: Platform,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum Platform {
    #[serde(rename = "pdp11")]
    Pdp11,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Pdp11 {
    pub cpu: String,
    pub mem: String,
}
