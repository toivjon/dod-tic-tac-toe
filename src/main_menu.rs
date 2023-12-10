// An enumeration of the main menu inputs.
#[derive(Debug, PartialEq)]
pub enum Input {
    Start,
    Exit,
}

// Parse the main menu input from the provided input string.
pub fn parse_input(input: &str) -> Result<Input, &'static str> {
    match input {
        "1" => Ok(Input::Start),
        "2" => Ok(Input::Exit),
        _ => Err("invalid input"),
    }
}

#[cfg(test)]
mod tests {

    use crate::main_menu::{parse_input, Input};

    #[test]
    fn parse_input_returns_ok_with_valid_input() {
        assert_eq!(parse_input("1"), Ok(Input::Start));
        assert_eq!(parse_input("2"), Ok(Input::Exit));
    }

    #[test]
    fn parse_input_returns_err_with_invalid_input() {
        assert_eq!(parse_input(""), Err("invalid input"));
        assert_eq!(parse_input("0"), Err("invalid input"));
        assert_eq!(parse_input("3"), Err("invalid input"));
    }
}
