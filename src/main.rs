use std::env;
use std::path::Path;
mod map_json_attr;
extern crate fstream;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Too few arguments. Use `./Porydelete --help` for more information");
    } else if args[1] == "--help" {
        println!("Porydelete help: \n");
        println!("--MAPATTR or --ma        deletes mapattributes in map.json");
    } else if args[1] == "--MAPATTR" || args[1] == "--ma"{
        let target_directory = "./data/maps"; // Directory of maps where map.json is

        let target_path = Path::new(target_directory);
        if !target_path.exists() {
            eprintln!("Error: The directory `./data/maps` does not exist.");
        } else if !target_path.is_dir() {
            eprintln!("Error: `./data/maps` is not a directory");
        } else if let Err(err) = map_json_attr::process_file_and_delete_attribute(&target_path, &args) {
            eprintln!("Error: {}", err);
        }
    } 

}