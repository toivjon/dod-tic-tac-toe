use crate::Command;

// A heading use to visualize the main menu.
const HEADING: &str = "
===================
=== Tic-Tac-Toe ===
===================

Please enter a selection:
[1] Play
[2] Quit
";

// An enumeration of the main menu inputs.
#[derive(Debug, PartialEq)]
enum Input {
    Play,
    Exit,
}

// Parse the main menu input from the provided input string.
fn parse_input(input: &str) -> Result<Input, &'static str> {
    match input {
        "1" => Ok(Input::Play),
        "2" => Ok(Input::Exit),
        _ => Err("invalid input"),
    }
}

// Handle the provided input to react on user input.
fn handle_input(input: Input) -> Command {
    match input {
        Input::Play => Command::OpenTurnMenu,
        Input::Exit => Command::Exit,
    }
}

pub fn run(output: fn(&str), input: fn() -> String) -> Command {
    output(HEADING);
    parse_input(input().trim()).map_or_else(|_| Command::MainMenu, |x| handle_input(x))
}

#[cfg(test)]
mod tests {

    use crate::{
        main_menu::{handle_input, parse_input, Input},
        Command,
    };

    #[test]
    fn parse_input_returns_ok_with_valid_input() {
        assert_eq!(parse_input("1"), Ok(Input::Play));
        assert_eq!(parse_input("2"), Ok(Input::Exit));
    }

    #[test]
    fn parse_input_returns_err_with_invalid_input() {
        assert_eq!(parse_input(""), Err("invalid input"));
        assert_eq!(parse_input("0"), Err("invalid input"));
        assert_eq!(parse_input("3"), Err("invalid input"));
    }

    #[test]
    fn handle_input_returns_commands() {
        assert_eq!(handle_input(Input::Play), Command::OpenTurnMenu);
        assert_eq!(handle_input(Input::Exit), Command::Exit);
    }
}
