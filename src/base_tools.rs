use std::{thread, time};

pub fn alienify_output_text(output_string: &str) {
    print!("{}", output_string);
    let sleep_time = time::Duration::from_millis(150);
    thread::sleep(sleep_time);
    println!("");
}
