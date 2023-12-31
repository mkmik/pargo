use clap::Parser;
use pargo::{conf::Config, env::Env};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("An IO error occurred: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Toml decode error: {0}")]
    TomlDecodeError(#[from] toml::de::Error),

    #[error("File not found: {0}")]
    FileNotFoundError(PathBuf),
}

// Type alias for Result with the custom Error type
type Result<T, E = Error> = std::result::Result<T, E>;

/// This struct defines the command line arguments we accept.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Opts {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    /// Build an binary
    #[clap(name = "build")]
    Build {},

    /// Build a binary and load the binary in the simulator
    #[clap(name = "run")]
    Run {},
}

use std::fs;

fn read_conf(path: impl AsRef<Path>) -> Result<Config> {
    let config_path = path.as_ref().join("Pargo.toml");
    let config_contents = fs::read_to_string(config_path)?;
    Ok(toml::from_str(&config_contents)?)
}

// Function to create the environment
fn create_env() -> Result<Env> {
    let base_dir = std::env::current_dir()?;
    let config = read_conf(&base_dir)?;
    Ok(Env { config, base_dir })
}

fn build(env: &Env) -> Result<()> {
    let src_dir = env.src_dir();
    let asm_file_path = src_dir.join("main.asm");
    if !asm_file_path.exists() {
        return Err(Error::FileNotFoundError(asm_file_path));
    }
    let build_dir = env.build_dir()?;
    println!("build dir: {}", build_dir.display());
    // TODO: Implement the actual build logic here
    Ok(())
}

fn run(_env: &Env) -> Result<()> {
    Ok(())
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let env = create_env()?;

    match opts.command {
        Commands::Build {} => build(&env)?,
        Commands::Run {} => {
            build(&env)?;
            run(&env)?;
        }
    }

    Ok(())
}
