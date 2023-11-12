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
    println!("  | A | B | C |");
    println!("------------------");
    println!("1 | {} | {} | {} | 1", grid[0], grid[1], grid[2]);
    println!("------------------");
    println!("2 | {} | {} | {} | 2", grid[3], grid[4], grid[5]);
    println!("------------------");
    println!("3 | {} | {} | {} | 3", grid[6], grid[7], grid[8]);
    println!("------------------");
    println!("  | A | B | C |");
    println!("");
    println!("Please enter a cell e.g. 'B2':");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input_to_slot_index(&input) {
            Ok(val) => {
                if is_free_slot(grid[val]) {
                    let mut newGrid = grid;
                    newGrid[val] = player;
                    if resolve_winner(newGrid).is_some() {
                        println!();
                        println!("Current turn: {}", player);
                        println!("  | A | B | C |");
                        println!("------------------");
                        println!("1 | {} | {} | {} | 1", newGrid[0], newGrid[1], newGrid[2]);
                        println!("------------------");
                        println!("2 | {} | {} | {} | 2", newGrid[3], newGrid[4], newGrid[5]);
                        println!("------------------");
                        println!("3 | {} | {} | {} | 3", newGrid[6], newGrid[7], newGrid[8]);
                        println!("------------------");
                        println!("  | A | B | C |");
                        println!("");
                        println!("Player {} wins the game! Congratulations!", player);
                    } else {
                        if player == 'O' {
                            play(newGrid, 'X');
                        } else {
                            play(newGrid, 'O');
                        }
                    }
                }
            }
            Err(_) => show_menu(),
        },
        Err(error) => println!("error: {error}"),
    }
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

// Resolve whether there is a winner for the given grid.
fn resolve_winner(grid: Grid) -> Option<char> {
    if grid[0] != ' ' {
        if grid[0] == grid[3] && grid[0] == grid[6] {
            return Some(grid[0]);
        }
    }
    if grid[1] != ' ' {
        if grid[1] == grid[4] && grid[1] == grid[7] {
            return Some(grid[1]);
        }
    }
    if grid[2] != ' ' {
        if grid[2] == grid[5] && grid[2] == grid[8] {
            return Some(grid[2]);
        }
    }
    if grid[3] != ' ' {
        if grid[3] == grid[4] && grid[3] == grid[5] {
            return Some(grid[3]);
        }
    }
    if grid[6] != ' ' {
        if grid[6] == grid[7] && grid[6] == grid[8] {
            return Some(grid[6]);
        }
    }
    return None;
}

// Check whether the target slot is a slot which can be occupied.
fn is_free_slot(slot: Slot) -> bool {
    slot == ' '
}

#[cfg(test)]
mod tests {
    use crate::input_to_slot_index;

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
}
