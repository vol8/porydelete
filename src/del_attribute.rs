use crate::args::Args;

use indexmap::IndexMap;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

// Prints the result of maps edited. Missing: Maps which failed and the path should be printed as well.
fn print_result(
    c_success: &mut i32,
    c_failed: &mut i32,
    v_failed: &mut Vec<String>,
    c_total_maps: &mut i32,
) {
    println!(
        "Modified {} out of {} maps successfully!",
        *c_success, *c_total_maps
    );
    println!("Maps failed to modify: {}", *c_failed);
    if *c_failed > 0 {
        println!("In these maps, 'map.json' was missing:");
        for map in v_failed {
            println!("  {}", map);
        }
    }
}

// Not really removes it but if replaces the key in the map with the new value.
fn remove_attribute(args: &Args, map: &mut IndexMap<String, Value>, c_success: &mut i32) {
    if args.value.as_str() == "connections" {
        map.insert(args.value.clone(), serde_json::Value::Null);
        *c_success += 1;
    } else {
        map.insert(args.value.clone(), json!([]));
        *c_success += 1;
    }
}

// Enters the subdir and removes the attribute.
fn enter_subdir_and_delete(
    subdir_path: &PathBuf,
    args: &Args,
    c_success: &mut i32,
    c_failed: &mut i32,
    v_failed: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Every map is a directory, so here it needs to check if it is one
    if subdir_path.is_dir() {
        // Is the name of the subdirectory which is used to create the path to check if map.json exists.
        let subdir_name = subdir_path.file_name().unwrap().to_str().unwrap();
        // Creates the path to the map.json file with 'subdir_name' as the name of the subdirectory.
        let full_dir_name_as_string = format!("./data/maps/{}/map.json", subdir_name);
        let full_dir_name = Path::new(&full_dir_name_as_string);

        // We check if map.json exists for each map
        if full_dir_name.exists() {
            // enters subdir
            for file in fs::read_dir(&subdir_path)? {
                // creates path to file in subdir
                let file = file?.path();

                // Checks if the file is not a directory
                if !file.is_dir() {
                    // Checks if the file is 'map.json'
                    if file.file_name().and_then(|n| n.to_str()) == Some("map.json") {
                        // read map.json and store to a string
                        let contents =
                            fstream::read_text(&file).expect("Error: Failed to read 'map.json'");
                        // create IndexMap from string
                        let mut map: IndexMap<String, Value> = serde_json::from_str(&contents)
                        .expect("Error: Failed to correctly read IndexMap of 'map.json', check for any last changes you made to any map!");
                        // remove attribute
                        remove_attribute(args, &mut map, c_success);
                        // Overwrite changes to map.json
                        let modified_json = serde_json::to_string_pretty(&map).expect(
                            "Error: Failed to correctly deserialize IndexMap of 'map.json'.",
                        );
                        fstream::write_text(&file, modified_json, true)
                            .expect("Error: Failed to write to 'map.json'");
                    }
                }
            }
        } else {
            *c_failed += 1;
            v_failed.push(full_dir_name.to_str().unwrap().to_string());
        }
    }
    Ok(())
}

// It searches for subdirectories in the maps folder and then uses 'enter_subdir_and_delete' to enter the subdirectory and deletes attributes.
pub fn execute_ma(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut c_success = 0;
    let mut c_failed = 0;
    let mut v_failed: Vec<String> = vec![];
    let mut c_total_maps = 0;

    let is_attribute = args.value.as_str() == "connections"
        || args.value.as_str() == "object_events"
        || args.value.as_str() == "bg_events"
        || args.value.as_str() == "warp_events"
        || args.value.as_str() == "coord_events";

    if is_attribute {
        // Path of map folder where map.json is located.
        let m_path = Path::new("./data/maps/");
        if m_path.is_dir() {
            for entry in fs::read_dir(m_path)? {
                let subdir_path = entry?.path(); // Create path to subdir.
                enter_subdir_and_delete(
                    &subdir_path,
                    args,
                    &mut c_success,
                    &mut c_failed,
                    &mut v_failed,
                )?;
                // counts for total maps
                if subdir_path.is_dir() {
                    c_total_maps += 1;
                }
            }
            print_result(
                &mut c_success,
                &mut c_failed,
                &mut v_failed,
                &mut c_total_maps,
            ); // Prints the result of maps edited.
        } else {
            eprintln!("Error: Directory '{}' not found, make sure that the executable is located in the root of your 3rd Generation project!", m_path.display());
        }
    } else if !is_attribute {
        eprintln!("Error: The given attribute is not a valid attribute, use '--help' for more information.");
    }
    Ok(())
}
