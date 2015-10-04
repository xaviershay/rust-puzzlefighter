use {make_board};
use puzzlefighter::*;

#[test]
fn count_groups_empty() {
    let mut board = make_board!(
        ""
    );
    assert_eq!(0, board.grid().count_groups());
}

#[test]
fn count_groups_one() {
    let mut board = make_board!(
        " R",
        "RR"
    );
    assert_eq!(1, board.grid().count_groups());
}

#[test]
fn count_groups_two() {
    let mut board = make_board!(
        "GGR",
        "GRR"
    );
    assert_eq!(2, board.grid().count_groups());
}

#[test]
fn count_groups_distinct() {
    let mut board = make_board!(
        "R R",
        "R R"
    );
    assert_eq!(2, board.grid().count_groups());
}
