use cgpt::Repl;

fn main() {
    let mut repl = Repl::new();
    if let Err(e) = repl.start() {
        println!("Could not start repl: {e}");
        std::process::exit(1);
    };
}
