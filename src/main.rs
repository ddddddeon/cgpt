use crate::cgpt::Repl;
pub mod cgpt;

fn main() {
    let mut repl = Repl::new();
    repl.start().unwrap_or_else(|err| {
        println!("Could not start repl: {err}");
        std::process::exit(1);
    });
}
