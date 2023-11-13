use clap::Parser;
use pargo::conf::Config;
use std::path::PathBuf;
use thiserror::Error;
use toml;

#[derive(Error, Debug)]
pub enum Error {
    #[error("An IO error occurred: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Toml decode error: {0}")]
    TomlDecodeError(#[from] toml::de::Error),
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

fn read_conf(path: PathBuf) -> Result<Config> {
    let config_path = path.join("Pargo.toml");
    let config_contents = fs::read_to_string(config_path)?;
    let config = toml::from_str(&config_contents)?;
    Ok(config)
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let conf = read_conf(std::env::current_dir()?)?;

    match opts.command {
        Commands::Build {} => todo!(),
        Commands::Run {} => {
            println!("TODO...{:?}", conf)
        }
    }

    Ok(())
}
