//! A frontend-local admission command using only the neutral contract checker.
#![forbid(unsafe_code)]
use kuiper_contracts::{MAX_MESSAGE, Package, canonical, validate};
use std::io::{self, Read};

fn check() -> Result<String, Box<dyn std::error::Error>> {
    let mut bytes = Vec::new();
    io::stdin()
        .take((MAX_MESSAGE + 1) as u64)
        .read_to_end(&mut bytes)?;
    let package: Package = canonical::parse(&bytes)?;
    validate::package(&package)?;
    Ok(canonical::digest(&package)?)
}
fn main() {
    match check() {
        Ok(digest) => println!("{digest}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
