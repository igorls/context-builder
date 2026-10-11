//! Console output that never panics.
//!
//! `eprintln!` / `println!` panic when the stream is closed (for example
//! `context-builder … 2>&1 | head -3`), which killed the run with exit
//! code 101 before the document was finished. Diagnostics and progress lines
//! are best-effort, so these helpers drop the write error instead. Document
//! content written to stdout (`-o -`) still reports errors through `io::Result`.

use std::fmt::Arguments;
use std::io::Write;

/// Best-effort line on stderr; a closed stream is ignored.
pub(crate) fn errln(args: Arguments<'_>) {
    let mut err = std::io::stderr();
    let _ = err.write_fmt(args);
    let _ = err.write_all(b"\n");
}

/// Best-effort text on stderr, no newline.
pub(crate) fn err(args: Arguments<'_>) {
    let _ = std::io::stderr().write_fmt(args);
}

/// Best-effort line on stdout; a closed stream is ignored.
pub(crate) fn outln(args: Arguments<'_>) {
    let mut out = std::io::stdout();
    let _ = out.write_fmt(args);
    let _ = out.write_all(b"\n");
}
