use std::io;

type Slot = char;
type Grid = [Slot; 9];

#[derive(PartialEq, Debug)]
enum GameState {
    InProgress,
    Draw,
    XWins,
    OWins,
}

pub fn show_menu() {
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
            "1" => play([' '; 9], 'O'),
            "2" => println!("Bye!"),
            _ => show_menu(),
        },
        Err(error) => println!("error: {error}"),
    }
}

fn play(grid: Grid, player: char) {
    assert!(player == 'O' || player == 'X'); // TODO replace with a type.
    println!();
    println!("Current turn: {}", player);
    print_grid(grid);
    println!("");
    println!("Please enter a cell e.g. 'B2':");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input_to_slot_index(&input) {
            Ok(val) => {
                if is_free_slot(grid[val]) {
                    let mut new_grid = grid;
                    new_grid[val] = player;
                    match check_game_state(grid) {
                        GameState::InProgress => {
                            if player == 'O' {
                                play(new_grid, 'X');
                            } else {
                                play(new_grid, 'O');
                            }
                        }
                        GameState::OWins => {
                            println!();
                            print_grid(new_grid);
                            println!("");
                            println!("Player O wins the game! Congratulations!");
                        }
                        GameState::XWins => {
                            println!();
                            print_grid(new_grid);
                            println!("");
                            println!("Player X wins the game! Congratulations!");
                        }
                        GameState::Draw => {
                            println!();
                            print_grid(new_grid);
                            println!("");
                            println!("Game ends in a draw! Better luck next time!");
                        }
                    }
                }
            }
            Err(_) => show_menu(),
        },
        Err(error) => println!("error: {error}"),
    }
}

// Print the provided grid into the standard output.
fn print_grid(grid: Grid) {
    println!("  | A | B | C |");
    println!("------------------");
    println!("1 | {} | {} | {} | 1", grid[0], grid[1], grid[2]);
    println!("------------------");
    println!("2 | {} | {} | {} | 2", grid[3], grid[4], grid[5]);
    println!("------------------");
    println!("3 | {} | {} | {} | 3", grid[6], grid[7], grid[8]);
    println!("------------------");
    println!("  | A | B | C |");
}

// Get the index of the slot that corresponds to provided player input.
fn input_to_slot_index(input: &str) -> Result<usize, &'static str> {
    match input.trim() {
        "A1" => Ok(0),
        "B1" => Ok(1),
        "C1" => Ok(2),
        "A2" => Ok(3),
        "B2" => Ok(4),
        "C2" => Ok(5),
        "A3" => Ok(6),
        "B3" => Ok(7),
        "C3" => Ok(8),
        _ => Err("invalid input"),
    }
}

// Check what is the current state of the game based on the given grid.
fn check_game_state(grid: Grid) -> GameState {
    if grid[0] != ' ' {
        if grid[0] == grid[1] && grid[0] == grid[2] {
            if grid[0] == 'X' {
                return GameState::XWins;
            }
            return GameState::OWins;
        }
        if grid[0] == grid[3] && grid[0] == grid[6] {
            if grid[0] == 'X' {
                return GameState::XWins;
            }
            return GameState::OWins;
        }
        if grid[0] == grid[4] && grid[0] == grid[8] {
            if grid[0] == 'X' {
                return GameState::XWins;
            }
            return GameState::OWins;
        }
    }
    if grid[1] != ' ' {
        if grid[1] == grid[4] && grid[1] == grid[7] {
            if grid[1] == 'X' {
                return GameState::XWins;
            }
            return GameState::OWins;
        }
    }
    if grid[2] != ' ' {
        if grid[2] == grid[5] && grid[2] == grid[8] {
            if grid[2] == 'X' {
                return GameState::XWins;
            }
            return GameState::OWins;
        }
        if grid[2] == grid[4] && grid[2] == grid[6] {
            if grid[2] == 'X' {
                return GameState::XWins;
            }
            return GameState::OWins;
        }
    }
    if grid[3] != ' ' {
        if grid[3] == grid[4] && grid[3] == grid[5] {
            if grid[3] == 'X' {
                return GameState::XWins;
            }
            return GameState::OWins;
        }
    }
    if grid[6] != ' ' {
        if grid[6] == grid[7] && grid[6] == grid[8] {
            if grid[6] == 'X' {
                return GameState::XWins;
            }
            return GameState::OWins;
        }
    }
    if grid_is_full(&grid) {
        return GameState::Draw;
    }
    return GameState::InProgress;
}

// Check whether the target grid is full i.e. every cell is occupied.
fn grid_is_full(grid: &Grid) -> bool {
    !grid.contains(&' ')
}

// Check whether the target slot is a slot which can be occupied.
fn is_free_slot(slot: Slot) -> bool {
    slot == ' '
}

#[cfg(test)]
mod tests {

    use crate::{check_game_state, input_to_slot_index, GameState};

    #[test]
    fn input_to_slot_index_returns_error_for_invalid_input() {
        assert_eq!(input_to_slot_index("").is_err(), true);
        assert_eq!(input_to_slot_index("A0").is_err(), true);
        assert_eq!(input_to_slot_index("A4").is_err(), true);
        assert_eq!(input_to_slot_index("D1").is_err(), true);
        assert_eq!(input_to_slot_index("AA").is_err(), true);
        assert_eq!(input_to_slot_index("11").is_err(), true);
    }

    #[test]
    fn input_to_slot_index_returns_valid_indexes() {
        assert_eq!(input_to_slot_index("A1").unwrap(), 0);
        assert_eq!(input_to_slot_index("B1").unwrap(), 1);
        assert_eq!(input_to_slot_index("C1").unwrap(), 2);
        assert_eq!(input_to_slot_index("A2").unwrap(), 3);
        assert_eq!(input_to_slot_index("B2").unwrap(), 4);
        assert_eq!(input_to_slot_index("C2").unwrap(), 5);
        assert_eq!(input_to_slot_index("A3").unwrap(), 6);
        assert_eq!(input_to_slot_index("B3").unwrap(), 7);
        assert_eq!(input_to_slot_index("C3").unwrap(), 8);
    }

    #[test]
    fn check_game_state_for_x_wins() {
        check_game_state_for_wins('X', GameState::XWins);
    }

    #[test]
    fn check_game_state_for_o_wins() {
        check_game_state_for_wins('O', GameState::OWins);
    }

    fn check_game_state_for_wins(tag: char, expected_state: GameState) {
        assert_eq!(
            check_game_state([tag, tag, tag, ' ', ' ', ' ', ' ', ' ', ' ']),
            expected_state
        );
        assert_eq!(
            check_game_state([' ', ' ', ' ', tag, tag, tag, ' ', ' ', ' ']),
            expected_state
        );
        assert_eq!(
            check_game_state([' ', ' ', ' ', ' ', ' ', ' ', tag, tag, tag]),
            expected_state
        );
        assert_eq!(
            check_game_state([tag, ' ', ' ', tag, ' ', ' ', tag, ' ', ' ']),
            expected_state
        );
        assert_eq!(
            check_game_state([' ', tag, ' ', ' ', tag, ' ', ' ', tag, ' ']),
            expected_state
        );
        assert_eq!(
            check_game_state([' ', ' ', tag, ' ', ' ', tag, ' ', ' ', tag]),
            expected_state
        );
        assert_eq!(
            check_game_state([tag, ' ', ' ', ' ', tag, ' ', ' ', ' ', tag]),
            expected_state
        );
        assert_eq!(
            check_game_state([' ', ' ', tag, ' ', tag, ' ', tag, ' ', ' ']),
            expected_state
        );
    }

    #[test]
    fn check_game_state_for_draws() {
        assert_eq!(
            check_game_state(['X', 'O', 'X', 'O', 'X', 'O', 'O', 'X', 'O']),
            GameState::Draw
        )
        // TODO Add more checks.
    }

    #[test]
    fn check_game_state_for_in_progress() {
        assert_eq!(
            check_game_state([' ', 'O', 'X', 'O', 'X', 'O', 'O', 'X', 'O']),
            GameState::InProgress,
        );
        // TODO Add more checks.
    }
}
