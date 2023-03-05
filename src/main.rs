use crate::cgpt::Repl;
pub mod cgpt;

fn main() {
    let mut repl = Repl::new();
    if let Err(e) = repl.start() {
        panic!("{}", e.to_string());
    }
}
