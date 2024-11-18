mod play_structure;
mod board;
mod pieces;
mod rules;
mod unit_tests;
mod base_tools;
mod interface;
mod fen_parser;

use play_structure::run_chess_interface;
fn main() {
    run_chess_interface();
}
