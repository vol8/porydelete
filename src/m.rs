// This file contains all the code of deleting maps. Note that even tho you succesfully removed a file that way, there may be some other files causing erros when
// compiling. So prepare yourself for fixing errors.
//
// Steps of deleting a map <target>:
//-----------------------------------------------------------------------------------------------------------------------------------------------------
//   1   Remove map connections from other maps connecting to <target>.                             done
//   2   Delete './data/maps/<target>'                                                              done
//   3   Delete './data/layouts/<target>'                                                           done
//   4   In './data/event_scripts.s' remove line '.include "data/maps/<target>/scripts.inc"'        done
//   5   In './data/layouts/layouts.json' remove attribute containing 'LAYOUT_<target>' (note: for LittlerootTest it would be: 'LAYOUT_LITTLEROOT_TEST') done
//   6   In './data/maps/map_groups.json' in map-group, delete <target>                             done
//-----------------------------------------------------------------------------------------------------------------------------------------------------

use std::fs;
use std::path::Path;

// There are different map name strings we need for different deletion processes (see from line 5).
// With 'n_which' we specify how the function should process 'n_map' to get the string we need.
fn get_path(
    n_map: &String,
    dirs_string: &Vec<String>,
    n_for_which_step: u8,
) -> Option<std::path::PathBuf> {
    match n_for_which_step {
        2 => {
            let map_as_path = Path::new(&n_map);
            let t_data_maps_dir = Path::new(&dirs_string[0]).join(map_as_path);
            return Some(t_data_maps_dir);
        }
        3 => {
            let map_as_path = Path::new(&n_map);
            let t_data_layouts_dir = Path::new(&dirs_string[1]).join(map_as_path);
            return Some(t_data_layouts_dir);
        }
        _ => {
            return None;
        }
    }
}

// Converts a camel case name (like the maps) into a snake case name. Its primary us is for step 1 and step 5.
// See the top of this file for more information.
fn get_snake_name(input: &String, which: u8) -> String {
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
    match which {
        1 => {
            result = String::from("MAP_") + &result;
        }
        2 => {
            result = String::from("LAYOUT_") + &result;
        }
        _ => {}
    }
    result
}

// This is the function used for the step 1 deletion process.
// Look at the top of the file for more information.
fn s1_delete_object(
    args: &Vec<String>,
    dir: &Path,
    n_map: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("this is todo");
        /*for entry in fs::read_dir(dir)? {
            if dir.is_dir() {
            let entry = entry?;
            let path_to_subdir = &entry.path();
            for subentry in fs::read_dir(&path_to_subdir)? {
                let entry = subentry?;
                let path_to_map_json = &entry.path();

                if path_to_map_json.file_name().and_then(|n| n.to_str()) == Some("map.json") {
                    let contents = fs::read_to_string(&path_to_map_json)?;
                    let mut map: serde_json::Value =
                        serde_json::from_str(&contents)?;

                    if let Some(connections) = map.get_mut("connections") {
                        if let Some(connections_obj) = connections.as_object_mut() {
                            // Remove the specific object with "map" equal to map_to_delete
                            if connections_obj.contains_key(n_map) {
                                connections_obj.remove(n_map);
                            }

                            let modified_json = serde_json::to_string_pretty(&map)?;
                            fs::write(path_to_map_json, modified_json)?;
                        } else {
                            eprintln!("Error: 'connections' in map.json is not an object.");
                        }
                    }
                }
            }
        }
    }
    println!("Success: Deleted all map connections to {}!", n_map);*/
    Ok(())
}

// This is the function used for the step 5 deletion process.
// Look at the top of the file for more information.
fn s5_delete_object(
    args: &Vec<String>,
    dir: &Path,
    n_map: String,
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_file() {
        if dir.file_name().and_then(|n| n.to_str()) == Some("layouts.json") {
            let contents = fs::read_to_string(&dir)?;
            let mut map: serde_json::Value =
                serde_json::from_str(&contents).expect("Error: failed to parse map.json");

            if let Some(layouts) = map.get_mut("layouts") {
                // If "layouts" is an array, filter out the object with id "LAYOUT_RUSTBORO_CITY"
                if let Some(layouts_array) = layouts.as_array_mut() {
                    layouts_array.retain(|layout| {
                        if let Some(layout_id) = layout.get("id") {
                            return layout_id != n_map.as_str();
                        }
                        true
                    });
                }
                let modified_json = serde_json::to_string_pretty(&map)
                    .expect("Error: failed to serialize map.json");
                println!("Success: Deleted layouts of {}!", args[2]);
                fstream::write_text(dir, modified_json, true).unwrap();
            } else {
                // Handle the case where the JSON data is not an array
                eprintln!("Error: layouts.json is not an array.");
            }
        }
    }
    Ok(())
}

// This is the function used for the step 6 deletion process.
// Look at the top of the file for more information.
fn s6_delete_object(
    args: &Vec<String>,
    dir: &Path,
    n_map: String,
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_file() {
        if dir.file_name().and_then(|n| n.to_str()) == Some("map_groups.json") {
            let contents = fs::read_to_string(&dir)?;
            let mut map: serde_json::Value =
                serde_json::from_str(&contents).expect("Error: failed to parse map.json");

            if let Some(array) = map.pointer_mut("/gMapGroup_TownsAndRoutes") {
                // Check if the array is an array and not null
                if let Some(array) = array.as_array_mut() {
                    // Remove "Route123" from the array
                    array.retain(|item| item != n_map.as_str());
                }
                let modified_json = serde_json::to_string_pretty(&map)
                    .expect("Error: failed to serialize map.json");
                println!("Success: Updated map_groups.json of {}!", args[2]);
                fstream::write_text(dir, modified_json, true).unwrap();
            } else {
                // Handle the case where the JSON data is not an array
                eprintln!("Error: map_groups.json is not an array.");
            }
        }
    }
    Ok(())
}

// This is the function that is responsible for the general deletion process.
// It has all the steps included which you can look up on the top of this file.
pub fn parse_file_and_delete_map(
    dirs_string: &Vec<String>,
    args: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Step 1 not working
    {
        let t_data_maps_dir = Path::new(&dirs_string[0]);
        let n_map = get_snake_name(&args[2], 1);
        s1_delete_object(args, &t_data_maps_dir, &n_map)?;
    }

    // Step 2 works
    {
        let t_data_maps_dir = get_path(&args[2], dirs_string, 2).unwrap(); // creates path to the map which is going to be deleted (for step 2)

        if t_data_maps_dir.exists() && t_data_maps_dir.is_dir() {
            fs::remove_dir_all(&t_data_maps_dir)?;
            if !t_data_maps_dir.exists() {
                println!(
                    "Success: '{}' successfully deleted!",
                    t_data_maps_dir.display()
                );
            }
        } else if !t_data_maps_dir.exists() {
            eprintln!("Warning: '{}' doesn't exist! This however doesn't affect the map deletion process!", t_data_maps_dir.display());
        }
    }

    // Step 3 works
    {
        let t_data_layouts_dir = get_path(&args[2], dirs_string, 3).unwrap();

        if t_data_layouts_dir.exists() && t_data_layouts_dir.is_dir() {
            fs::remove_dir_all(&t_data_layouts_dir)?;
            if !t_data_layouts_dir.exists() {
                println!(
                    "Success: '{}' successfully deleted!",
                    t_data_layouts_dir.display()
                );
            }
        } else if !t_data_layouts_dir.exists() {
            eprintln!("Warning: '{}' doesn't exist! This however doesn't affect the map deletion process!", t_data_layouts_dir.display());
        }
    }

    // Step 4 works
    {
        let include_string1 = String::from(".include \"data/maps/");
        let include_string2 = "/scripts.inc\"";
        let string_to_delete = include_string1 + &args[2].as_str() + include_string2;
        let path = Path::new(&dirs_string[2]);

        if path.exists() {
            let mut content_event_scripts_s = fs::read_to_string(&dirs_string[2])?;
            content_event_scripts_s = content_event_scripts_s.replace(&string_to_delete, "");
            fs::write(&dirs_string[2], content_event_scripts_s)?;
            println!(
                "Success: Include in '{}' successfully deleted!",
                dirs_string[2]
            );
        } else {
            eprintln!("Fatal Error: '{}' doesn't exist! Without 'event_scripts.s', your project cannot compile!", path.display());
        }
    }

    // Step 5 works
    {
        let t_data_layouts_json_dir = Path::new(&dirs_string[3]);
        let n_map = get_snake_name(&args[2], 2);
        s5_delete_object(args, &t_data_layouts_json_dir, n_map)?;
    }

    // Step 6 not working
    {
        let t_data_maps_dir = Path::new(&dirs_string[4]);
        let n_map = &args[2];
        s6_delete_object(args, t_data_maps_dir, n_map.to_string())?;
    }
    println!("\nWarning: This map deletion process is now complete. However there may be some errors when compiling your project.\nThese need to be fixed manually.\n\n");
    Ok(())
}
