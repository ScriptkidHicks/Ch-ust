
use std::{io::stdin, ops::Index};

use crate::{board::{self, *}, unit_tests::*, base_tools::alienify_output_text, interface::parse_square};

fn play_chess() {
    let mut turn_string = "WHITE TO MOVE";
    let mut game_not_over = true;
    let mut current_board = Board::default();
    let mut board_states: Vec<Board> = Vec::new();
    while game_not_over {
        println!("{}", turn_string);
        print!("{}", current_board);

        alienify_output_text("Please enter a selection:");
        alienify_output_text("1: move");
        alienify_output_text("2: show legal moves from square");
        alienify_output_text("3: surrender");

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
                move_piece_on_board(&mut current_board, &mut board_states);
            },
            2 => {
                query_legal_squares(&current_board);
            },
            3 => {
                game_not_over = false;
            },
            _ => { alienify_output_text("Hey friend, I think you entered an invalid number")}
        }
    }
}

pub fn move_piece_on_board(current_board: &mut Board, board_states: &mut Vec<Board>) {
    loop {
        println!("Please enter a move in the form: a3 b3. Otherwise enter X to exit.");
    
        let mut indication = String::new();
        let previous_turn_board = current_board.clone();
        
        stdin()
            .read_line(&mut indication)
            .expect("Failed to read line");


        if indication.to_lowercase().chars().nth(0).unwrap() == 'x' {
            break;
        }

        if indication.len() != 7 {
            println!("That input format appears to be incorrect.")
        } else {
            println!("first coord: {}", &indication[0..2]);
            println!("second coord: {}", &indication[3..5]);
            match parse_square(&indication[0..2]) {
                Ok(from) => {
                    match parse_square(&indication[3..5]) {
                        Ok(to) => {
                            match current_board.move_piece(&from, &to) {
                                MoveResult::CompletedSafely => {
                                    board_states.push(previous_turn_board);
                                    break;
                                },
                                _ => {break;}
                            }
                        },
                        Err(_) => {
                            println!("That input format appears to be incorrect.")
                        }
                    }
                },
                Err(_) => {println!("That input format appears to be incorrect.")}
            }
        }
    
    }
    
}

pub fn query_legal_squares(current_board: &Board) {
    loop {
        println!("Please enter coordinates, or X to quit:");
    
        let mut indication = String::new();
    
        stdin()
                .read_line(&mut indication)
                .expect("Failed to read line");
        
        if indication.to_lowercase().chars().nth(0).unwrap() == 'x' {
            break;
        }
    
        match parse_square(&indication[0..2]) {
            Ok(coordinates) => {
                print!("Legal Moves: ");
                current_board.show_me_legal_squares(&coordinates);
                break;
            },
            Err(_) => {
                alienify_output_text("Please input a valid square in the form of one letter, and one number")
            }
        }
    }
}

pub fn run_chess_interface() {
    let mut should_keep_running = true;

    while should_keep_running {
        alienify_output_text("Please enter a selection:");
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