use {make_board,assert_block,assert_no_block};

#[test]
fn drop_to_bottom() {
    let mut board = make_board!(
        "R",
        " "
    );

    board.drop_blocks();

    assert_block(&board, 0, 0);
    assert_no_block(&board, 0, 1);
}

#[test]
fn drop_on_other_block() {
    let mut board = make_board!(
        "R",
        " ",
        "R"
    );

    board.drop_blocks();

    assert_block(&board, 0, 0);
    assert_block(&board, 0, 1);
    assert_no_block(&board, 0, 2);
}

#[test]
fn drop_fused() {
    let mut board = make_board!(
        "BB",
        "BB",
        " "
    );

    board.drop_blocks();

    assert_block(&board, 0, 0);
    assert_block(&board, 1, 0);
    assert_no_block(&board, 0, 2);
    assert_no_block(&board, 1, 2);
}

#[test]
fn fused_does_not_drop_unless_all_below_spaces_are_open() {
    let mut board = make_board!(
        "RR",
        "RR",
        "G "
    );

    board.drop_blocks();
    assert_no_block(&board, 1, 0);

    let mut board = make_board!(
        "RR",
        "RR",
        " G"
    );

    board.drop_blocks();
    assert_no_block(&board, 0, 0);
}

#[test]
fn adjacent_fused_drops_correctly() {
    let mut board = make_board!(
        "RRYY",
        "RRYY",
        "G "
    );

    board.drop_blocks();

    board.debug();
    assert_no_block(&board, 1, 0);
    assert_block(&board, 2, 0);
    assert_block(&board, 3, 0);
    assert_no_block(&board, 2, 2);
    assert_no_block(&board, 3, 2);
}
