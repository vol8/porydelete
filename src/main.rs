use std::env;
use std::path::Path;

// Filter module
mod filter;
// Texts
mod text;
// Commands
mod m;
mod ma;
mod ts;

extern crate fstream;

fn main() {
    let args: Vec<String> = env::args().collect();

    // CREATE FILTERS
    // This is the filter for maps when using th option to delete MAP-ATTRIBUTES
    let ma_filter = filter::PdFilter {
        elem: args.clone(),
        start_dir: String::from("./data/maps/"),
        dest_dir: String::from("./data/maps/porydelete-filter"),
    };

    // WRONG AMOUNT OF ARGS
    if args.len() == 1 {
        eprintln!("Error: Too few arguments. Use './porydelete --help' for more information");
    }

    // HELP AND FILTER
    // Why the length checking? Because if I would run './porydelete --ma --help' it would also execute for '--ma'
    if args.len() >= 3 {
        // Checks for help for a specific [OPTIONS]
        if args[2] == "--help" {
            match args[1].as_str() {
                // Matches for which message to print
                "--ma" => println!("{}", text::T_MA_HELP),
                "--m" => println!("{}", text::T_M_HELP),
                "--ts" => println!("{}", text::T_TS_HELP),
                "--s" => println!("{}", text::T_NONE_HELP),
                "--pkmn" => println!("{}", text::T_NONE_HELP),
                "--item" => println!("{}", text::T_NONE_HELP),
                "--l-Us" => println!("{}", text::T_NONE_HELP),
                "--l-s" => println!("{}", text::T_NONE_HELP),
                _ => eprintln!("Error: Unknown argument {}", args[1]),
            }
        } else if args[2] == "--filter" {
            match args[1].as_str() {
                "--ma" => ma_filter.do_filter(),
                _ => eprintln!(
                    "Error: '--filter' cannot be used on this argument '{}'",
                    args[1]
                ),
            }
        } else if args[2] == "--defilter" {
            match args[1].as_str() {
                "--ma" => ma_filter.do_defilter(),
                _ => eprintln!(
                    "Error: '--defilter' cannot be used on this argument '{}'",
                    args[1]
                ),
            }
        }
    }

    // EXECUTE
    // Why does it check that? Because it would then not work if there are arguments for the map attributes. See 'map_json_attr.rs'
    // Matches for the first argument after './porydelete'
    match args[1].as_str() {
        // Prints help message constant which is located in text.rs
        "--help" => println!("{}", text::T_PORYDEL_HELP),

        // Executes the part of deleting the specified 'json' attributes in 'map.json'
        "--ma" => {
            // So that --ma wouldnt be executed if 'args.len() == 3'
            if args.len() == 2
                || (args.len() >= 3
                    && args[2] != "--help"
                    && args[2] != "--filter"
                    && args[2] != "--defilter")
            {
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
