mod main_menu;
mod turn_menu;

// An enumeration of all available commands within the game.
#[derive(Debug, PartialEq)]
pub enum Command {
    OpenMainMenu,
    OpenTurnMenu,
    Exit,
}

// Run the game with the provided input and output channels.
pub fn run(output: fn(&str), input: fn() -> String) {
    let mut command = Command::OpenMainMenu;
    loop {
        match command {
            Command::OpenMainMenu => command = main_menu::run(output, input),
            Command::OpenTurnMenu => command = turn_menu::run(output, input),
            Command::Exit => {
                output("Bye!");
                break;
            }
        }
    }
}
