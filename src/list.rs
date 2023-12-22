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
