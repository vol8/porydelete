use std::path::Path;

// Imports
use clap::Parser;
use porydelete::filter::PdFilter;
use porydelete::*;

// Arguments which need to be passed to the program.
#[derive(Parser)]
pub struct Args {
    // Command to execute
    pub command: String,
    // Values to pass to the command
    pub value: String,
}

impl Args {
    pub fn other_case_command(&self) {
        eprintln!(
            "Command '{}' is not an available command. Use '--help' for more information.",
            self.command
        );
    }

    pub fn other_case_value(&self) {
        eprintln!(
            "Value '{}' is not an available command. Use '--help' for more information.",
            self.command
        );
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments
    let args = Args::parse();
    let location_check_path = Path::new("./data");
    // Create a filter
    let attr_filter = filter::MaFilter {
        elem: args.value.clone(),
        start_dir: String::from("./data/maps"),
        dest_dir: String::from("./data/maps/porydelete-filter"),
    };

    if location_check_path.exists() {
        // Run command
        match args.command.as_str() {
            // Delete an attribute
            "attr" => del_attribute::execute_del(&args.value),
            // Delete a map
            "map" => del_map::execute_del(&args.value),
            // Delete a tileset
            "tileset" => del_tileset::execute_del(&args.value),
            // Delete tileset animations seperatly.
            "tileset-anims" => Ok(()),
            // Delete a script
            "script" => Ok(()),
            // Delete a pokemon
            "pkmn" => Ok(()),
            // Delete an item
            "item" => Ok(()),
            // List an object
            "list" => list::list_for_value(&args.command, &args.value),
            // Filter command for attributes feature
            "attr-fil" => Ok(attr_filter.do_filter()),
            // Defilter command for attributes feature
            "attr-defil" => Ok(attr_filter.do_defilter()),
            // other cases
            _ => Ok(args.other_case_command()),
        }
    } else {
        println!("Fatal Error: This executable is not located in the root of any 3rd generation decompilation projects!");
        Ok(())
    }
}
