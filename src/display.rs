use colored::*;
use error::*;

pub fn error(error: VSTHostError) {
    println!("{}", format!("Error: {}", error).red());
}

pub fn log(message: &str) {
    println!("{}", message);
}
