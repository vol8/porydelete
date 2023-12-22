// gTileset_General is not deleteable. Instead use porytiles to edit it. However every other tileset is deleteable
// Step 1: Regex: const struct Tileset \w+ =\n\{\n    .isCompressed = \w+,\n    .isSecondary = \w+,\n    .tiles = \w+,\n    .palettes = \w+,\n    .metatiles = \w+,\n    .metatileAttributes = \w+,\n    .callback = \w+,\n\};

#![allow(unused_variables)]

pub mod del_anim;
use convert_case::{Case, Casing};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

type PdTsError = Result<(), Box<dyn std::error::Error>>;
type PdTsErrorCaptures = Result<Vec<String>, Box<dyn std::error::Error>>;

// Todo: Finish and use this function
fn get_paths(ts_name: &str) -> Vec<PathBuf> {
    vec![
        // tileset struct definition (for general and other tilesets)
        Path::new("./src/data/tilesets/headers.h").to_path_buf(),
        // tileset definitions for palettes and tiles (ONLY OTHER TILES)
        /*other  */
        Path::new("src/data/tilesets/graphics.h").to_path_buf(),
        /*general*/ Path::new("src/graphics.h").to_path_buf(),
        // Metatiles and MetatilsAttribute definitons (for general and other tilesets)
        Path::new("src/data/tilesets/metatiles.h").to_path_buf(),
        // Location of tile and palette data for tilesets
        /*PRIMARY  */
        Path::new("/data/tilesets/primary/")
            .join(ts_name)
            .to_path_buf(),
        /*SECONDARY*/
        Path::new("/data/tilesets/secondary/")
            .join(ts_name)
            .to_path_buf(),
        // Path to tileset_anims.h
        Path::new("include/tileset_anims.h").to_path_buf(),
        // Path to tileset_anims.c\
        Path::new("src/tileset_anims.c").to_path_buf(),
    ]
}

// Todo: Finish and use this function
fn tileset_exists(ts_name: &str) -> bool {
    let paths = get_paths(ts_name);

    for path in paths {
        if !path.exists() {
            println!("Error: '{}' does not exist!", ts_name);
            return false;
        }
    }
    true
}

fn remove_tileset_def(ts_name: &str) -> PdTsErrorCaptures {
    let re_prefix = r"const\s*struct\s*Tileset\s*";
    let re_suffix = r"\s*=\n\{\n\s*.isCompressed\s*=\s*\w+,\n\s*.isSecondary\s*=\s*\w+,\n\s*.tiles\s*=\s*(?P<tiles>\w+),\n\s*.palettes\s*=\s*(?P<pals>\w+),\n\s*.metatiles\s*=\s*(?P<metatiles>\w+),\n\s*.metatileAttributes\s*=\s*(?P<metatilesattr>\w+),\n\s*.callback\s*=\s*(?P<callback>\w+),\n\};";

    let re = Regex::new(format!(r"{}{}{}", re_prefix, ts_name, re_suffix).as_str()).unwrap();
    let path = Path::new("./src/data/tilesets/headers.h").to_path_buf();

    let contents = fs::read_to_string(&path).unwrap();

    let tileset_def = re.find(&contents);

    if tileset_def.is_none() {
        eprintln!(
            "Step 1: Error: Couldn't find tileset definition in './src/data/tilesets/headers.h'."
        );
        Ok(vec![])
    } else {
        let new = contents.replace(tileset_def.unwrap().as_str(), "");
        fs::write(path, new)?;

        let mut captures: Vec<String> = vec![];
        if let Some(cap) = re.captures(&contents) {
            captures.push(cap.name("tiles").unwrap().as_str().to_string());
            captures.push(cap.name("pals").unwrap().as_str().to_string());
            captures.push(cap.name("metatiles").unwrap().as_str().to_string());
            captures.push(cap.name("metatilesattr").unwrap().as_str().to_string());
            captures.push(cap.name("callback").unwrap().as_str().to_string());
        }

        println!("Step 1: Found and deleted tileset definition!");
        Ok(captures)
    }
}

fn remove_tiles_pal_def(fn_tile_name: &str, fn_pals_name: &str) -> PdTsError {
    let re_tiles_prefix = "const\\s*u32\\s*";
    let re_tiles_suffix =
        "\\s*\\[\\]\\s*=\\s*INCBIN_U32\\(\"data\\/tilesets\\/\\w+\\/\\w+\\/tiles\\.4bpp\\.lz\"\\);";

    let re_pals_prefix = "const u16 ";
    let re_pals_suffix = r#"\[\]\[16\] =\s*\{(?:\s*\n\s*INCBIN_U16\("data/tilesets/\w+/\w+/palettes/\d{2}\.gbapal"\),)*\s*\};"#;

    // Regular expressions
    let re_tiles =
        Regex::new(format!("{}{}{}", re_tiles_prefix, fn_tile_name, re_tiles_suffix).as_str())
            .unwrap();
    let re_pals =
        Regex::new(format!("{}{}{}", re_pals_prefix, fn_pals_name, re_pals_suffix).as_str())
            .unwrap();
    let path = Path::new("src/data/tilesets/graphics.h").to_path_buf();

    let contents = fs::read_to_string(&path).unwrap();

    let tiles_def = re_tiles.find(&contents);
    let pals_def = re_pals.find(&contents);

    if tiles_def.is_none() || pals_def.is_none() {
        eprintln!("Step 2: Error: Couldn't find tileset gfx and palettes gfx definitions in './src/data/tilesets/graphics.h'.");
        Ok(())
    } else {
        let new = contents
            .replace(pals_def.unwrap().as_str(), "")
            .replace(tiles_def.unwrap().as_str(), "");
        fs::write(path, new)?;
        println!("Step 2: Found and deleted tileset gfx and palettes gfx definitions!");
        Ok(())
    }
}

fn remove_metatiles_def(fn_metatiles_name: &str, fn_metatiles_attr_name: &str) -> PdTsError {
    let prefix = "const\\s*u16\\s*";
    let suffix = "\\s*\\[\\]\\s*=\\s*INCBIN_U16\\(\"\\w+\\/\\w+\\/\\w+\\/\\w+\\/\\w+.\\w+\"\\);";

    let re_metatiles =
        Regex::new(format!("{}{}{}", prefix, fn_metatiles_name, suffix).as_str()).unwrap();
    let re_metatiles_attr =
        Regex::new(format!("{}{}{}", prefix, fn_metatiles_attr_name, suffix).as_str()).unwrap();

    let path = Path::new("./src/data/tilesets/metatiles.h").to_path_buf();
    let contents = fs::read_to_string(&path).unwrap();

    let found_metatiles = re_metatiles.find(&contents);
    let found_metatiles_attr = re_metatiles_attr.find(&contents);

    if found_metatiles.is_none() || found_metatiles_attr.is_none() {
        eprintln!("Step 3: Error: Couldn't find metatile and metatile attribute definitions in 'src/data/tilesets/metatiles.h'.");
        Ok(())
    } else {
        let new = contents
            .replace(found_metatiles.unwrap().as_str(), "")
            .replace(found_metatiles_attr.unwrap().as_str(), "");
        fs::write(path, new)?;
        println!("Step 3: Found and deleted metatile and metatile attribute definitions!");
        Ok(())
    }
}

fn remove_folder(ts_name: &str) -> PdTsError {
    let dir_name = ts_name.replace("gTileset_", "").to_case(Case::Snake);
    let path_primary =
        Path::new(format!("./data/tilesets/primary/{}", dir_name).as_str()).to_path_buf();
    let path_secondary =
        Path::new(format!("./data/tilesets/secondary/{}", dir_name).as_str()).to_path_buf();

    if path_primary.exists() {
        fs::remove_dir_all(&path_primary)?;
        if !path_primary.exists() {
            println!(
                "Step 4: Removed tileset directory '{}'.",
                path_primary.display()
            );
        }
    } else if path_secondary.exists() {
        fs::remove_dir_all(&path_secondary)?;
        if !path_secondary.exists() {
            println!(
                "Step 4: Removed tileset directory '{}'.",
                path_secondary.display()
            );
        }
    } else {
        println!("Step 4: Something went wrong!?");
    }
    Ok(())
}

// **************************************************
// Note: Clean up this function if del_anims finished
// **************************************************
// Todo: If function 'get_paths' and 'tileset_exists' are finished, use them in here to check if the tileset the user wants to delete, already exists
pub fn execute_del(ts_name: &str) -> PdTsError {
    if ts_name == "gTileset_General" {
        eprintln!("Sorry: Can't delete 'gTileset_General'. Instead, try to reuse it, by using Porytiles: https://github.com/grunt-lucas/porytiles");
        Ok(())
    } else {
        let captures = remove_tileset_def(ts_name)?;
        remove_tiles_pal_def(captures[0].as_str(), captures[1].as_str())?;
        remove_metatiles_def(captures[2].as_str(), captures[3].as_str())?;
        remove_folder(ts_name)?;
        Ok(())
    }
}
