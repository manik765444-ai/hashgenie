use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use sha2::{Sha256, Digest};
use thiserror::Error;

// Custom error type for better error handling
#[derive(Debug, Error)]
enum HashCalculatorError {
    #[error("File error: {0}")]
    FileError(#[from] io::Error),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

fn calculate_hash<P: AsRef<Path>>(path: P) -> Result<String, HashCalculatorError> {
    let mut file = File::open(&path).map_err(|e| {
        HashCalculatorError::FileError(io::Error::new(
            e.kind(),
            format!("Failed to open file {}: {}", path.as_ref().display(), e),
        ))
    })?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 1024];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn main() -> Result<(), HashCalculatorError> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure the correct number of arguments
    if args.len() != 2 {
        return Err(HashCalculatorError::InvalidInput(
            "Usage: hash_calculator <file_path>".to_string(),
        ));
    }

    let file_path = &args[1];

    // Calculate the hash and handle any errors
    match calculate_hash(file_path) {
        Ok(hash) => {
            println!("SHA-256 hash of file '{}': {}", file_path, hash);
            Ok(())
        }
        Err(e) => Err(e),
    }
}