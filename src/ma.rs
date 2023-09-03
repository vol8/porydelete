#![allow(unused_assignments)]
use indexmap::IndexMap;
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};

fn overwrite_json_with_changes(map: &IndexMap<String, Value>, path: &Path) {
    // converts it back to a string
    let map_str = serde_json::to_string_pretty(&map).unwrap();
    // writes data to map.json
    fstream::write_text(path, map_str, true).unwrap();
}

fn remove_attributes(
    map: &mut IndexMap<String, Value>,
    args: &Vec<String>,
    c_change_counter: &mut i16,
) {
    let keys_to_remove = vec![
        "connections",
        "object_events",
        "warp_events",
        "bg_events",
        "coord_events",
    ];

    let remove_all = args.len() > 2 && args[2] == "-a";

    // todo: Fix result message for -a
    if remove_all {
        for key in &keys_to_remove {
            if key.to_string() == "connections" {
                map.insert(key.to_string(), serde_json::Value::Null);
                *c_change_counter += 1;
            } else {
                map.insert(key.to_string(), json!([]));
                *c_change_counter += 1;
            }
        }
        return;
    }

    for i in 2..args.len() {
        if let Some(key) = keys_to_remove.iter().find(|&k| args[i] == *k) {
            if key.to_string() == "connections" {
                map.insert(key.to_string(), serde_json::Value::Null);
                *c_change_counter += 1;
            } else {
                map.insert(key.to_string(), json!([]));
                *c_change_counter += 1;
            }
        }
    }
}

fn enter_subdir_and_execute_deletion(
    dir: &Path,
    args: &Vec<String>,
    c_change: &mut i16,
    c_failed: &mut i16,
    maps_failed: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            // Subdir
            let entry = entry?;
            // Path to Subdir
            let path = entry.path();
            if path.file_name().and_then(|n| n.to_str()) == Some("map.json") {
                let text = fstream::read_text(&path).expect("Error: Failed to read 'map.json'");
                let mut map = serde_json::from_str(&text)
                    .expect("Error: Failed to correctly read IndexMap of 'map.json', check for any last changes you made to any map!");
                remove_attributes(&mut map, args, c_change);
                overwrite_json_with_changes(&map, &path);
            } else if path.file_name().and_then(|n| n.to_str()) == Some("scripts.inc") {
                continue;
            } else if path.file_name().and_then(|n| n.to_str()) == Some("connections.inc") {
                continue;
            } else if path.file_name().and_then(|n| n.to_str()) == Some("events.inc") {
                continue;
            } else if path.file_name().and_then(|n| n.to_str()) == Some("header.inc") {
                continue;
            }
        }
    }
    Ok(())
}

// ISSUE FOR COUNTING CHANGES: Recursion! Because the functin jsut creates a new variable to store the counting
pub fn parse_file_and_delete_attribute(
    dir: &Path,
    args: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {

// COLLECTING RESULT DATA
    let mut change_count = 0; // counter for successful changes
    let mut failed_count = 0; // counter for failed changes
    let mut failed_maps: Vec<PathBuf> = Vec::new(); // Array which holds map paths of all changes which failed for that map
    let mut selected_attributes: Vec<String> = vec![]; // Array which holds all attributes selected via arguments
    let available_attributes = vec![
        // Array which holds all available attributes
        "connections",
        "object_events",
        "warp_events",
        "bg_events",
        "coord_events",
        "-a",
    ];
    for i in 2..args.len() {
        let mut found_arg = false;
        if let Some(key) = available_attributes.iter().find(|&k| args[i] == *k) {
            selected_attributes.push(key.to_string());
            found_arg = true;
        } else if found_arg == false {
            eprintln!("Error: Unknown argument `{}`", args[i]);
        }

        // we only want this to be printed once when we have an wrong argument, even if we have multiple wrong arguments
        if i == args.len() - 1 && found_arg == false {
            eprintln!("Use './porydelete --ma --help' for more information.");
        }
    }

// MAP-ATTRIBUTE MODIFICATION EXECUTION
    if dir.is_dir() && args.len() >= 3 {
        // checks if there are at least 3 arguments, since thats needed to modify something

        // ALGORITHM
        for entry in fs::read_dir(dir)? {
            // Subdir
            let entry = entry?;
            // Path to Subdir
            let path = entry.path();

            if path.is_dir() {
                // 'porydelete-filter'
                if path.file_name().and_then(|n| n.to_str()) == Some("porydelete-filter") {
                    continue;
                }
                enter_subdir_and_execute_deletion(
                    &path,
                    args,
                    &mut change_count,
                    &mut failed_count,
                    &mut failed_maps,
                )?;
            }
        }
    }

// RESULT PRINTING
    println!("\nResult:");
    if selected_attributes.len() == 0 {
        selected_attributes.push(String::from("None"));
    }
    
    if selected_attributes[0] != String::from("-a") {
        println!(
            "  Maps successfully modified: {}",
            change_count / selected_attributes.len() as i16
        ); // The devision is there because every deletion per map, counts the counter so for 3 attributes, the final count is 3 times as high
        println!("  Map-Attributes:");
        for ma in selected_attributes.iter() {
            println!("      {}", ma);
        }
    } else if selected_attributes[0] == String::from("-a") {
        println!("  Maps successfully modified: {}", change_count / 5);
        println!("  Map-Attributes:");
        println!("      connections");
        println!("      object_events");
        println!("      warp_events");
        println!("      bg_events");
        println!("      coord_events");
    }

    if failed_maps.len() >= 1 && selected_attributes[0] != String::from("None") {
        // If there is any failure and theres at least one available argument, do that
        println!("  Maps failed to read: {}, Path(s):", failed_count);
        for maps in failed_maps {
            // Prints path of every map failed to read because 'map.json' is not available
            println!("      {}", maps.display().to_string());
        }
        println!("\nFilename is not 'map.json'! Try to rename and then proceed.");
    } else {
        println!("  Maps failed to read: 0"); // Why 0 instead of 'failed_count'? Because it doesn't need to be printed because it would just confuse other while printing. Why? Because the paths aren't printed.
    }
    Ok(())
}
