use {make_board};
use puzzlefighter::*;

#[test]
fn rotate_anticlockwise_no_kick() {
    let mut board = make_board!(
        "  ",
        "  ",
        "  "
    );
    board.set_current_piece(Piece::rand(1, 1));
    let piece = board.rotate(Rotation::AntiClockwise).unwrap();

    assert_eq!(GridPosition::new(1, 1), piece.position);
    assert_eq!(Direction::Left, piece.direction);
}

#[test]
fn kick_on_left_anticlockwise() {
    let mut board = make_board!(
        "  ",
        "  ",
        "  "
    );
    board.set_current_piece(Piece::rand(0, 1));
    let piece = board.rotate(Rotation::AntiClockwise).unwrap();

    assert_eq!(GridPosition::new(1, 1), piece.position);
    assert_eq!(Direction::Left, piece.direction);
}

#[test]
fn kick_upside_down_on_anticlockwise() {
    let mut board = make_board!(
        " R",
        " R",
        " R"
    );
    board.set_current_piece(Piece::rand(0, 1));
    let piece = board.rotate(Rotation::AntiClockwise).unwrap();

    assert_eq!(GridPosition::new(0, 2), piece.position);
    assert_eq!(Direction::Down, piece.direction);
}

#[test]
fn kick_upside_down_twice_on_anticlockwise() {
    let mut board = make_board!(
        " R",
        " R",
        " R"
    );
    board.set_current_piece(Piece::rand(0, 1));
    board.rotate(Rotation::AntiClockwise);
    let piece = board.rotate(Rotation::AntiClockwise).unwrap();

    assert_eq!(GridPosition::new(0, 1), piece.position);
    assert_eq!(Direction::Up, piece.direction);
}


#[test]
fn rotate_clockwise_no_kick() {
    let mut board = make_board!(
        "  ",
        "  ",
        "  "
    );
    board.set_current_piece(Piece::rand(1, 1));
    let piece = board.rotate(Rotation::Clockwise).unwrap();

    assert_eq!(GridPosition::new(1, 1), piece.position);
    assert_eq!(Direction::Right, piece.direction);
}

#[test]
fn kick_on_right_clockwise() {
    let mut board = make_board!(
        "  R",
        "  R",
        "  R"
    );
    board.set_current_piece(Piece::rand(1, 1));
    let piece = board.rotate(Rotation::Clockwise).unwrap();

    assert_eq!(GridPosition::new(0, 1), piece.position);
    assert_eq!(Direction::Right, piece.direction);
}

#[test]
fn kick_upside_down_on_clockwise() {
    let mut board = make_board!(
        " R",
        " R",
        " R"
    );
    board.set_current_piece(Piece::rand(0, 1));
    let piece = board.rotate(Rotation::Clockwise).unwrap();

    assert_eq!(GridPosition::new(0, 2), piece.position);
    assert_eq!(Direction::Down, piece.direction);
}

#[test]
fn kick_upside_down_twice_on_clockwise() {
    let mut board = make_board!(
        " R",
        " R",
        " R"
    );
    board.set_current_piece(Piece::rand(0, 1));
    board.rotate(Rotation::Clockwise);
    let piece = board.rotate(Rotation::Clockwise).unwrap();

    assert_eq!(GridPosition::new(0, 1), piece.position);
    assert_eq!(Direction::Up, piece.direction);
}

#[test]
fn floor_kick_clockwise() {
    let mut board = make_board!(
        "  ",
        "  "
    );
    board.set_current_piece(Piece::rand(0, 0).rotate(Rotation::Clockwise));
    let piece = board.rotate(Rotation::Clockwise).unwrap();

    assert_eq!(GridPosition::new(0, 1), piece.position);
    assert_eq!(Direction::Down, piece.direction);
}

#[test]
fn floor_kick_anti_clockwise() {
    let mut board = make_board!(
        "  ",
        "  "
    );
    board.set_current_piece(Piece::rand(1, 0).rotate(Rotation::AntiClockwise));
    let piece = board.rotate(Rotation::AntiClockwise).unwrap();

    assert_eq!(GridPosition::new(1, 1), piece.position);
    assert_eq!(Direction::Down, piece.direction);
}

#[test]
fn floor_kick_clockwise_flip() {
    let mut board = make_board!(
        "R ",
        "  "
    );
    board.set_current_piece(Piece::rand(0, 0).rotate(Rotation::Clockwise));
    let piece = board.rotate(Rotation::Clockwise).unwrap();

    assert_eq!(GridPosition::new(1, 0), piece.position);
    assert_eq!(Direction::Left, piece.direction);
}

#[test]
fn floor_kick_anti_clockwise_flip() {
    let mut board = make_board!(
        " R",
        "  "
    );
    board.set_current_piece(Piece::rand(1, 0).rotate(Rotation::AntiClockwise));
    let piece = board.rotate(Rotation::Clockwise).unwrap();

    assert_eq!(GridPosition::new(0, 0), piece.position);
    assert_eq!(Direction::Right, piece.direction);
}

// Prevent infinite delay on floor
#[test]
fn double_floor_kick_forbidden() {
    let mut board = make_board!(
        "   ",
        "   "
    );
    board.set_current_piece(Piece::rand(1, 0).rotate(Rotation::Clockwise));
    // Full rotation puts piece in same place but one position higher
    board.rotate(Rotation::Clockwise);
    board.rotate(Rotation::Clockwise);
    board.rotate(Rotation::Clockwise);
    board.rotate(Rotation::Clockwise);

    // Drop piece per normal clock
    board.move_piece(|current| current.offset(Direction::Down));

    // Rotating again should no longer kick
    let piece = board.rotate(Rotation::Clockwise).unwrap();

    // This is a bit weird, the piece flips horizontally.  Leaving for now, but
    // probably better to clean up and make it [(1, 0) Right]. Still needs to
    // be able to flip horizontally though if you move it under a ledge.
    assert_eq!(GridPosition::new(2, 0), piece.position);
    assert_eq!(Direction::Left, piece.direction);
}

#[test]
fn double_wall_kick_allowed() {
    let mut board = make_board!(
        "  ",
        "  ",
        "  "
    );
    board.set_current_piece(Piece::rand(0, 1));
    // Full rotation puts piece in same place but one position higher
    board.rotate(Rotation::AntiClockwise);
    board.rotate(Rotation::AntiClockwise);
    board.rotate(Rotation::AntiClockwise);
    board.rotate(Rotation::AntiClockwise);

    board.move_piece(|current| current.offset(Direction::Left));

    let piece = board.rotate(Rotation::AntiClockwise).unwrap();

    assert_eq!(GridPosition::new(1, 1), piece.position);
    assert_eq!(Direction::Left, piece.direction);
}
