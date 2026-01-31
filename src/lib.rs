pub mod cli;
mod env;
mod errors;
mod std;

pub use std::result::Result;
use ::std::path::Path;


pub fn build_databases<P: AsRef<Path>>(
    source: P,
    database: Option<&str>,
    user: Option<&str>,
    password: Option<&str>,
    all: bool,
    force: bool
) -> Result<()> {
    println!("\nBuild databases.");
    println!("{:?}", source.as_ref());
    println!("{:?}", database.as_ref());
    println!("{user:?}");
    println!("{password:?}");
    println!("--all: {}", all);
    println!("--force: {}", force);
    Ok(())
}
