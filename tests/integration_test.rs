use tic_tac_toe;

#[test]
fn quit_with_main_menu_exit() {
    tic_tac_toe::run(&|_| {}, &|| String::from("2"));
}

#[test]
fn retry_main_menu_on_invalid_input() {
    tic_tac_toe::run(&|_| {}, &|| {
        static mut COUNTER: u32 = 0;
        unsafe {
            COUNTER += 1;
            match COUNTER {
                1 => "3".to_string(),
                2 => "2".to_string(),
                _ => panic!("Test should not reach this block!"),
            }
        }
    });
}

#[test]
fn quit_after_victory() {
    // O O O
    // _ X X
    // _ _ _
    tic_tac_toe::run(&|_| {}, &|| {
        static mut COUNTER: u32 = 0;
        unsafe {
            COUNTER += 1;
            match COUNTER {
                1 => "1".to_string(),
                2 => "A1".to_string(),
                3 => "B2".to_string(),
                4 => "B1".to_string(),
                5 => "C2".to_string(),
                6 => "C1".to_string(),
                _ => panic!("Test should not reach this block!"),
            }
        }
    });
}

#[test]
fn quit_after_draw() {
    // O X O
    // X O X
    // X O O
    tic_tac_toe::run(&|_| {}, &|| {
        static mut COUNTER: u32 = 0;
        unsafe {
            COUNTER += 1;
            match COUNTER {
                1 => "1".to_string(),
                2 => "A1".to_string(),
                3 => "B1".to_string(),
                4 => "C1".to_string(),
                5 => "A2".to_string(),
                6 => "B2".to_string(),
                7 => "A3".to_string(),
                8 => "B3".to_string(),
                9 => "C2".to_string(),
                10 => "C3".to_string(),
                _ => panic!("Test should not reach this block!"),
            }
        }
    });
}
