// This file contains all the code of deleting maps. Note that even tho you succesfully removed a file that way, there may be some other files causing erros when 
// compiling. So prepare yourself for fixing errors.
//
// Steps of deleting a map <target>:
//-----------------------------------------------------------------------------------------------------------------------------------------------------
//   1   Remove map connections from other maps connecting to <target>. 
//   2   Delete './data/maps/<target>'                                                              done
//   3   Delete './data/layouts/<target>'                                                           done
//   4   In './data/event_scripts.s' remove line '.include "data/maps/<target>/scripts.inc"'        done
//   5   In './data/layouts/layouts.json' remove attribute containing 'LAYOUT_<target>' (note: for LittlerootTest it would be: 'LAYOUT_LITTLEROOT_TEST')
//   6   In './data/maps/map_groups.json' in map-group, delete <target>
//-----------------------------------------------------------------------------------------------------------------------------------------------------

use std::path::{Path, PathBuf};
use std::fs::{self, File};

// There are different map name strings we need for different deletion processes (see from line 5).
// With 'n_which' we specify how the function should process 'n_map' to get the string we need.
fn get_path(n_map: &String, dirs_string: &Vec<String>, n_for_which_step: u8) -> Option<std::path::PathBuf> {
    match n_for_which_step {
        2 => {
            let map_as_path = Path::new(&n_map);
            let t_data_maps_dir = Path::new(&dirs_string[0]).join(map_as_path);
            return Some(t_data_maps_dir);
        },
        3 => {
            let map_as_path = Path::new(&n_map);
            let t_data_layouts_dir = Path::new(&dirs_string[1]).join(map_as_path);
            return Some(t_data_layouts_dir);
        },
        _ => {
            return None;
        }
    }
}

fn camel_to_snake_with_map_prefix(input: &String) -> String {
    let mut result = String::new();
    let mut prev_char: Option<char> = None;

    for current_char in input.chars() {
        if let Some(prev) = prev_char {
            // Check if the current character is uppercase and the previous character is not an underscore
            if current_char.is_uppercase() && prev != '_' {
                result.push('_');
            }
        }

        result.push(current_char.to_ascii_uppercase());
        prev_char = Some(current_char);
    }
    result = String::from("MAP_") + &result;
    result
}

fn delete_object(args: &Vec<String>, dir: &Path, n_map: String) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        if dir.is_dir() {
            let entry = entry?;
            let path = entry.path();

            if path.file_name().and_then(|n| n.to_str()) == Some("map.json") {
                let contents = fs::read_to_string(&path)?;
                let mut map: serde_json::Value = serde_json::from_str(&contents).expect("Error: failed to parse map.json");
                
                if let Some(array) = map.as_array_mut() {
                    // Remove objects with "id": "test"
                    array.retain(|item| {
                        if let Some(obj) = item.as_object() {
                            if let Some(id) = obj.get("map") {
                                return id != n_map.as_str();
                            }
                        }
                        true
                    });
                    let modified_json = serde_json::to_string_pretty(&map).expect("Error: failed to serialize map.json");
                    fstream::write_text(path, modified_json, true).unwrap();
                } else {
                    // Handle the case where the JSON data is not an array
                    eprintln!("Error: map.json is not an array.");
                }
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
    println!("Success: Deleted all map connecions to {}!", args[2]);
    Ok(())
}

pub fn parse_file_and_delete_map(
    dirs_string: &Vec<String>,
    args: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {

    // Step 1 (untested)
    {
       let t_data_maps_dir = Path::new(&dirs_string[0]);
       let n_map = camel_to_snake_with_map_prefix(&args[2]);
       delete_object(args, &t_data_maps_dir, n_map)?;
    }   
    
    // Step 2
    {
        let t_data_maps_dir = get_path(&args[2], dirs_string, 2).unwrap();    // creates path to the map which is going to be deleted (for step 2)

        if t_data_maps_dir.exists() && t_data_maps_dir.is_dir() {
            fs::remove_dir_all(&t_data_maps_dir)?;
            if !t_data_maps_dir.exists() {
                println!("Success: '{}' successfully deleted!", t_data_maps_dir.display());
            }
        } else if !t_data_maps_dir.exists() {
            eprintln!("Warning: '{}' doesn't exist! This however doesn't affect the map deletion process!", t_data_maps_dir.display());
        }
    }

    // Step 3
    {
        let t_data_layouts_dir = get_path(&args[2], dirs_string, 3).unwrap();
        
        if t_data_layouts_dir.exists() && t_data_layouts_dir.is_dir() {
            fs::remove_dir_all(&t_data_layouts_dir)?;
            if !t_data_layouts_dir.exists() {
                println!("Success: '{}' successfully deleted!", t_data_layouts_dir.display());
            }
        } else if !t_data_layouts_dir.exists() {
            eprintln!("Warning: '{}' doesn't exist! This however doesn't affect the map deletion process!", t_data_layouts_dir.display());
        }
    }

    // Step 4
    {
        let include_string1 = String::from(".include \"data/maps/");
        let include_string2 = "/scripts.inc\"";
        let string_to_delete = include_string1 + &args[2].as_str() + include_string2; 
        let path = Path::new(&dirs_string[2]);

        if path.exists() {
            let mut content_event_scripts_s = fs::read_to_string(&dirs_string[2])?;
            content_event_scripts_s = content_event_scripts_s.replace(&string_to_delete, "");
            fs::write(&dirs_string[2], content_event_scripts_s)?;
            println!("Success: Include in '{}' successfully deleted!", dirs_string[2]);
        } else {
            eprintln!("Fatal Error: '{}' doesn't exist! Without 'event_scripts.s', your project cannot compile!", path.display());
        }
    }
    Ok(())
}