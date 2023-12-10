use main_menu::Input;

mod main_menu;
mod turn_menu;

// Run the game with the provided input and output channels.
pub fn run(output: fn(&str), input: fn() -> String) {
    let mut command = Option::Some(Command::MainMenu);
    while command.is_some() {
        command = execute_command(&command.unwrap(), output, input);
    }
}

// Execute the given command possibly leading into a next command to be executed.
fn execute_command(command: &Command, output: fn(&str), input: fn() -> String) -> Option<Command> {
    match command {
        Command::MainMenu => {
            output_main_menu(output);
            Option::Some(handle_main_menu(
                main_menu::parse_input(input().trim()).unwrap(), // TODO get rid of unwrap
            ))
        }
        Command::TurnMenu(grid, player) => {
            output_turn_menu(output, grid, player);
            Option::Some(handle_turn_menu(
                turn_menu::parse_input(input().trim()).unwrap(), // TODO get rid of unwrap
                grid,
                player,
            ))
        }
        Command::Victory(grid, player) => {
            output_victory(output, grid, player);
            Option::Some(Command::Exit)
        }
        Command::Draw(grid) => {
            output_draw(output, grid);
            Option::Some(Command::Exit)
        }
        Command::Exit => {
            output("Bye!");
            Option::None
        }
    }
}

// An enumeration for all available player types.
#[derive(Clone, Copy, Debug)]
enum Player {
    X,
    O,
}

// An enumeration for all available slot types.
#[derive(Clone, Copy, PartialEq)]
enum Slot {
    Empty,
    X,
    O,
}

// A type for the game grid containing 3x3 slots.
type Grid = [Slot; 9];

// An enumeration of all available commands within the game.
enum Command {
    MainMenu,
    TurnMenu(Grid, Player),
    Victory(Grid, Player),
    Draw(Grid),
    Exit,
}

// Render the visualization of the main menu.
fn output_main_menu(output: fn(&str)) {
    output("===================");
    output("=== Tic-Tac-Toe ===");
    output("===================");
    output("");
    output("Please enter a selection:");
    output("[1] Play");
    output("[2] Quit");
}

// Handle the input for the main menu.
fn handle_main_menu(input: Input) -> Command {
    match input {
        Input::Start => Command::TurnMenu([Slot::Empty; 9], Player::O),
        Input::Exit => Command::Exit,
    }
}

// Render the visualization of the turn menu.
fn output_turn_menu(output: fn(&str), grid: &Grid, player: &Player) {
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

// Handle the input for the turn menu.
fn handle_turn_menu(input: turn_menu::Input, grid: &Grid, player: &Player) -> Command {
    let index = turn_menu::input_index(input);
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
fn output_victory(output: fn(&str), grid: &Grid, player: &Player) {
    output_grid(output, &grid);
    output("");
    output(format!("Player {:?} wins the game! Congratulations!", player).as_str());
}

// Render the visualization of the draw.
fn output_draw(output: fn(&str), grid: &Grid) {
    output_grid(output, &grid);
    output("");
    output("Game ends in a draw! Better luck next time!");
}

#[cfg(test)]
mod tests {

    use crate::Slot::{Empty, O, X};

    use crate::{has_free, has_win};

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
