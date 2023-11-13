
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PargoError {
    #[error("An example error occurred")]
    ExampleError,
}

fn main() -> Result<(), PargoError> {
    println!("Hello, pargo!");
    // Here you can use your custom error type with `?` or return it explicitly
    // For demonstration purposes, let's return an error
    Err(PargoError::ExampleError.into())
}
