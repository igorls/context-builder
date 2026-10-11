//! Console output that never panics.
//!
//! `eprintln!` / `println!` panic when the stream is closed (for example
//! `context-builder … 2>&1 | head -3`), which killed the run with exit
//! code 101 before the document was finished. Diagnostics and progress lines
//! are best-effort, so these macros drop the write error instead. Document
//! content written to stdout (`-o -`) still reports errors through `io::Result`.

macro_rules! errln {
    () => {{
        use ::std::io::Write as _;
        let _ = ::std::writeln!(::std::io::stderr());
    }};
    ($($arg:tt)*) => {{
        use ::std::io::Write as _;
        let _ = ::std::writeln!(::std::io::stderr(), $($arg)*);
    }};
}

macro_rules! err {
    ($($arg:tt)*) => {{
        use ::std::io::Write as _;
        let _ = ::std::write!(::std::io::stderr(), $($arg)*);
    }};
}

macro_rules! outln {
    () => {{
        use ::std::io::Write as _;
        let _ = ::std::writeln!(::std::io::stdout());
    }};
    ($($arg:tt)*) => {{
        use ::std::io::Write as _;
        let _ = ::std::writeln!(::std::io::stdout(), $($arg)*);
    }};
}
