pub mod cli;
mod env;
mod errors;
mod std;

pub use std::result::Result;


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
