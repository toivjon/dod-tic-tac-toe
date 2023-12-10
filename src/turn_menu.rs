// An enumeration of the main menu inputs.
#[derive(Debug, PartialEq)]
pub enum Input {
    A1,
    B1,
    C1,
    A2,
    B2,
    C2,
    A3,
    B3,
    C3,
}

// Parse the turn menu input form the provided input string.
pub fn parse_input(input: &str) -> Result<Input, &'static str> {
    match input.trim() {
        "A1" => Ok(Input::A1),
        "B1" => Ok(Input::B1),
        "C1" => Ok(Input::C1),
        "A2" => Ok(Input::A2),
        "B2" => Ok(Input::B2),
        "C2" => Ok(Input::C2),
        "A3" => Ok(Input::A3),
        "B3" => Ok(Input::B3),
        "C3" => Ok(Input::C3),
        _ => Err("invalid input"),
    }
}

// Get the corresponding grid index for the provided input.
pub fn input_index(input: Input) -> usize {
    match input {
        Input::A1 => 0,
        Input::B1 => 1,
        Input::C1 => 2,
        Input::A2 => 3,
        Input::B2 => 4,
        Input::C2 => 5,
        Input::A3 => 6,
        Input::B3 => 7,
        Input::C3 => 8,
    }
}

#[cfg(test)]
mod tests {

    use crate::turn_menu::{input_index, parse_input, Input};

    #[test]
    fn parse_input_returns_ok_with_valid_input() {
        assert_eq!(parse_input("A1"), Ok(Input::A1));
        assert_eq!(parse_input("B1"), Ok(Input::B1));
        assert_eq!(parse_input("C1"), Ok(Input::C1));
        assert_eq!(parse_input("A2"), Ok(Input::A2));
        assert_eq!(parse_input("B2"), Ok(Input::B2));
        assert_eq!(parse_input("C2"), Ok(Input::C2));
        assert_eq!(parse_input("A3"), Ok(Input::A3));
        assert_eq!(parse_input("B3"), Ok(Input::B3));
        assert_eq!(parse_input("C3"), Ok(Input::C3));
    }

    #[test]
    fn parse_input_returns_err_with_invalid_input() {
        assert_eq!(parse_input(""), Err("invalid input"));
        assert_eq!(parse_input("A0"), Err("invalid input"));
        assert_eq!(parse_input("A4"), Err("invalid input"));
        assert_eq!(parse_input("D1"), Err("invalid input"));
    }

    #[test]
    fn input_index_returns_correct_indices() {
        assert_eq!(input_index(Input::A1), 0);
        assert_eq!(input_index(Input::B1), 1);
        assert_eq!(input_index(Input::C1), 2);
        assert_eq!(input_index(Input::A2), 3);
        assert_eq!(input_index(Input::B2), 4);
        assert_eq!(input_index(Input::C2), 5);
        assert_eq!(input_index(Input::A3), 6);
        assert_eq!(input_index(Input::B3), 7);
        assert_eq!(input_index(Input::C3), 8);
    }
}
