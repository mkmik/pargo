use clap::Parser;

/// This struct defines the command line arguments we accept.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Opts {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    /// Dump command to process a file
    Dump {
        /// The filename to process
        #[clap(required = true)]
        filename: String,
    },
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.command {
        Commands::Dump { filename } => {
            // Implement the logic for the dump command here
            println!("Processing file: {}", filename);
        }
    }
}
