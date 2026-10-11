use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    match context_builder::finish(context_builder::run()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            // Once, human-readable, and never a panic if stderr is closed.
            let _ = writeln!(io::stderr(), "Error: {e}");
            ExitCode::FAILURE
        }
    }
}
