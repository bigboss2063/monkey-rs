use std::io;

fn main() {
    println!("This is the Monkey programming language!");
    monkey::repl::start(io::stdin(), io::stdout());
}
