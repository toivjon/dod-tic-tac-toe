use std::io;
use tic_tac_toe::run;

fn main() {
    run(|output| println!("{output}"), input)
}

fn input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(_) => todo!(),
    }
}
