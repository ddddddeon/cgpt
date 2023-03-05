use crate::cgpt::Repl;
pub mod cgpt;

fn main() {
    let mut repl = Repl::new();
    repl.start().expect("Cannot start REPL");
}
