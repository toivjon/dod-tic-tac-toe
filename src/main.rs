use std::io;

fn main() {
    show_menu()
}

fn show_menu() {
    println!("===================");
    println!("=== Tic-Tac-Toe ===");
    println!("===================");
    println!();
    println!("Please enter a selection:");
    println!("[1] Play");
    println!("[2] Quit");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.trim() {
            "1" => println!("ONE!"),
            "2" => println!("TWO!"),
            _ => show_menu(),
        },
        Err(error) => println!("error: {error}"),
    }
}
