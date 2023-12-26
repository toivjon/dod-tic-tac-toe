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
    let mut command = Option::Some(Command::OpenMainMenu);
    while command.is_some() {
        command = execute_command(&command.unwrap(), output, input);
    }
}

// Execute the given command possibly leading into a next command to be executed.
fn execute_command(command: &Command, output: fn(&str), input: fn() -> String) -> Option<Command> {
    match command {
        Command::OpenMainMenu => Some(main_menu::run(output, input)),
        Command::OpenTurnMenu => Some(turn_menu::run(output, input)),
        Command::Exit => {
            output("Bye!");
            Option::None
        }
    }
}
