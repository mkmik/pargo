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

use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() -> io::Result<()> {
    let opts: Opts = Opts::parse();

    match opts.command {
        Commands::Dump { filename } => {
            let mut file = File::open(&filename)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            let mut cursor = 0;
            while cursor < buffer.len() {
                // Skip leader bytes (0x00)
                while cursor < buffer.len() && buffer[cursor] == 0x00 {
                    cursor += 1;
                }

                // Expecting a block start or end of file
                if cursor >= buffer.len() {
                    break;
                }

                if buffer[cursor] != 0x01 {
                    return Err(io::Error::new(ErrorKind::InvalidData, "Invalid start marker"));
                }
                cursor += 1; // Skip start marker

                if buffer[cursor] != 0x00 {
                    return Err(io::Error::new(ErrorKind::InvalidData, "Invalid pad"));
                }
                cursor += 1; // Skip pad

                // Read block header
                let byte_count = buffer[cursor] as usize + ((buffer[cursor + 1] as usize) << 8);
                let load_address = buffer[cursor + 2] as usize + ((buffer[cursor + 3] as usize) << 8);
                cursor += 4;

                // Check for end-of-input condition
                if byte_count == 6 {
                    if load_address % 2 == 0 {
                        // Jump to address (not implemented)
                    } else {
                        // Halt (not implemented)
                    }
                    break;
                }

                // Read program data
                let program_data = &buffer[cursor..cursor + byte_count - 6];
                cursor += byte_count - 6;

                // Read and verify checksum
                let checksum = buffer[cursor];
                cursor += 1;

                // TODO: Calculate and verify checksum, print block info and hexdump
            }

            Ok(())
        }
    }
}
