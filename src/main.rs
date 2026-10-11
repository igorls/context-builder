use std::io;

fn main() -> io::Result<()> {
    context_builder::finish(context_builder::run())
}
