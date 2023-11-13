use std::path::PathBuf;

use crate::conf::Config;

pub struct Env {
    pub config: Config,
    pub target_dir: PathBuf,
}
