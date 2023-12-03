use std::{io, vec};

// An enumeration for available player types.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Player {
    X,
    O,
}

// The definition which player starts the game.
const STARTING_PLAYER: Player = Player::O;

// Return the slot representing the player.
fn player_slot(player: &Player) -> Slot {
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
    MainMenu,
    TurnMenu(Grid, Player),
    Victory(Grid, Player),
    Draw(Grid)
}

pub fn run(output: fn(&str), input: fn() -> String) {
    let mut commands = Vec::new();
    commands.extend(vec![Command::MainMenu]);
    while !commands.is_empty() {
        commands = commands
            .iter()
            .flat_map(|command| execute_command(command, output, input))
            .collect();
    }
    output("Bye!")
}

fn execute_command(command: &Command, output: fn(&str), input: fn() -> String) -> Vec<Command> {
    match command {
        Command::MainMenu => {
            output_main_menu(output);
            handle_main_menu(input)
        }
        Command::TurnMenu(grid, player) => {
            output_turn_menu(output, grid, player);
            handle_turn_menu(input, grid, player)
        }
        Command::Victory(grid, player) => {
            output_victory(output, grid, player);
            vec![]
        }
        Command::Draw(grid) => {
            output_draw(output, grid);
            vec![]
        }
    }
}

fn output_main_menu(output: fn(&str)) {
    output("===================");
    output("=== Tic-Tac-Toe ===");
    output("===================");
    output("");
    output("Please enter a selection:");
    output("[1] Play");
    output("[2] Quit");
}

fn handle_main_menu(input: fn() -> String) -> Vec<Command> {
    match input().trim() {
        "1" => vec![Command::TurnMenu([Slot::Empty; 9], STARTING_PLAYER)],
        "2" => vec![],
        _ => vec![Command::MainMenu],
    }
}

fn output_turn_menu(output: fn(&str), grid: &Grid, player: &Player) {
    output(format!("Current turn: {:?}", player).as_str());
    output(format!("{}", grid_string(grid)).as_str());
    output("");
    output("Please enter a cell e.g. 'B2':");
}

fn handle_turn_menu(input: fn() -> String, grid: &Grid, player: &Player) -> Vec<Command> {
    match input_to_slot_index(input().trim()) {
        Ok(val) => {
            if grid[val] == Slot::Empty {
                let mut new_grid = grid.clone();
                new_grid[val] = player_slot(player);
                match game_state(&new_grid) {
                    GameState::Victory => vec![Command::Victory(new_grid, *player)],
                    GameState::Draw => vec![Command::Draw(new_grid)],
                    GameState::Unfinished => match player {
                        Player::O => vec![Command::TurnMenu(new_grid, Player::X)],
                        Player::X => vec![Command::TurnMenu(new_grid, Player::O)],
                    },
                }
            } else {
                vec![Command::TurnMenu(*grid, *player)]
            }
        }
        Err(_) => vec![Command::TurnMenu(*grid, *player)],
    }
}

fn output_victory(output: fn(&str), grid: &Grid, player: &Player) {
    output(format!("{}", grid_string(grid)).as_str());
    output("");
    output(format!("Player {:?} wins the game! Congratulations!", player).as_str());
}

fn output_draw(output: fn(&str), grid: &Grid) {
    output(format!("{}", grid_string(grid)).as_str());
    output("");
    output("Game ends in a draw! Better luck next time!");
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
