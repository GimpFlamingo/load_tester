use std::{
    error::Error,
    fmt
};

// Change the alias to `Box<dyn error::Error>`.
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
