use std::path::{Path, PathBuf};

use crate::conf::Config;

pub struct Env {
    pub config: Config,
    target_dir: PathBuf,
}

impl Env {
    pub fn build_dir(&self) -> std::io::Result<PathBuf> {
        join(&self.target_dir, "build")
    }

    pub fn target_dir(&self) -> std::io::Result<PathBuf> {
        join(&self.target_dir, "build")
    }
}

fn join<B: AsRef<Path>, P: AsRef<Path>>(base: B, path: P) -> std::io::Result<PathBuf> {
    let res = base.as_ref().join(path);
    std::fs::create_dir_all(&res)?;
    Ok(res)
}
