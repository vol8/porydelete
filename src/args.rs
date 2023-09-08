use clap::Parser;

// Arguments which need to be passed to the program.
#[derive(Parser)]
pub struct Args {
    // Command to execute
    pub command: String,
    // Values to pass to the command
    pub value: String,
}

impl Args {
    pub fn other_case_command(&self) {
        eprintln!(
            "Command '{}' is not an available command. Use '--help' for more information.",
            self.command
        );
    }

    pub fn other_case_value(&self) {
        eprintln!(
            "Value '{}' is not an available command. Use '--help' for more information.",
            self.command
        );
    }
}
