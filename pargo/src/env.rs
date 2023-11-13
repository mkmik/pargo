use std::path::{Path, PathBuf};

use crate::conf::Config;

pub struct Env {
    pub config: Config,
    pub target_dir: PathBuf,
}

impl Env {
    pub fn build_dir(&self) -> std::io::Result<PathBuf> {
        join(self.target_dir, "build")
    }
}

fn join<B: AsRef<Path>, P: AsRef<Path>>(base: B, path: P) -> std::io::Result<PathBuf> {
    Ok(base.join(path))
}
