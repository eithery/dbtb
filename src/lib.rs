pub mod cli;
mod config;
mod env;
mod errors;
mod std;

use ::std::path::Path;
pub use crate::std::result::Result;


pub fn build_databases(
    source: Option<&Path>,
    host: Option<&str>,
    database: Option<&str>,
    user: Option<&str>,
    password: Option<&str>,
    all: bool,
    force: bool
) -> Result<()> {
    println!("\nBuild databases.");
    println!("{:?}", source.as_ref());
    println!("{:?}", host.as_ref());
    println!("{:?}", database.as_ref());
    println!("{user:?}");
    println!("{password:?}");
    println!("--all: {}", all);
    println!("--force: {}", force);
    Ok(())
}
