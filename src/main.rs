use std::io;

fn main() {
    tic_tac_toe::run(&output, &input)
}

// An output channel that simply writes a line to standard out.
fn output(value: &str) {
    println!("{value}")
}

// An input channel that simply reads a line from standard in.
fn input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(_) => String::from(""),
    }
}
