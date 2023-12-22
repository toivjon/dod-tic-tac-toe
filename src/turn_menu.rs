use crate::Command;

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
fn input_index(input: Input) -> usize {
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

// An enumeration for all available slot types.
#[derive(Clone, Copy, PartialEq)]
pub enum Slot {
    Empty,
    X,
    O,
}

// A type for the game grid containing 3x3 slots.
pub type Grid = [Slot; 9];

// An enumeration for all available player types.
#[derive(Clone, Copy, Debug)]
pub enum Player {
    X,
    O,
}

// Render the visualization of the turn menu.
pub fn output_turn_menu(output: fn(&str), grid: &Grid, player: &Player) {
    output(format!("Current turn: {:?}", player).as_str());
    output_grid(output, &grid);
    output("");
    output("Please enter a cell e.g. 'B2':");
}

// Render the visualization of the grid.
fn output_grid(output: fn(&str), grid: &Grid) {
    let chars = grid.map(slot_char);
    output("  | A | B | C |     ");
    output("------------------  ");
    output(format!("1 | {} | {} | {} | 1", chars[0], chars[1], chars[2]).as_str());
    output("------------------  ");
    output(format!("2 | {} | {} | {} | 2", chars[3], chars[4], chars[5]).as_str());
    output("------------------  ");
    output(format!("3 | {} | {} | {} | 3", chars[6], chars[7], chars[8]).as_str());
    output("------------------  ");
    output("  | A | B | C |     ");
}

// Return the char presenting the slot.
fn slot_char(slot: Slot) -> char {
    match slot {
        Slot::Empty => ' ',
        Slot::O => 'O',
        Slot::X => 'X',
    }
}

// TODO change idx into a type to avoid out-of-range indices.
fn assign_grid_slot(grid: &Grid, idx: usize, slot: Slot) -> Grid {
    let mut result = grid.clone();
    result[idx] = slot;
    return result;
}

// Return the opposite player for the given player.
fn opposite_player(player: Player) -> Player {
    match player {
        Player::O => Player::X,
        Player::X => Player::O,
    }
}

// Return the slot representing the player.
fn player_slot(player: &Player) -> Slot {
    match player {
        Player::O => Slot::O,
        Player::X => Slot::X,
    }
}

// An enumeration of all possible game states.
enum GameState {
    Victory,
    Draw,
    Unfinished,
}

// Check the current state of the grid.
fn game_state(grid: &Grid) -> GameState {
    if has_win(grid) {
        GameState::Victory
    } else if has_free(grid) {
        GameState::Unfinished
    } else {
        GameState::Draw
    }
}

// Check whether the given grid contains a winning line.
fn has_win(grid: &Grid) -> bool {
    (grid[0] != Slot::Empty && grid[0] == grid[1] && grid[1] == grid[2])
        || (grid[3] != Slot::Empty && grid[3] == grid[4] && grid[4] == grid[5])
        || (grid[6] != Slot::Empty && grid[6] == grid[7] && grid[7] == grid[8])
        || (grid[0] != Slot::Empty && grid[0] == grid[3] && grid[3] == grid[6])
        || (grid[1] != Slot::Empty && grid[1] == grid[4] && grid[4] == grid[7])
        || (grid[2] != Slot::Empty && grid[2] == grid[5] && grid[5] == grid[8])
        || (grid[0] != Slot::Empty && grid[0] == grid[4] && grid[4] == grid[8])
        || (grid[2] != Slot::Empty && grid[2] == grid[4] && grid[4] == grid[6])
}

// Check whether the given grid contains a free cell.
fn has_free(grid: &Grid) -> bool {
    grid.contains(&Slot::Empty)
}

// Render the visualization of the victory.
pub fn output_victory(output: fn(&str), grid: &Grid, player: &Player) {
    output_grid(output, &grid);
    output("");
    output(format!("Player {:?} wins the game! Congratulations!", player).as_str());
}

// Render the visualization of the draw.
pub fn output_draw(output: fn(&str), grid: &Grid) {
    output_grid(output, &grid);
    output("");
    output("Game ends in a draw! Better luck next time!");
}

// Handle the provided input to react on user input.
pub fn handle_input(input: Input, grid: &Grid, player: &Player) -> Command {
    let index = input_index(input);
    if grid[index] == Slot::Empty {
        let new_grid = assign_grid_slot(grid, index, player_slot(player));
        match game_state(&new_grid) {
            GameState::Victory => Command::Victory(new_grid, *player),
            GameState::Draw => Command::Draw(new_grid),
            GameState::Unfinished => Command::TurnMenu(new_grid, opposite_player(*player)),
        }
    } else {
        Command::TurnMenu(*grid, *player)
    }
}

#[cfg(test)]
mod tests {

    use crate::turn_menu::{input_index, parse_input, Input};

    use crate::turn_menu::Slot::{Empty, O, X};

    use crate::turn_menu::{has_free, has_win};

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

    #[test]
    fn has_win_returns_true_when_win() {
        assert!(has_win(&[
            X, X, X, Empty, Empty, Empty, Empty, Empty, Empty
        ]));
        assert!(has_win(&[
            Empty, Empty, Empty, X, X, X, Empty, Empty, Empty
        ]));
        assert!(has_win(&[
            Empty, Empty, Empty, Empty, Empty, Empty, X, X, X
        ]));
        assert!(has_win(&[
            X, Empty, Empty, X, Empty, Empty, X, Empty, Empty
        ]));
        assert!(has_win(&[
            Empty, X, Empty, Empty, X, Empty, Empty, X, Empty
        ]));
        assert!(has_win(&[
            Empty, Empty, X, Empty, Empty, X, Empty, Empty, X
        ]));
        assert!(has_win(&[
            X, Empty, Empty, Empty, X, Empty, Empty, Empty, X
        ]));
        assert!(has_win(&[
            Empty, Empty, X, Empty, X, Empty, X, Empty, Empty
        ]));
    }

    #[test]
    fn has_win_returns_false_when_no_win() {
        assert!(!has_win(&[
            X, O, X, Empty, Empty, Empty, Empty, Empty, Empty
        ]));
        assert!(!has_win(&[
            Empty, Empty, Empty, X, O, X, Empty, Empty, Empty
        ]));
        assert!(!has_win(&[
            Empty, Empty, Empty, Empty, Empty, Empty, X, O, X
        ]));
        assert!(!has_win(&[
            X, Empty, Empty, O, Empty, Empty, X, Empty, Empty
        ]));
        assert!(!has_win(&[
            Empty, X, Empty, Empty, O, Empty, Empty, X, Empty
        ]));
        assert!(!has_win(&[
            Empty, Empty, X, Empty, Empty, O, Empty, Empty, X
        ]));
        assert!(!has_win(&[
            X, Empty, Empty, Empty, O, Empty, Empty, Empty, X
        ]));
        assert!(!has_win(&[
            Empty, Empty, X, Empty, O, Empty, X, Empty, Empty
        ]));
    }

    #[test]
    fn has_free_returns_true_when_free_cell_exists() {
        assert!(has_free(&[Empty, X, X, X, X, X, X, X, X]));
        assert!(has_free(&[X, Empty, X, X, X, X, X, X, X]));
        assert!(has_free(&[X, X, Empty, X, X, X, X, X, X]));
        assert!(has_free(&[X, X, X, Empty, X, X, X, X, X]));
        assert!(has_free(&[X, X, X, X, Empty, X, X, X, X]));
        assert!(has_free(&[X, X, X, X, X, Empty, X, X, X]));
        assert!(has_free(&[X, X, X, X, X, X, Empty, X, X]));
        assert!(has_free(&[X, X, X, X, X, X, X, Empty, X]));
        assert!(has_free(&[X, X, X, X, X, X, X, X, Empty]));
    }

    #[test]
    fn has_free_returns_false_when_no_free_cell_exists() {
        assert!(!has_free(&[O, X, X, X, X, X, X, X, X]));
        assert!(!has_free(&[X, O, X, X, X, X, X, X, X]));
        assert!(!has_free(&[X, X, O, X, X, X, X, X, X]));
        assert!(!has_free(&[X, X, X, O, X, X, X, X, X]));
        assert!(!has_free(&[X, X, X, X, O, X, X, X, X]));
        assert!(!has_free(&[X, X, X, X, X, O, X, X, X]));
        assert!(!has_free(&[X, X, X, X, X, X, O, X, X]));
        assert!(!has_free(&[X, X, X, X, X, X, X, O, X]));
        assert!(!has_free(&[X, X, X, X, X, X, X, X, O]));
    }
}
