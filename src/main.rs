mod base_tools;
mod board;
mod fen_parser;
mod interface;
mod pieces;
mod play_structure;
mod rules;
mod unit_tests;

use play_structure::run_chess_interface;
fn main() {
    run_chess_interface();
}
