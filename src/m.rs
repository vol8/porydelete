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

use std::path::Path;
use std::fs::{self, File};

// There are different map name strings we need for different deletion processes (see from line 5).
// With 'n_which' we specify how the function should process 'n_map' to get the string we need.
fn get_path(n_map: &String, dirs_string: &Vec<String>, n_which: u8) -> Option<std::path::PathBuf> {
    match n_which {
        
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

// Does step one of the map deletion process.
fn remove_map_connection(dirs_string: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn parse_file_and_delete_map(
    dirs_string: &Vec<String>,
    args: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {

    // Step 1


    
    // Step 2
    {
        let t_data_maps_dir = get_path(&args[2], dirs_string, 2).unwrap();    // creates path to the map which is going to be deleted (for step 2)

        if t_data_maps_dir.exists() && t_data_maps_dir.is_dir() {
            fs::remove_dir_all(&t_data_maps_dir)?;
            if !t_data_maps_dir.exists() {
                println!("Success: '{}' successfully deleted!", t_data_maps_dir.display());
            }
        } else if !t_data_maps_dir.exists() {
            eprintln!("Error: '{}' doesn't exist! To delete a map as safely as possible, you need to provide '{}'!", t_data_maps_dir.display(), t_data_maps_dir.display());
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
            eprintln!("Error: '{}' doesn't exist! To delete a map as safely as possible, you need to provide '{}'!", t_data_layouts_dir.display(), t_data_layouts_dir.display());
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