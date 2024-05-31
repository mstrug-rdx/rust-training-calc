use calc::eval_str;
use std::io;

fn main() {
    println!("Enter equation:");
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(_) => match eval_str(&buf) {
            Ok(result) => println!("Result: {result}"),
            Err(e) => println!("Error occured: {e}"),
        },
        Err(e) => println!("Error occured: {e}"),
    }
}
