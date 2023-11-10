use anyhow::Result;
use clap::Parser;
use std::io::{self};
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error opening file: {0}")]
    FileOpen(#[from] io::Error),
    #[error("Invalid start marker")]
    InvalidStartMarker,
    #[error("Invalid pad")]
    InvalidPad,
    #[error("Invalid checksum")]
    InvalidChecksum,
    #[error("Missing end block")]
    MissingEndBlock,
}

use std::io::Write;

fn dump_lda<W: Write>(buffer: &[u8], writer: &mut W) -> Result<()> {
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
            return Err(Error::InvalidStartMarker.into());
        }
        let block_start = cursor;
        cursor += 1; // Skip start marker

        if buffer[cursor] != 0x00 {
            return Err(Error::InvalidPad.into());
        }
        cursor += 1; // Skip pad

        // Read block header
        let byte_count = buffer[cursor] as usize + ((buffer[cursor + 1] as usize) << 8);
        let load_address = buffer[cursor + 2] as usize + ((buffer[cursor + 3] as usize) << 8);
        cursor += 4;

        // Check for end-of-input condition
        if byte_count == 6 {
            if load_address % 2 == 0 {
                writeln!(writer, "Jump to address: 0x{:04X}", load_address)?;
            } else {
                writeln!(writer, "Not jumping to program right after loading")?;
            }
            break;
        }

        // Read program data
        let block_data = &buffer[block_start..block_start + byte_count]; // Include header in checksum calculation
        let program_data = &block_data[6..];
        cursor += byte_count - 6;

        // Read and verify checksum
        let checksum = buffer[block_start + byte_count];
        cursor += 1;

        // Calculate checksum over the entire block, including the header
        let calculated_checksum: u8 = block_data
            .iter()
            .fold(checksum, |acc, &x| acc.wrapping_add(x));

        // Verify checksum by checking if the calculated checksum is zero
        if calculated_checksum != 0 {
            return Err(Error::InvalidChecksum.into());
        }

        // Print block info
        writeln!(writer, "Block info:")?;
        writeln!(writer, "Byte count: {}", byte_count - 6)?;
        writeln!(writer, "Load address: 0x{:04X}", load_address)?;

        // Print hexdump
        writeln!(writer, "Hexdump:")?;
        for (index, byte) in program_data.iter().enumerate() {
            write!(writer, "{:02X} ", byte)?;
            if (index + 1) % 16 == 0 {
                writeln!(writer)?;
            }
        }
        if program_data.len() % 16 != 0 {
            writeln!(writer)?; // Ensure we end with a newline if not exactly 16 bytes per line
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    match opts.command {
        Commands::Dump { filename } => {
            let buffer = std::fs::read(filename)?;
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            dump_lda(&buffer, &mut handle)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dump_lda() -> Result<()> {
        // Sample data representing a valid LDA dump
        let sample_data = vec![
            0x00, 0x00, // Leader bytes
            0x01, 0x00, // Start marker and pad
            0x0A, 0x00, // Byte count (6 bytes, including header)
            0x02, 0x04, // Load address (0x0402)
            0x01, 0x02, 0x03, 0x4,  // Program data
            0xE5, // Checksum (0x01 + 0x00 + 0x0A + 0x00 + 0x02 + 0x04 + 0x01 + 0x02 + 0x03 + 0x04 = 0x1B; 0x1B + ??? = 0)
        ];

        // Cursor for capturing the output
        let mut output = std::io::Cursor::new(Vec::new());

        // Call the function with the sample data
        dump_lda(&sample_data, &mut output)?;

        // Convert the captured output into a String
        let output_string = String::from_utf8(output.into_inner()).expect("Found invalid UTF-8");

        // Expected output
        let expected_output = "\
            Block info:\n\
            Byte count: 4\n\
            Load address: 0x0402\n\
            Hexdump:\n\
            01 02 03 04 \n";

        // Assert that the output is as expected
        assert_eq!(output_string, expected_output);

        Ok(())
    }

    #[test]
    fn test_dump_lda_without_leader_bytes() -> Result<()> {
        // Sample data representing a valid LDA dump without leader bytes
        let sample_data = vec![
            0x01, 0x00, // Start marker and pad
            0x0A, 0x00, // Byte count (6 bytes, including header)
            0x02, 0x04, // Load address (0x0402)
            0x01, 0x02, 0x03, 0x4,  // Program data
            0xE5, // Checksum (0x01 + 0x00 + 0x0A + 0x00 + 0x02 + 0x04 + 0x01 + 0x02 + 0x03 + 0x04 = 0x1B; 0x1B + ??? = 0)
        ];

        // Cursor for capturing the output
        let mut output = std::io::Cursor::new(Vec::new());

        // Call the function with the sample data
        dump_lda(&sample_data, &mut output)?;

        // Convert the captured output into a String
        let output_string = String::from_utf8(output.into_inner()).expect("Found invalid UTF-8");

        // Expected output
        let expected_output = "\
            Block info:\n\
            Byte count: 4\n\
            Load address: 0x0402\n\
            Hexdump:\n\
            01 02 03 04 \n";

        // Assert that the output is as expected
        assert_eq!(output_string, expected_output);

        Ok(())
    }

    #[test]
    fn test_dump_lda_two_blocks() -> Result<()> {
        // Sample data representing a valid LDA dump
        let sample_data = vec![
            0x00, 0x00, // Leader bytes
            0x01, 0x00, // Start marker and pad
            0x0A, 0x00, // Byte count (6 bytes, including header)
            0x02, 0x04, // Load address (0x0402)
            0x01, 0x02, 0x03, 0x4,  // Program data
            0xE5, // Checksum (0x01 + 0x00 + 0x0A + 0x00 + 0x02 + 0x04 + 0x01 + 0x02 + 0x03 + 0x04 = 0x1B; 0x1B + ??? = 0)
            0x01, 0x00, // Start marker and pad
            0x0A, 0x00, // Byte count (6 bytes, including header)
            0x02, 0x04, // Load address (0x0402)
            0x01, 0x02, 0x03, 0x4,  // Program data
            0xE5, // Checksum (0x01 + 0x00 + 0x0A + 0x00 + 0x02 + 0x04 + 0x01 + 0x02 + 0x03 + 0x04 = 0x1B; 0x1B + ??? = 0)
        ];

        // Cursor for capturing the output
        let mut output = std::io::Cursor::new(Vec::new());

        // Call the function with the sample data
        dump_lda(&sample_data, &mut output)?;

        // Convert the captured output into a String
        let output_string = String::from_utf8(output.into_inner()).expect("Found invalid UTF-8");

        // Expected output
        let expected_output = "\
            Block info:\n\
            Byte count: 4\n\
            Load address: 0x0402\n\
            Hexdump:\n\
            01 02 03 04 \n\
            Block info:\n\
            Byte count: 4\n\
            Load address: 0x0402\n\
            Hexdump:\n\
            01 02 03 04 \n";

        // Assert that the output is as expected
        assert_eq!(output_string, expected_output);

        Ok(())
    }
}
