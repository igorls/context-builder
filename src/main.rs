use std::io;

fn main() -> io::Result<()> {
    match context_builder::run() {
        // The reader of `-o -` went away (`context-builder -o - | head`).
        // That ends a pipeline; it is not a failure of ours.
        Err(e) if e.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        other => other,
    }
}
