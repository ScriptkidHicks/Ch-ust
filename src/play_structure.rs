
use std::io::stdin;

use crate::{board::{self, *}, unit_tests::*, base_tools::alienify_output_text};

fn play_chess() {
    let mut game_not_over = true;
    let mut current_board = Board::default();
    let mut board_states: Vec<Board> = Vec::new();
    print!("{}", current_board);
    println!("Play Chess");
}

pub fn run_chess_interface() {
    let mut should_keep_running = true;

    while should_keep_running {
        alienify_output_text("Please enter a number selection from the following:");
        alienify_output_text("1: Run Unit Tests");
        alienify_output_text("2: Play Chess");
        alienify_output_text("3: Run Current Test");
        alienify_output_text("4: Exit");

        let mut indication = String::new();

        stdin()
            .read_line(&mut indication)
            .expect("Failed to read line");

        let indication_number: u32 = match indication.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match indication_number {
            1 => {
                run_all_unit_tests();
            },
            2 => {
                play_chess();
            },
            3 => {
                run_current_test();
            },
            4 => {
                should_keep_running = false;
            },
            _ => {
                alienify_output_text("Hey friend, I think you entered an invalid number");
            }
        };
    }
}