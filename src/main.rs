// Modules
mod args;
mod del_attribute;
mod list;

// Imports
use args::Args;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments
    let args = Args::parse();

    // Run command
    match args.command.as_str() {
        // Delete an attribute
        "attr" => del_attribute::execute_ma(&args),
        // Delete a map
        "map" => Ok(()),
        // Delete a tileset
        "tileset" => Ok(()),
        // Delete a script
        "script" => Ok(()),
        // Delete a pokemon
        "pkmn" => Ok(()),
        // Delete an item
        "item" => Ok(()),
        // List an object
        "list" => list::list_for_value(&args),
        // other cases
        _ => Ok(args.other_case_command()),
    }
}
