use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    match context_builder::run() {
        Ok(()) => ExitCode::SUCCESS,
        // The reader of `-o -` went away (`context-builder -o - | head`).
        // That ends a pipeline; it is not a failure of ours.
        Err(e) if e.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(e) => {
            // Once, human-readable, and never a panic if stderr is closed.
            let _ = writeln!(io::stderr(), "Error: {e}");
            ExitCode::FAILURE
        }
    }
}
