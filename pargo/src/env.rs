use std::path::{Path, PathBuf};

use crate::conf::Config;

pub struct Env {
    pub config: Config,
    pub base_dir: PathBuf,
}

impl Env {
    pub fn src_dir(&self) -> PathBuf {
        self.base_dir.join("src")
    }

    pub fn build_dir(&self) -> std::io::Result<PathBuf> {
        join(self.target_dir()?, "build")
    }

    pub fn target_dir(&self) -> std::io::Result<PathBuf> {
        join(&self.base_dir, "target")
    }
}

fn join<B: AsRef<Path>, P: AsRef<Path>>(base: B, path: P) -> std::io::Result<PathBuf> {
    let res = base.as_ref().join(path);
    std::fs::create_dir_all(&res)?;
    Ok(res)
}
