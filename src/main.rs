use board::{Board, RowLetter, Coordinates};
use interface::parse_square;

mod board;
mod pieces;
mod interface;
fn main() {
    let valid_length = parse_square("h3");
    let invalid_length = parse_square("eeee");

   match valid_length {
       Err(msg) =>  println!("{}", msg),
       Ok(result) => ()
   }

   match invalid_length {
       Err(msg) =>  println!("{}", msg),
       Ok(result) => ()
   }
}
