use std::env;
use std::path::Path;

mod ma;
mod text;

extern crate fstream;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("Error: Too few arguments. Use './Porydelete --help' for more information");
    }

    // Why the length checking? Because if I would run './porydelete --ma --help' if would also execute for '--ma'
    if args.len() == 3 {
        // Checks for help for a specific [OPTIONS]
        if args[2] == "--help" {
            match args[1].as_str() {
                // Matches for which message to print
                "--ma" => println!("{}", text::T_MA_HELP),
                _ => eprintln!("Error: Unknown argument {}", args[1]),
            }
        }
    }

    // Why does it check that? Because it would then not work if there are arguments for the map attributes. See 'map_json_attr.rs'
    // Matches for the first argument after './porydelete'
    match args[1].as_str() {
        // Prints help message constant which is located in text.rs
        "--help" => println!("{}", text::T_PORYDEL_HELP),

        // Executes the part of deleting the specified 'json' attributes in 'map.json'
        "--ma" => {
            // So that --ma wouldnt be executed if 'args.len() == 3'
            if args.len() >= 3 && args[2] != "--help" {
                let target_directory = "./data/maps"; // Directory of where map.json is located
                let path = Path::new(target_directory); // Creates a variable of the type '&Path' which is used to locate 'map.json' and edit it

                if !path.exists() {
                    eprintln!("Error: The directory './data/maps' does not exist.");
                } else if !path.is_dir() {
                    eprintln!("Error: './data/maps' is not a directory");
                } else if let Err(err) = ma::parse_file_and_delete_attribute(&path, &args) {
                    eprintln!("Error: {}", err);
                }
            } else if args.len() == 2 {
                // So that --ma is executed if "args[2] != "--help" but 'args.len() == 2'
                let target_directory = "./data/maps"; // Directory of where map.json is located
                let path = Path::new(target_directory); // Creates a variable of the type '&Path' which is used to locate 'map.json' and edit it

                if !path.exists() {
                    eprintln!("Error: The directory './data/maps' does not exist.");
                } else if !path.is_dir() {
                    eprintln!("Error: './data/maps' is not a directory");
                } else if let Err(err) = ma::parse_file_and_delete_attribute(&path, &args) {
                    eprintln!("Error: {}", err);
                }
            }
        }

        _ => eprintln!("Error: Unknown argument {}", args[1]),
    }
}
