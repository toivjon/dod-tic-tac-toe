use std::{collections::VecDeque, io};

// An enumeration for available player types.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Player {
    X,
    O,
}

// The definition which player starts the game.
const STARTING_PLAYER: Player = Player::O;

// Return the slot representing the player.
fn player_slot(player: Player) -> Slot {
    match player {
        Player::O => Slot::O,
        Player::X => Slot::X,
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Slot {
    Empty,
    X,
    O,
}

// Return the char presenting the slot.
fn slot_char(slot: Slot) -> char {
    match slot {
        Slot::Empty => ' ',
        Slot::O => 'O',
        Slot::X => 'X',
    }
}

// A type for the game grid containing 3x3 slots.
type Grid = [Slot; 9];

// Return a string representing the grid contents.
fn grid_string(grid: &Grid) -> String {
    let chars = grid.map(slot_char);
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
        chars[0], chars[1], chars[2], chars[3], chars[4], chars[5], chars[6], chars[7], chars[8]
    )
}

enum Command {
    Print {
        val: String,
    },
    WaitInput {
        grid: Grid,
        player: Player,
        handler: fn(&str, grid: &Grid, player: Player) -> Vec<Command>,
    },
}

pub fn run() {
    let mut commands = VecDeque::new();
    commands.extend(cmd_main_menu([Slot::Empty; 9], STARTING_PLAYER));
    while !commands.is_empty() {
        let new_commands = match commands.pop_front() {
            None => panic!("PANIC: Empty command queue was popped!"),
            Some(command) => execute_command(command),
        };
        commands.extend(new_commands);
    }
    println!("Bye!")
}

fn execute_command(command: Command) -> Vec<Command> {
    match command {
        Command::Print { val } => {
            println!("{val}");
            vec![]
        }
        Command::WaitInput {
            grid,
            player,
            handler,
        } => {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => handler(input.trim(), &grid, player),
                Err(_) => todo!(),
            }
        }
    }
}

fn handle_main_menu_input(input: &str, grid: &Grid, player: Player) -> Vec<Command> {
    match input {
        "1" => cmd_turn_menu(*grid, player),
        "2" => vec![],
        _ => cmd_main_menu(*grid, player),
    }
}

enum GameState {
    Victory,
    Draw,
    Unfinished,
}

fn game_state(grid: &Grid) -> GameState {
    if has_win(grid) {
        GameState::Victory
    } else if has_free(grid) {
        GameState::Unfinished
    } else {
        GameState::Draw
    }
}

fn handle_turn_menu_input(input: &str, grid: &Grid, player: Player) -> Vec<Command> {
    match input_to_slot_index(&input) {
        Ok(val) => {
            if grid[val] == Slot::Empty {
                let mut new_grid = grid.clone();
                new_grid[val] = player_slot(player);
                match game_state(&new_grid) {
                    GameState::Victory => cmd_victory(new_grid, player),
                    GameState::Draw => cmd_draw(new_grid),
                    GameState::Unfinished => match player {
                        Player::O => cmd_turn_menu(new_grid, Player::X),
                        Player::X => cmd_turn_menu(new_grid, Player::O),
                    },
                }
            } else {
                cmd_turn_menu(*grid, player)
            }
        }
        Err(_) => cmd_turn_menu(*grid, player),
    }
}

fn cmd_main_menu(grid: Grid, player: Player) -> Vec<Command> {
    let handler = handle_main_menu_input;
    vec![
        print_main_menu(),
        Command::WaitInput {
            grid,
            player,
            handler,
        },
    ]
}

fn cmd_turn_menu(grid: Grid, player: Player) -> Vec<Command> {
    let handler = handle_turn_menu_input;
    vec![
        print_turn_menu(&grid, player),
        Command::WaitInput {
            grid,
            player,
            handler,
        },
    ]
}

fn cmd_victory(grid: Grid, player: Player) -> Vec<Command> {
    vec![print_victory(&grid, player)]
}

fn cmd_draw(grid: Grid) -> Vec<Command> {
    vec![print_draw(&grid)]
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

fn print_turn_menu(grid: &Grid, player: Player) -> Command {
    Command::Print {
        val: format!(
            "

Current turn: {:?}
{}

Please enter a cell e.g. 'B2':

",
            player,
            grid_string(grid)
        ),
    }
}

fn print_victory(grid: &Grid, player: Player) -> Command {
    Command::Print {
        val: format!(
            "

{}

Player {:?} wins the game! Congratulations!
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

#[cfg(test)]
mod tests {

    use crate::Slot::{Empty, O, X};

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
