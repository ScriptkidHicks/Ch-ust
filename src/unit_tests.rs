use std::str;

use crate::board::Board;

#[derive(Clone)]
struct UnitTest {
    test_name: &'static str,
    board: Board
}


impl UnitTest {  
    pub fn run_self_test(&self) {}
}

static ALL_UNIT_TESTS: Vec<UnitTest> = Vec::new();

pub fn run_current_test() {}

pub fn run_all_unit_tests() {
    for unit_test in ALL_UNIT_TESTS.clone() {
        unit_test.run_self_test();
    }
}
