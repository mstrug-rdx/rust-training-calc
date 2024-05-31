use calc::eval_str;
use std::io;

fn main() -> Result<(), io::Error> {
    loop {
        println!("Enter equation (or Enter to finish):");
        let mut buf = String::new();
        if io::stdin().read_line(&mut buf)? == 1 {
            break;
        } else {
            match eval_str(&buf) {
                Ok(result) => println!("Result: {result}"),
                Err(e) => println!("Error occured: {e}"),
            }
        }
    }
    Ok(())
}
