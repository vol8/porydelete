use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

type PdError = Result<(), Box<dyn std::error::Error>>;


fn fn_init_tileset_anim(ts_name: &str) {
    let fn_name = ts_name.replace("gTileset", "InitTilesetAnim");

    let header_regex = Regex::new(format!(r"\w+\s*{}\(\w+\);", fn_name).as_str()).unwrap();
    let src_re_suffix = r"\(\w+\)\n\{\n(\s*\w+\s*=\s*\w+;)*\n}";
    let source_regex = Regex::new(format!(r"\w+\s*{}{}", fn_name, src_re_suffix).as_str()).unwrap();
    
    let header_contents = fs::read_to_string("./include/tileset_anims.h").unwrap();
    let source_contents = fs::read_to_string("./src/tileset_anims.c").unwrap();

    let header_match = header_regex.find(&header_contents);
    let source_match = source_regex.find(&source_contents);

    //println!("{}\n\n", source_regex);

    if header_match.is_none() {
        eprintln!("Error: Can't find '{}' in './include/tileset_anims.h'", fn_name);
    } else {
        let header_new = header_contents.replace(header_match.unwrap().as_str(), ""); 
        match fs::write("./include/tileset_anims.h", header_new) {
            Ok(o) => println!("Anims. 1.2: Deleted '{}' in './include/tileset_anims.h'", fn_name),
            Err(e) => eprintln!("Fatal Error: Anims. 1.2: Failed to write to './include/tileset_anims.h'"),
        }
    }
    if source_match.is_none() {
        eprintln!("Error: Can't find '{}' in './src/tileset_anims.c'", fn_name);
    } else {
        let source_new = source_contents.replace(source_match.unwrap().as_str(), "");
        match fs::write("./src/tileset_anims.c", source_new) {
            Ok(o) => println!("Anims. 1.1: Deleted '{}' in './src/tileset_anims.c", fn_name),
            Err(e) => eprintln!("Fatal Error: Anims. 1.1: Failed to write to '/src/tileset_anims.c'"),
        }
    }
}

fn fn_tileset_anim(ts_name: &str) {
    let fn_name = ts_name.replace("gTileset", "TilesetAnim");

    let re_dec = Regex::new(format!(r"static\s*void\s*{}\s*\(\w+\);", fn_name).as_str()).unwrap();
    
    let re_def_suffix = r#"\s*\(\w+\s*\w+\)\n\{\n(\s*if\s*\(\w+\s*%\s*\w+\s*==\s*\w+\)\n\s*\w+\(\w+\s*\/\s*\w+\);)*\n\}"#;
    let re_def_suffix = r"\{([^{}]*|(?R))*\}";
    let re_def = Regex::new(format!(r"static\s*void\s*{}\(\w+\s*\w+\)\n{}", fn_name, re_def_suffix).as_str()).unwrap();

    let contents = fs::read_to_string("./src/tileset_anims.c").unwrap();

    let dec_match = re_dec.find(&contents);
    let def_match = re_def.find(&contents);

    if dec_match.is_none() {
        eprintln!("dec failed");
    }
    if def_match.is_none() {
        eprintln!("def failed");    
    }


    /*if dec_match.is_none() || def_match.is_none() {
        eprintln!("Error: Couldn't find function declaration '{}' in './src/tileset_anims.c'", fn_name);
    } else {
        let contents_new = &contents.replace(dec_match.unwrap().as_str(), "").replace(def_match.unwrap().as_str(), "");
        match fs::write("./src/tileset_anims.c", contents_new) {
            Ok(o) => println!("Anims. 2: Deleted '{}' in './src/tileset_anims.c", fn_name),
            Err(e) => eprintln!("Fatal Error: Anims. 2: Failed to write to '/src/tileset_anims.c'"),
        }
    }*/
}

fn fn_queue_anim_tiles(ts_name: &str) {
    println!("Not ready yet...");
}

fn tileset_anims_frame(ts_name: &str) {
    println!("Not ready yet...");
}

pub fn execute_del(ts_name: &str) -> PdError {
    fn_init_tileset_anim(ts_name);
    fn_tileset_anim(ts_name);
    fn_queue_anim_tiles(ts_name);
    tileset_anims_frame(ts_name);
    Ok(())
}