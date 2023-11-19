use std::io;

type Slot = char;
type Grid = [Slot; 9];

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
            "1" => play(&[' '; 9], 'O'),
            "2" => println!("Bye!"),
            _ => show_menu(),
        },
        Err(error) => println!("error: {error}"),
    }
}

fn play(grid: &Grid, player: char) {
    print_turn_menu(grid, player);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input_to_slot_index(&input) {
            Ok(val) => {
                if grid[val] == ' ' {
                    let mut new_grid = grid.clone();
                    new_grid[val] = player;
                    if has_win(&new_grid) {
                        print_victory(&grid, player);
                    } else if has_free(&new_grid) {
                        if player == 'O' {
                            play(&new_grid, 'X');
                        } else {
                            play(&new_grid, 'O');
                        }
                    } else {
                        print_draw(&new_grid);
                    }
                }
            }
            Err(_) => show_menu(),
        },
        Err(error) => println!("error: {error}"),
    }
}

// Print the main menu into the standard output.
fn print_main_menu() {
    println!("===================");
    println!("=== Tic-Tac-Toe ===");
    println!("===================");
    println!();
    println!("Please enter a selection:");
    println!("[1] Play");
    println!("[2] Quit");
}

// Print the turn menu into the standard output.
fn print_turn_menu(grid: &Grid, player: char) {
    println!();
    println!("Current turn: {}", player);
    print_grid(&grid);
    println!("");
    println!("Please enter a cell e.g. 'B2':");
}

// Print the provided grid into the standard output.
fn print_grid(grid: &Grid) {
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

// Print the victory into the standard output.
fn print_victory(grid: &Grid, player: char) {
    println!();
    print_grid(grid);
    println!("");
    println!("Player {} wins the game! Congratulations!", player);
}

// Print the draw into the standard output.
fn print_draw(grid: &Grid) {
    println!();
    print_grid(grid);
    println!("");
    println!("Game ends in a draw! Better luck next time!");
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

// Check whether the given grid contains a winning line.
fn has_win(grid: &Grid) -> bool {
    (grid[0] != ' ' && grid[0] == grid[1] && grid[1] == grid[2])
        || (grid[3] != ' ' && grid[3] == grid[4] && grid[4] == grid[5])
        || (grid[6] != ' ' && grid[6] == grid[7] && grid[7] == grid[8])
        || (grid[0] != ' ' && grid[0] == grid[3] && grid[3] == grid[6])
        || (grid[1] != ' ' && grid[1] == grid[4] && grid[4] == grid[7])
        || (grid[2] != ' ' && grid[2] == grid[5] && grid[5] == grid[8])
        || (grid[0] != ' ' && grid[0] == grid[4] && grid[4] == grid[8])
        || (grid[2] != ' ' && grid[2] == grid[4] && grid[4] == grid[6])
}

// Check whether the given grid contains a free cell.
fn has_free(grid: &Grid) -> bool {
    grid.contains(&' ')
}

#[cfg(test)]
mod tests {

    use crate::{has_free, has_win, input_to_slot_index};

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
    fn has_win_returns_true_when_win() {
        assert!(has_win(&['X', 'X', 'X', ' ', ' ', ' ', ' ', ' ', ' ']));
        assert!(has_win(&[' ', ' ', ' ', 'X', 'X', 'X', ' ', ' ', ' ']));
        assert!(has_win(&[' ', ' ', ' ', ' ', ' ', ' ', 'X', 'X', 'X']));
        assert!(has_win(&['X', ' ', ' ', 'X', ' ', ' ', 'X', ' ', ' ']));
        assert!(has_win(&[' ', 'X', ' ', ' ', 'X', ' ', ' ', 'X', ' ']));
        assert!(has_win(&[' ', ' ', 'X', ' ', ' ', 'X', ' ', ' ', 'X']));
        assert!(has_win(&['X', ' ', ' ', ' ', 'X', ' ', ' ', ' ', 'X']));
        assert!(has_win(&[' ', ' ', 'X', ' ', 'X', ' ', 'X', ' ', ' ']));
    }

    #[test]
    fn has_win_returns_false_when_no_win() {
        assert!(!has_win(&['X', 'O', 'X', ' ', ' ', ' ', ' ', ' ', ' ']));
        assert!(!has_win(&[' ', ' ', ' ', 'X', 'O', 'X', ' ', ' ', ' ']));
        assert!(!has_win(&[' ', ' ', ' ', ' ', ' ', ' ', 'X', 'O', 'X']));
        assert!(!has_win(&['X', ' ', ' ', 'O', ' ', ' ', 'X', ' ', ' ']));
        assert!(!has_win(&[' ', 'X', ' ', ' ', 'O', ' ', ' ', 'X', ' ']));
        assert!(!has_win(&[' ', ' ', 'X', ' ', ' ', 'O', ' ', ' ', 'X']));
        assert!(!has_win(&['X', ' ', ' ', ' ', 'O', ' ', ' ', ' ', 'X']));
        assert!(!has_win(&[' ', ' ', 'X', ' ', 'O', ' ', 'X', ' ', ' ']));
    }

    #[test]
    fn has_free_returns_true_when_free_cell_exists() {
        assert!(has_free(&[' ', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X']));
        assert!(has_free(&['X', ' ', 'X', 'X', 'X', 'X', 'X', 'X', 'X']));
        assert!(has_free(&['X', 'X', ' ', 'X', 'X', 'X', 'X', 'X', 'X']));
        assert!(has_free(&['X', 'X', 'X', ' ', 'X', 'X', 'X', 'X', 'X']));
        assert!(has_free(&['X', 'X', 'X', 'X', ' ', 'X', 'X', 'X', 'X']));
        assert!(has_free(&['X', 'X', 'X', 'X', 'X', ' ', 'X', 'X', 'X']));
        assert!(has_free(&['X', 'X', 'X', 'X', 'X', 'X', ' ', 'X', 'X']));
        assert!(has_free(&['X', 'X', 'X', 'X', 'X', 'X', 'X', ' ', 'X']));
        assert!(has_free(&['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', ' ']));
    }

    #[test]
    fn has_free_returns_false_when_no_free_cell_exists() {
        assert!(!has_free(&['O', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X']));
        assert!(!has_free(&['X', 'O', 'X', 'X', 'X', 'X', 'X', 'X', 'X']));
        assert!(!has_free(&['X', 'X', 'O', 'X', 'X', 'X', 'X', 'X', 'X']));
        assert!(!has_free(&['X', 'X', 'X', 'O', 'X', 'X', 'X', 'X', 'X']));
        assert!(!has_free(&['X', 'X', 'X', 'X', 'O', 'X', 'X', 'X', 'X']));
        assert!(!has_free(&['X', 'X', 'X', 'X', 'X', 'O', 'X', 'X', 'X']));
        assert!(!has_free(&['X', 'X', 'X', 'X', 'X', 'X', 'O', 'X', 'X']));
        assert!(!has_free(&['X', 'X', 'X', 'X', 'X', 'X', 'X', 'O', 'X']));
        assert!(!has_free(&['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'O']));
    }
}
