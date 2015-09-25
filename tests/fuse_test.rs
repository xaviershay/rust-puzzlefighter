use board_asserts::*;

#[test]
fn fuse_2x2() {
    let board = make_board!(
        "RR",
        "RR"
    );

    assert_fused(&board, 0, 0, SIDE_BOTTOM_LEFT);
    assert_fused(&board, 1, 0, SIDE_BOTTOM_RIGHT);
    assert_fused(&board, 0, 1, SIDE_TOP_LEFT);
    assert_fused(&board, 1, 1, SIDE_TOP_RIGHT);
}

#[test]
fn fuse_2x3() {
    let board = make_board!(
        "YY",
        "YY",
        "YY"
    );

    assert_fused(&board, 0, 0, SIDE_BOTTOM_LEFT);
    assert_fused(&board, 1, 0, SIDE_BOTTOM_RIGHT);
    assert_fused(&board, 0, 1, SIDE_LEFT);
    assert_fused(&board, 1, 1, SIDE_RIGHT);
    assert_fused(&board, 0, 2, SIDE_TOP_LEFT);
    assert_fused(&board, 1, 2, SIDE_TOP_RIGHT);
}

#[test]
fn fuse_3x2() {
    let board = make_board!(
        "YYY",
        "YYY"
    );

    assert_fused(&board, 0, 0, SIDE_BOTTOM_LEFT);
    assert_fused(&board, 1, 0, SIDE_BOTTOM);
    assert_fused(&board, 2, 0, SIDE_BOTTOM_RIGHT);
    assert_fused(&board, 0, 1, SIDE_TOP_LEFT);
    assert_fused(&board, 1, 1, SIDE_TOP);
    assert_fused(&board, 2, 1, SIDE_TOP_RIGHT);
}

#[test]
fn fuse_above_existing_2x2() {
    // We know this case will fuse from other test cases
    let mut board = make_board!(
        "YY",
        "YY"
    );

    board.add_blocks(vec!(
        "YY",
        "",
        ""
    ));
    board.fuse_blocks();

    assert_fused(&board, 0, 0, SIDE_BOTTOM_LEFT);
    assert_fused(&board, 1, 0, SIDE_BOTTOM_RIGHT);
    assert_fused(&board, 0, 1, SIDE_LEFT);
    assert_fused(&board, 1, 1, SIDE_RIGHT);
    assert_fused(&board, 0, 2, SIDE_TOP_LEFT);
    assert_fused(&board, 1, 2, SIDE_TOP_RIGHT);
}

#[test]
fn fuse_below_existing_2x2() {
    // We know this case will fuse from other test cases
    let mut board = make_board!(
        "YY",
        "YY",
        "  "
    );

    board.add_blocks(vec!(
        "  ",
        "  ",
        "YY"
    ));
    board.fuse_blocks();

    assert_fused(&board, 0, 0, SIDE_BOTTOM_LEFT);
    assert_fused(&board, 1, 0, SIDE_BOTTOM_RIGHT);
    assert_fused(&board, 0, 1, SIDE_LEFT);
    assert_fused(&board, 1, 1, SIDE_RIGHT);
    assert_fused(&board, 0, 2, SIDE_TOP_LEFT);
    assert_fused(&board, 1, 2, SIDE_TOP_RIGHT);
}

#[test]
fn fuse_to_right_of_2x2() {
    let mut board = make_board!(
        "YY",
        "YY"
    );

    board.add_blocks(vec!(
        "  Y",
        "  Y"
    ));
    board.fuse_blocks();

    assert_fused(&board, 0, 0, SIDE_BOTTOM_LEFT);
    assert_fused(&board, 1, 0, SIDE_BOTTOM);
    assert_fused(&board, 2, 0, SIDE_BOTTOM_RIGHT);
    assert_fused(&board, 0, 1, SIDE_TOP_LEFT);
    assert_fused(&board, 1, 1, SIDE_TOP);
    assert_fused(&board, 2, 1, SIDE_TOP_RIGHT);
}

#[test]
fn fuse_to_left_of_2x2() {
    let mut board = make_board!(
        " YY",
        " YY"
    );

    board.add_blocks(vec!(
        "Y  ",
        "Y  "
    ));
    board.fuse_blocks();

    assert_fused(&board, 0, 0, SIDE_BOTTOM_LEFT);
    assert_fused(&board, 1, 0, SIDE_BOTTOM);
    assert_fused(&board, 2, 0, SIDE_BOTTOM_RIGHT);
    assert_fused(&board, 0, 1, SIDE_TOP_LEFT);
    assert_fused(&board, 1, 1, SIDE_TOP);
    assert_fused(&board, 2, 1, SIDE_TOP_RIGHT);
}

#[test]
fn fuse_offset_2x2() {
    let board = make_board!(
        " YY",
        " YY",
        "YY ",
        "YY "
    );

    assert_fused(&board, 0, 0, SIDE_BOTTOM_LEFT);
    assert_fused(&board, 1, 0, SIDE_BOTTOM_RIGHT);
    assert_fused(&board, 0, 1, SIDE_TOP_LEFT);
    assert_fused(&board, 1, 1, SIDE_TOP_RIGHT);

    assert_fused(&board, 1, 2, SIDE_BOTTOM_LEFT);
    assert_fused(&board, 2, 2, SIDE_BOTTOM_RIGHT);
    assert_fused(&board, 1, 3, SIDE_TOP_LEFT);
    assert_fused(&board, 2, 3, SIDE_TOP_RIGHT);
}

#[test]
fn fuse_does_not_go_across_corners() {
    let mut board = make_board!(
        "YY  ",
        "YY  ",
        "YY  ",
        " YY",
        " YY"
    );

    board.add_blocks(vec!(
        "    ",
        "  YY",
        "  YY",
        "    ",
        "    ",
    ));
    board.fuse_blocks();

    assert_fused(&board, 1, 0, SIDE_BOTTOM_LEFT);
    assert_fused(&board, 2, 0, SIDE_BOTTOM_RIGHT);
    assert_fused(&board, 1, 1, SIDE_TOP_LEFT);
    assert_fused(&board, 2, 1, SIDE_TOP_RIGHT);

    assert_fused(&board, 0, 2, SIDE_BOTTOM_LEFT);
    assert_fused(&board, 0, 3, SIDE_LEFT);
    assert_fused(&board, 0, 4, SIDE_TOP_LEFT);
    assert_fused(&board, 1, 2, SIDE_BOTTOM_RIGHT);
    assert_fused(&board, 1, 3, SIDE_RIGHT);
    assert_fused(&board, 1, 4, SIDE_TOP_RIGHT);

    assert_fused(&board, 2, 2, SIDE_BOTTOM_LEFT);
    assert_fused(&board, 3, 2, SIDE_BOTTOM_RIGHT);
    assert_fused(&board, 2, 3, SIDE_TOP_LEFT);
    assert_fused(&board, 3, 3, SIDE_TOP_RIGHT);
}

#[test]
fn fuse_l_shape_favours_horizontal() {
    let board = make_board!(
        "YY ",
        "YYY",
        "YYY"
    );

    board.debug();

    assert_fused(&board, 0, 0, SIDE_BOTTOM_LEFT);
    assert_fused(&board, 1, 0, SIDE_BOTTOM);
    assert_fused(&board, 2, 0, SIDE_BOTTOM_RIGHT);
    assert_fused(&board, 0, 1, SIDE_TOP_LEFT);
    assert_fused(&board, 1, 1, SIDE_TOP);
    assert_fused(&board, 2, 1, SIDE_TOP_RIGHT);
}

#[test]
fn fuse_on_top_of_existing_3x2() {
    // We know this case will fuse from other test cases
    let mut board = make_board!(
        "YYY",
        "YYY"
    );

    board.add_blocks(vec!(
        "YYY",
        "",
        ""
    ));
    board.fuse_blocks();

    assert_fused(&board, 0, 0, SIDE_BOTTOM_LEFT);
    assert_fused(&board, 1, 0, SIDE_BOTTOM);
    assert_fused(&board, 2, 0, SIDE_BOTTOM_RIGHT);
    assert_fused(&board, 0, 1, SIDE_LEFT);
    assert_fused(&board, 1, 1, SIDE_NONE);
    assert_fused(&board, 2, 1, SIDE_RIGHT);
    assert_fused(&board, 0, 2, SIDE_TOP_LEFT);
    assert_fused(&board, 1, 2, SIDE_TOP);
    assert_fused(&board, 2, 2, SIDE_TOP_RIGHT);
}


#[test]
fn different_colors_do_not_fuse() {
    let board = make_board!(
        "GR",
        "RR"
    );

    assert_not_fused(&board, 0, 0);
    assert_not_fused(&board, 1, 0);
    assert_not_fused(&board, 0, 1);
    assert_not_fused(&board, 1, 1);
}

#[test]
fn non_blocks_do_not_fuse() {
    let board = make_board!(
        " G",
        "GG"
    );

    assert_not_fused(&board, 0, 0);
    assert_not_fused(&board, 1, 0);
    assert_not_fused(&board, 1, 1);
}

#[test]
fn breakers_do_not_fuse() {
    let board = make_board!(
        "Gg",
        "GG"
    );

    assert_not_fused(&board, 0, 0);
    assert_not_fused(&board, 1, 0);
    assert_not_fused(&board, 0, 1);
    assert_not_fused(&board, 1, 1);
}

