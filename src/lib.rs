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


pub fn create_database(config_path: Option<&str>) -> Result<()> {
    println!("\nCreate database.");
    println!("{config_path:?}");
    Ok(())
}


pub fn drop_database(config_path: Option<&str>) -> Result<()> {
    println!("\nDrop database.");
    println!("{config_path:?}");
    Ok(())
}


pub fn create_tables(template_name: &str, config_path: Option<&str>, force: bool) -> Result<()> {
    println!("\nCreate DB tables for the template: `{template_name}`.");
    println!("{config_path:?}");
    println!("{force}");
    Ok(())
}


pub fn drop_tables(template_name: &str, config_path: Option<&str>) -> Result<()> {
    println!("\nDrop DB tables for the template: `{template_name}`.");
    println!("{config_path:?}");
    Ok(())
}
