mod main_menu;
mod turn_menu;

// An enumeration of all available commands within the game.
#[derive(Debug, PartialEq)]
pub enum Command {
    MainMenu,
    OpenTurnMenu,
    TurnMenu(turn_menu::Grid, turn_menu::Player),
    Victory(turn_menu::Grid, turn_menu::Player),
    Draw(turn_menu::Grid),
    Exit,
}

// Run the game with the provided input and output channels.
pub fn run(output: fn(&str), input: fn() -> String) {
    let mut command = Option::Some(Command::MainMenu);
    while command.is_some() {
        command = execute_command(&command.unwrap(), output, input);
    }
}

// Execute the given command possibly leading into a next command to be executed.
fn execute_command(command: &Command, output: fn(&str), input: fn() -> String) -> Option<Command> {
    match command {
        Command::MainMenu => {
            output(main_menu::HEADING);
            main_menu::parse_input(input().trim()).map_or_else(
                |_| Some(Command::MainMenu),
                |x| Some(main_menu::handle_input(x)),
            )
        }
        Command::OpenTurnMenu => {
            let grid = [turn_menu::Slot::Empty; 9];
            let player = turn_menu::Player::O;
            output(turn_menu::turn_menu_string(&grid, &player).as_str());
            turn_menu::parse_input(input().trim()).map_or_else(
                |_| Some(Command::OpenTurnMenu),
                |x| Some(turn_menu::handle_input(&x, &grid, &player)),
            )
        }
        Command::TurnMenu(grid, player) => {
            output(turn_menu::turn_menu_string(grid, player).as_str());
            turn_menu::parse_input(input().trim()).map_or_else(
                |_| Some(Command::TurnMenu(*grid, *player)),
                |x| Some(turn_menu::handle_input(&x, &grid, &player)),
            )
        }
        Command::Victory(grid, player) => {
            output(turn_menu::victory_string(grid, player).as_str());
            Option::Some(Command::Exit)
        }
        Command::Draw(grid) => {
            output(turn_menu::draw_string(grid).as_str());
            Option::Some(Command::Exit)
        }
        Command::Exit => {
            output("Bye!");
            Option::None
        }
    }
}
