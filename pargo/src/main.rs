use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("An example error occurred")]
    ExampleError,
}

// Type alias for Result with the custom Error type
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    println!("Hello, pargo!");
    // Here you can use your custom error type with `?` or return it explicitly
    // For demonstration purposes, let's return an error
    Err(Error::ExampleError)
}
