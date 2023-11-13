use thiserror::Error;

#[derive(Error, Debug)]
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
    Build {},

    /// Build a binary and load the binary in the simulator
    Run {},
}

fn main() -> Result<()> {
    println!("Hello, pargo!");
    // Here you can use your custom error type with `?` or return it explicitly
    // For demonstration purposes, let's return an error
    Err(Error::ExampleError)
}
