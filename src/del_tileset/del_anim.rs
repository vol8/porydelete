use regex::Regex;
use std::fs;
use std::path::Path;

type PdError = Result<(), Box<dyn std::error::Error>>;
type PdTsErrorCaptures = Result<Vec<String>, Box<dyn std::error::Error>>;

const PATH_TILESET_ANIMS: &str = "./src/tileset_anims.c";

// DONE
// In the wiki, this removes the first two code snippets in Step 6: https://github.com/Voluptua/porydelete/wiki/Map-Tilesets#step-6-only-applies-to-tilesets-with-animations
fn remove_fn_init_tileset_anim(ts_name: &str) {
    let fn_name = ts_name.replace("gTileset", "InitTilesetAnim");

    let header_regex = Regex::new(format!(r"\w+\s*{}\(\w+\);", fn_name).as_str()).unwrap();
    let src_re_suffix = r"\(\w+\)\n\{\n(\s*\w+\s*=\s*\w+;)*\n}";
    let source_regex = Regex::new(format!(r"\w+\s*{}{}", fn_name, src_re_suffix).as_str()).unwrap();

    let header_contents = fs::read_to_string("./include/tileset_anims.h").unwrap();
    let source_contents = fs::read_to_string(PATH_TILESET_ANIMS).unwrap();

    let header_match = header_regex.find(&header_contents);
    let source_match = source_regex.find(&source_contents);

    if source_match.is_none() {
        eprintln!("Error: Can't find '{}' in './src/tileset_anims.c'", fn_name);
    } else {
        let source_new = source_contents.replace(source_match.unwrap().as_str(), "");
        match fs::write("./src/tileset_anims.c", source_new) {
            Ok(o) => println!(
                "Anims. 1.1: Deleted '{}' in '{}'",
                fn_name, PATH_TILESET_ANIMS
            ),
            Err(e) => {
                eprintln!("Fatal Error: Anims. 1.1: Failed to write to '/src/tileset_anims.c'")
            }
        }
    }

    if header_match.is_none() {
        eprintln!(
            "Error: Can't find '{}' in './include/tileset_anims.h'",
            fn_name
        );
    } else {
        let header_new = header_contents.replace(header_match.unwrap().as_str(), "");
        match fs::write("./include/tileset_anims.h", header_new) {
            Ok(o) => println!(
                "Anims. 1.2: Deleted '{}' in './include/tileset_anims.h'",
                fn_name
            ),
            Err(e) => {
                eprintln!("Fatal Error: Anims. 1.2: Failed to write to './include/tileset_anims.h'")
            }
        }
    }
}

// DONE? REGEX ISSUE
// Removes code snippet 3 & 4 in Step 6 of the wiki: https://github.com/Voluptua/porydelete/wiki/Map-Tilesets#step-6-only-applies-to-tilesets-with-animations
fn remove_fn_tileset_anim(ts_name: &str) {
    let fn_name = ts_name.replace("gTileset", "TilesetAnim");

    let re_dec = Regex::new(format!(r"static\s*void\s*{}\s*\(\w+\);", fn_name).as_str()).unwrap();

    // issue here.
    let re_def_suffix = r#"\(\w+\s*\w+\)\n\{\n(.*\n)*\}"#;
    let re_def =
        Regex::new(format!(r#"static\s*void\s*{}{}"#, fn_name, re_def_suffix).as_str()).unwrap();

    let contents = fs::read_to_string(PATH_TILESET_ANIMS).unwrap();

    let dec_match = re_dec.find(&contents);
    let def_match = re_def.find(&contents);

    if dec_match.is_none() {
        eprintln!("Warning: Couldn't find function declaration. Proceeding...");
    } else {
        let contents_new = &contents.replace(dec_match.unwrap().as_str(), "");
        match fs::write(PATH_TILESET_ANIMS, contents_new) {
            Ok(o) => println!("Anims. 2: Deleted function declaration in './src/tileset_anims.c'"),
            Err(e) => eprintln!("Fatal Error: Anims. 2: Failed to write to '/src/tileset_anims.c'"),
        }
    }

    if def_match.is_none() {
        eprintln!("Warning: Couldn't find function definition. Proceeding...");
    } else {
        let contents_new = &contents.replace(def_match.unwrap().as_str(), "");
        match fs::write(PATH_TILESET_ANIMS, contents_new) {
            Ok(o) => println!("Anims. 2: Deleted function definition in './src/tileset_anims.c'"),
            Err(e) => eprintln!("Fatal Error: Anims. 2: Failed to write to '/src/tileset_anims.c'"),
        }
    }
}

// DONE
// Removes code snipped 5 in https://github.com/Voluptua/porydelete/wiki/Map-Tilesets#step-6-only-applies-to-tilesets-with-animations
fn remove_fn_queue_anim_tiles_declaration(ts_name: &str) -> PdTsErrorCaptures {
    let mut fn_names: Vec<String> = vec![];
    let re = Regex::new(
        format!(
            r"static\s*void\s*(QueueAnimTiles_{}_\w+)\((.+)*\);\n",
            ts_name.replace("gTileset_", "")
        )
        .as_str(),
    )
    .unwrap();
    let mut contents_new = fs::read_to_string(PATH_TILESET_ANIMS).unwrap();

    if Path::new(PATH_TILESET_ANIMS).exists() {
        for captures in re.captures_iter(&contents_new.clone()) {
            dbg!("{}", captures.get(0).unwrap().as_str());
            contents_new = contents_new.replace(captures.get(0).unwrap().as_str(), "");
            fn_names.push(captures.get(1).unwrap().as_str().to_owned());
        }
    }

    if fn_names.is_empty() {
        Ok(vec![])
    } else {
        let _ = fs::write(PATH_TILESET_ANIMS, contents_new);
        println!("Anims. 3.1: Removed Queue-Animations-Tiles function declarations.");
        Ok(fn_names)
    }
}

// Todo: REGEX FIX (removes multiple functions than just one, ik why but not how to fix that)
// Removes code snipped 6 in https://github.com/Voluptua/porydelete/wiki/Map-Tilesets#step-6-only-applies-to-tilesets-with-animations
fn remove_fn_queue_anim_tiles_definition(fn_names: &Vec<String>) -> PdError {
    let re_suffix = "\\s*\\((.*)*\\)\\n\\{(.*\\n)*\\}";
    let mut contents = fs::read_to_string(PATH_TILESET_ANIMS)?;

    if Path::new(PATH_TILESET_ANIMS).exists() {
        for name in fn_names {
            let re = Regex::new(format!("\\w+\\s*\\w+\\s*{}{}", name, re_suffix).as_str()).unwrap();
            let fn_match = re.find(&contents);

            if fn_match.is_none() {
                println!("Warning: Anims. 3.2 Couldn't find function definiton of '{name}' in '{PATH_TILESET_ANIMS}'");
            } else if fn_match.is_some() {
                contents = contents.replace(fn_match.unwrap().as_str(), "");
            }
        }
    }
    fs::write(PATH_TILESET_ANIMS, contents)?;
    println!("Anims. 3.2: Removed Queue-Animations-Tiles function definitons.");
    Ok(())
}

// DONE
// Removes last code snippet
fn remove_tileset_anims_frame(fn_names: &Vec<String>) {
    let mut contents = fs::read_to_string(PATH_TILESET_ANIMS).unwrap();

    for name in fn_names {
        let frame_name = name.replace("QueueAnimTiles", "gTilesetAnims");
        let re_def = Regex::new(format!("const u16 {}_\\w+\\[\\]\\s*=\\s*INCBIN_U16\\(\"data/tilesets/\\w+/\\w+/anim/\\w+/\\w+.4bpp\"\\);", frame_name).as_str()).unwrap();
        let re_name = Regex::new(format!("{}_\\w+", frame_name).as_str()).unwrap();

        let mut n: Vec<String> = vec![];
        for capture in re_def.captures_iter(&contents.clone()) {
            n.push(
                re_name
                    .captures(&contents)
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str()
                    .to_string(),
            );
            contents = contents.replace(&capture.get(0).unwrap().as_str(), "");
        }

        let re_prefix = r"const\s*u16\s*\*const\s*";
        let re_suffix = String::from(r"\{\s*((.*),\n\s*)*") + n.last().unwrap() + r"\s*\};";
        let re =
            Regex::new(format!("{}{}\\[\\]\\s*=\\s*{}", re_prefix, frame_name, re_suffix).as_str())
                .unwrap();

        let arr_match = re.find(&contents);

        if arr_match.is_none() {
            eprintln!("Warning: Anims. 4: Couldn't find frame definitions. Proceeding...");
        } else {
            contents = contents.replace(arr_match.unwrap().as_str(), "");
        }
    }

    let _ = fs::write(PATH_TILESET_ANIMS, contents);
    println!("Anims. 4: Successfully deleted frames.");
}

// [TEST]
pub fn test_del(ts_name: &str) -> PdError {
    let names = remove_fn_queue_anim_tiles_declaration(ts_name).unwrap();
    if !names.is_empty() {
        remove_tileset_anims_frame(&names);
    }
    //remove_fn_init_tileset_anim(ts_name);
    //remove_fn_tileset_anim(ts_name);
    //let fn_names = remove_fn_queue_anim_tiles_declaration(ts_name).unwrap();
    //for names in fn_names {
    //    println!("'{}' , ", names);
    //}
    //if !fn_names.is_empty() {
    //    remove_fn_queue_anim_tiles_definition(fn_names)?;
    //    remove_tileset_anims_frame(ts_name);
    //} else if fn_names.is_empty() {
    //    eprintln!("Fatal Error: Anims. 3.2: Couldn't find any Queue-Animations-Tiles functions!");
    //}
    Ok(())
}

pub fn execute_del(ts_name: &str) -> PdError {
    remove_fn_init_tileset_anim(ts_name);
    remove_fn_tileset_anim(ts_name);
    let fn_names = &remove_fn_queue_anim_tiles_declaration(ts_name).unwrap();
    if !fn_names.is_empty() {
        println!("Function def del not ready yet...");
        //remove_fn_queue_anim_tiles_definition(fn_names)?;
        remove_tileset_anims_frame(fn_names);
    } else if fn_names.is_empty() {
        eprintln!("Fatal Error: Anims. 3.2: Couldn't find any Queue-Animations-Tiles functions!");
    }
    Ok(())
}
