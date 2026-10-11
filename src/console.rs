//! Console output that never panics.
//!
//! `eprintln!` / `println!` panic when the stream is closed (for example
//! `context-builder … 2>&1 | head -3`), which killed the run with exit
//! code 101 before the document was finished. Diagnostics and progress lines
//! are best-effort, so these macros drop the write error instead. Document
//! content written to stdout (`-o -`) still reports errors through `io::Result`.

use std::fmt::Arguments;
use std::io::Write;

/// The macros are thin wrappers over these functions so the call site keeps
/// its own line (and coverage attribution) instead of the macro body's.
pub(crate) fn write_err(args: Arguments<'_>, newline: bool) {
    let mut err = std::io::stderr();
    let _ = err.write_fmt(args);
    if newline {
        let _ = err.write_all(b"\n");
    }
}

pub(crate) fn write_out(args: Arguments<'_>, newline: bool) {
    let mut out = std::io::stdout();
    let _ = out.write_fmt(args);
    if newline {
        let _ = out.write_all(b"\n");
    }
}

macro_rules! errln {
    () => {
        $crate::console::write_err(format_args!(""), true)
    };
    ($($arg:tt)*) => {
        $crate::console::write_err(format_args!($($arg)*), true)
    };
}

macro_rules! err {
    ($($arg:tt)*) => {
        $crate::console::write_err(format_args!($($arg)*), false)
    };
}

macro_rules! outln {
    () => {
        $crate::console::write_out(format_args!(""), true)
    };
    ($($arg:tt)*) => {
        $crate::console::write_out(format_args!($($arg)*), true)
    };
}
