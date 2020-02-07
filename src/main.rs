#![allow(dead_code)]

mod token;
mod executor;

fn main() {
    let tokens = token::tokenize("++>>++<++<.>.>.");
    let mut env = executor::VM::new(&tokens);
    env.execute();

    println!("");
    println!("");
    env.dump();
}
