// Modules
mod args;
mod del_attribute;
mod filter;
mod list;

// Imports
use args::Args;
use clap::Parser;
use filter::PdFilter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments
    let args = Args::parse();
    // Create a filter
    let attr_filer = filter::MaFilter {
        elem: &args,
        start_dir: String::from("./data/maps"),
        dest_dir: String::from("./data/maps/porydelete-filter"),
    };

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
        // Filter command for attributes feature
        "attr-fil" => Ok(attr_filer.do_filter()),
        // Defilter command for attributes feature
        "attr-defil" => Ok(attr_filer.do_defilter()),
        // other cases
        _ => Ok(args.other_case_command()),
    }
}
