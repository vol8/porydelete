use crate::args::Args;

pub fn list_for_value(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    match args.value.as_str() {
        "script" => Ok(()),
        _ => Ok(args.other_case_value()),
    }
}
