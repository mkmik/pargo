use clap::Parser;
use thiserror::Error;

#[derive(Parser, Error, Debug)]
pub enum Error {
    #[error("An example error occurred")]
    ExampleError,
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

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    match opts.command {
        Commands::Build {} => todo!(),
        Commands::Run {} => {
            println!("TODO");
        }
    }

    Ok(())
}
