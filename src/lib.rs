pub mod del_attribute;
pub mod del_map;
pub mod filter;
// -----------
// To be added
// -----------
//pub mod list;
/*
pub fn list_for_value(command: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
    match value {
        _ => {
            eprintln!(
                "Value '{command}' is not an available command. Use '--help' for more information.",
            );
            Ok(())
        }
    }
}
*/