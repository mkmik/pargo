
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PargoError {
    #[error("An example error occurred")]
    ExampleError,
}

fn main() -> Result<()> {
    println!("Hello, pargo!");
    // Here you can use your custom error type with `?` or return it explicitly
    // For demonstration purposes, let's return an error
    Err(PargoError::ExampleError.into())
}
// Type alias for Result with the custom PargoError type
type Result<T> = std::result::Result<T, PargoError>;
