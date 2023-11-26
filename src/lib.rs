use std::{collections::VecDeque, io};

type Slot = char;
type Grid = [Slot; 9];

// The definition which player starts the game.
const STARTING_PLAYER: char = 'O';

enum Command {
    Exit,
    Print {
        val: String,
    },
    WaitInput {
        grid: Grid,
        player: char,
        handler: fn(&str, grid: &Grid, player: char) -> Vec<Command>,
    },
}

pub fn run() {
    let mut commands = VecDeque::new();
    commands.push_back(print_main_menu());
    commands.push_back(Command::WaitInput {
        grid: [' '; 9],
        player: STARTING_PLAYER,
        handler: handle_main_menu_input,
    });
    while !commands.is_empty() {
        match commands.pop_front() {
            None => panic!("PANIC: Empty command queue was popped!"),
            Some(command) => match command {
                Command::Exit => break,
                Command::Print { val } => println!("{val}"),
                Command::WaitInput {
                    grid,
                    player,
                    handler,
                } => {
                    let mut input = String::new();
                    match io::stdin().read_line(&mut input) {
                        Ok(_) => commands.extend(handler(input.trim(), &grid, player)),
                        Err(_) => todo!(),
                    };
                }
            },
        }
    }
    println!("Bye!")
}

fn handle_main_menu_input(input: &str, grid: &Grid, player: char) -> Vec<Command> {
    match input {
        "1" => {
            vec![
                print_turn_menu(grid, player),
                Command::WaitInput {
                    grid: *grid,
                    player: player,
                    handler: handle_turn_menu_input,
                },
            ]
        }
        "2" => vec![Command::Exit],
        _ => vec![
            print_main_menu(),
            Command::WaitInput {
                grid: *grid,
                player: player,
                handler: handle_main_menu_input,
            },
        ],
    }
}

fn handle_turn_menu_input(input: &str, grid: &Grid, player: char) -> Vec<Command> {
    match input_to_slot_index(&input) {
        Ok(val) => {
            if grid[val] == ' ' {
                let mut new_grid = grid.clone();
                new_grid[val] = player;
                if has_win(&new_grid) {
                    vec![print_victory(grid, player), Command::Exit]
                } else if has_free(&new_grid) {
                    if player == 'O' {
                        vec![
                            print_turn_menu(&new_grid, 'X'),
                            Command::WaitInput {
                                grid: new_grid,
                                player: 'X',
                                handler: handle_turn_menu_input,
                            },
                        ]
                    } else {
                        vec![
                            print_turn_menu(&new_grid, 'O'),
                            Command::WaitInput {
                                grid: new_grid,
                                player: 'O',
                                handler: handle_turn_menu_input,
                            },
                        ]
                    }
                } else {
                    vec![print_draw(grid), Command::Exit]
                }
            } else {
                vec![
                    print_turn_menu(grid, player),
                    Command::WaitInput {
                        grid: *grid,
                        player: player,
                        handler: handle_turn_menu_input,
                    },
                ]
            }
        }
        Err(_) => vec![
            print_turn_menu(grid, player),
            Command::WaitInput {
                grid: *grid,
                player: player,
                handler: handle_turn_menu_input,
            },
        ],
    }
}

fn print_main_menu() -> Command {
    Command::Print {
        val: format!(
            "
===================
=== Tic-Tac-Toe ===
===================

Please enter a selection:
[1] Play
[2] Quit

"
        ),
    }
}

fn print_turn_menu(grid: &Grid, player: char) -> Command {
    Command::Print {
        val: format!(
            "

Current turn: {}
{}

Please enter a cell e.g. 'B2':

",
            player,
            grid_string(grid)
        ),
    }
}

fn grid_string(grid: &Grid) -> String {
    format!(
        "
  | A | B | C |
------------------
1 | {} | {} | {} | 1
------------------
2 | {} | {} | {} | 2
------------------
3 | {} | {} | {} | 3
------------------
  | A | B | C |
  ",
        grid[0], grid[1], grid[2], grid[3], grid[4], grid[5], grid[6], grid[7], grid[8]
    )
}

fn print_victory(grid: &Grid, player: char) -> Command {
    Command::Print {
        val: format!(
            "

{}

Player {} wins the game! Congratulations!
",
            grid_string(grid),
            player
        ),
    }
}

fn print_draw(grid: &Grid) -> Command {
    Command::Print {
        val: format!(
            "
{}            

Game ends in a draw! Better luck next time!
            ",
            grid_string(grid)
        ),
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
