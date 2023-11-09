use clap::{Parser, Arg};

/// This struct defines the command line arguments we accept.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Opts {
    /// An example option, use more meaningful names and descriptions for your application.
    #[clap(short, long)]
    example_option: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Use the options here
    println!("The provided example option is: {}", opts.example_option);
}
