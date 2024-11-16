
use std::{io::stdin, ops::Index};

use crate::{base_tools::alienify_output_text, board::{self, *}, interface::parse_square, pieces::PieceColor, unit_tests::*};

fn play_chess() {
    let white_turn_string = "WHITE TO MOVE";
    let black_turn_string = "BLACK TO MOVE";
    let mut game_not_over = true;
    let mut current_board = Board::default();
    let mut board_states: Vec<Board> = Vec::new();
    while game_not_over {
        let turn_string = if current_board.get_turn() == PieceColor::White {white_turn_string} else {black_turn_string};
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
                                MoveResult::WrongTurn => {
                                    println!("Oops! It looks like you tried to move the wrong piece. It's {}'s turn", current_board.get_turn());
                                },
                                result => {
                                    println!("{}", result);
                                    break;}
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
        alienify_output_text("1: Play Chess");
        alienify_output_text("2: Exit");

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
                play_chess();
            },
            2 => {
                should_keep_running = false;
            },
            _ => {
                alienify_output_text("Hey friend, I think you entered an invalid number");
            }
        };
    }
}