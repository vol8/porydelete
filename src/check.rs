type PdFilter = Result<(), Box<dyn std::error::Error>>;

pub fn check(value: &str) -> PdFilter {
    match value {
        "map" => Ok(()),
        "tileset" => Ok(()),
        "pkmn" => Ok(()),
        "item" => Ok(()),
        _ => Ok(())
    }
}


