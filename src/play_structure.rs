use std::io::stdin;

use crate::{
    base_tools::alienify_output_text, board::*, interface::parse_square, pieces::PieceColor,
};

fn play_chess() {
    let white_turn_string = "WHITE TO MOVE";
    let black_turn_string = "BLACK TO MOVE";
    let mut game_not_over = true;
    let mut current_board = Board::default();
    let mut board_states: Vec<Board> = Vec::new();
    while game_not_over {
        let turn_string = if current_board.get_turn() == PieceColor::White {
            white_turn_string
        } else {
            black_turn_string
        };
        let not_first_turn = board_states.len() > 0;
        println!("{}", turn_string);
        print!("{}", current_board);

        alienify_output_text("Please enter a selection:");
        alienify_output_text("1: move");
        alienify_output_text("2: show legal moves from square");
        alienify_output_text("3: show previous turn");
        alienify_output_text("4: surrender");

        let mut indication = String::new();

        stdin()
            .read_line(&mut indication)
            .expect("Failed to read line");

        let indication_number: u32 = match indication.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("It appears you entered something that wasn't a positive integer. Oops!");
                continue;
            }
        };

        match indication_number {
            1 => match move_piece_on_board(&mut current_board, &mut board_states) {
                MoveResult::BlackKingCheckmated => {
                    alienify_output_text("Black king has been put in checkmate. The game is over.");
                }
                MoveResult::WhiteKingCheckmated => {
                    alienify_output_text("White king has been put in checkmate. The game is over.");
                }
                MoveResult::Stalemate => {
                    println!("The game has ended in a stalemate!");
                }
                _ => {}
            },
            2 => {
                query_legal_squares(&current_board);
            }
            3 => {
                if not_first_turn {
                    show_previous_board_state(&board_states);
                } else {
                    println!("Oops! Looks like it's still the first turn.");
                }
            }
            4 => {
                println!("{} has surrendered.", current_board.get_turn_full());
                game_not_over = false;
            }
            _ => alienify_output_text("Hey friend, I think you entered an invalid number"),
        }
    }
}

fn show_previous_board_state(previous_states: &Vec<Board>) {
    let turn_text = format!("You are on turn {}", previous_states.len() - 1);
    loop {
        alienify_output_text(&turn_text);
        alienify_output_text("Choose one of the following:");
        alienify_output_text("1: Show previous turn");
        alienify_output_text("2: Show turn N");
        alienify_output_text("3: Exit");

        let mut indication = String::new();

        stdin()
            .read_line(&mut indication)
            .expect("Failed to read line");

        let indication_number: u32 = match indication.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("It appears you entered something that wasn't a positive integer. Oops!");
                continue;
            }
        };

        match indication_number {
            1 => match previous_states.last() {
                Some(previous_state) => {
                    println!("Previous turn:\n{}", previous_state);
                    break;
                }
                None => {
                    panic!("Oops! Looks like there both is, and isn't a previous board state!");
                }
            },
            2 => {
                show_specific_turn_state(previous_states);
                break;
            }
            3 => {
                break;
            }
            _ => {
                println!("Oops! That wasn't one of the options.");
            }
        }
    }
}

fn show_specific_turn_state(previous_states: &Vec<Board>) {
    loop {
        let turn_text = format!("You are on turn {}", previous_states.len() - 1);
        alienify_output_text(&turn_text);
        alienify_output_text("Please enter a turn to view, or X to exit: ");
        let mut indication = String::new();

        stdin()
            .read_line(&mut indication)
            .expect("Failed to read line");

        if indication.to_lowercase().chars().nth(0).unwrap() == 'x' {
            break;
        }

        let indication_number: usize = match indication.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                alienify_output_text(
                    "It appears you entered something that wasn't a positive integer. Oops!",
                );
                continue;
            }
        };

        if indication_number == 0 || indication_number > previous_states.len() {
            alienify_output_text(
                "Oops! Looks like you entered a number outside the range of previous turns!",
            );
            continue;
        }

        match previous_states.get(indication_number - 1) {
            Some(board_state) => {
                let formatted_turn_output = format!(
                    "On turn {} the state of the game was:\n{}",
                    indication_number, board_state
                );
                alienify_output_text(&formatted_turn_output);
            }
            None => {
                alienify_output_text("Somehow you access an in-range, but non existant board!");
            }
        }
    }
}

fn move_piece_on_board(current_board: &mut Board, board_states: &mut Vec<Board>) -> MoveResult {
    let mut final_result = MoveResult::CompletedSafely;
    loop {
        alienify_output_text("Please enter a move in the form: a3 b3. Otherwise enter X to exit.");

        let mut indication = String::new();
        let previous_turn_board = current_board.clone();

        stdin()
            .read_line(&mut indication)
            .expect("Failed to read line");

        if indication.to_lowercase().chars().nth(0).unwrap() == 'x' {
            break;
        }

        if indication.len() != 7 {
            alienify_output_text("That input format appears to be incorrect.")
        } else {
            match parse_square(&indication[0..2]) {
                Ok(from) => match parse_square(&indication[3..5]) {
                    Ok(to) => {
                        final_result = current_board.move_piece(&from, &to);
                        match final_result {
                            MoveResult::CompletedSafely => {
                                board_states.push(previous_turn_board);
                                break;
                            }
                            MoveResult::WrongTurn => {
                                println!("Oops! It looks like you tried to move the wrong piece. It's {}'s turn", current_board.get_turn_full());
                            }
                            MoveResult::MoveIllegal => {
                                alienify_output_text(
                                    "It appears that that was an illegal move! I'm sorry.",
                                );
                            }
                            _ => {}
                        }
                    }
                    Err(_) => {
                        println!("That input format appears to be incorrect.")
                    }
                },
                Err(_) => {
                    println!("That input format appears to be incorrect.")
                }
            }
        }
    }
    final_result
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
                alienify_output_text("Legal Moves: ");
                current_board.show_me_legal_squares(&coordinates);
                break;
            }
            Err(_) => alienify_output_text(
                "Please input a valid square in the form of one letter, and one number",
            ),
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
            Err(_) => {
                println!("It appears you entered something that wasn't a positive integer. Oops!");
                continue;
            }
        };

        match indication_number {
            1 => {
                play_chess();
            }
            2 => {
                println!("Goodbye!");
                should_keep_running = false;
            }
            _ => {
                alienify_output_text("Hey friend, I think you entered an invalid number");
            }
        };
    }
}
