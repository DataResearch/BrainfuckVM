#![allow(dead_code)]

mod token;
mod executor;

fn main() {
    let tokens = token::tokenize("+[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.");
    
    println!("{:#?}", tokens);
    
    let mut env = executor::VM::new(&tokens);
    env.execute();

    println!("");
    println!("");
    env.dump();
}
