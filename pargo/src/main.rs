use clap::Parser;
use pargo::conf::Program;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("An IO error occurred: {0}")]
    IoError(#[from] std::io::Error),
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

fn read_conf(_path: PathBuf) -> Result<Program> {
    todo!()
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
