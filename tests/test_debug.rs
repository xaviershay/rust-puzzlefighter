use {make_board};

#[test]
// Smoke test to make sure debug function doesn't blow up
fn test_debug() {
    let board = make_board!(
        " ",
        "RRYYBBGG",
        "RRYYBBGG",
        "RGBYrgby"
    );

    board.debug();
}
