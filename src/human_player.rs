use board::Board;
use piston_window::*;
use values::*;
use wrapper_types::*;

use std::collections::{HashMap,LinkedList};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum InputAction {
    AntiClockwise,
    Clockwise,
    Left,
    Right,
    Turbo,
    DebugLoadBoard,
    DebugAttack,
    DebugBreaker(Color),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum JoystickDirection {
    Left,
    Right,
    Up,
    Down,
}

impl JoystickDirection {
    fn to_axis(&self) -> u8 {
        match *self {
            JoystickDirection::Left => 0,
            JoystickDirection::Right => 0,
            JoystickDirection::Up => 1,
            JoystickDirection::Down => 1,
        }
    }

    fn from_axis(axis: u8, position: f64) -> Option<Self> {
        if position > 0.0 {
            match axis {
                0 => Some(JoystickDirection::Right),
                1 => Some(JoystickDirection::Down),
                _ => {
                    println!("Unknown joystick axis: {}", axis);
                    None
                }
            }
        } else {
            match axis {
                0 => Some(JoystickDirection::Left),
                1 => Some(JoystickDirection::Up),
                _ => {
                    println!("Unknown joystick axis: {}", axis);
                    None
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum InputButton {
    Piston(Button),
    Joystick(i32, JoystickDirection),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum InputEvent {
    Press(InputButton),
    Release(InputButton),
}

enum PressState {
    Initial(f64),
    Repeat(f64),
    Release,
}

pub struct HumanPlayer {
    input_map: HashMap<InputButton, InputAction>,
    held: HashMap<InputButton, PressState>,
}

impl HumanPlayer {
    pub fn new(left: bool) -> Self {
        let mut inputs = HashMap::new();

        if left {
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::W)), InputAction::AntiClockwise);
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::S)), InputAction::Clockwise);
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::A)), InputAction::Left);
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::D)), InputAction::Right);
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::C)), InputAction::Turbo);
            inputs.insert(InputButton::Piston(Button::Joystick(JoystickButton::new(0, 1))), InputAction::Clockwise);
            inputs.insert(InputButton::Piston(Button::Joystick(JoystickButton::new(0, 3))), InputAction::AntiClockwise);
            inputs.insert(InputButton::Joystick(0, JoystickDirection::Left), InputAction::Left);
            inputs.insert(InputButton::Joystick(0, JoystickDirection::Right), InputAction::Right);
            inputs.insert(InputButton::Joystick(0, JoystickDirection::Down), InputAction::Turbo);
        } else {
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::Up)), InputAction::AntiClockwise);
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::Down)), InputAction::Clockwise);
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::Left)), InputAction::Left);
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::Right)), InputAction::Right);
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::Space)), InputAction::Turbo);
            inputs.insert(InputButton::Piston(Button::Joystick(JoystickButton::new(1, 1))), InputAction::Clockwise);
            inputs.insert(InputButton::Piston(Button::Joystick(JoystickButton::new(1, 3))), InputAction::AntiClockwise);
            inputs.insert(InputButton::Joystick(1, JoystickDirection::Left), InputAction::Left);
            inputs.insert(InputButton::Joystick(1, JoystickDirection::Right), InputAction::Right);
            inputs.insert(InputButton::Joystick(1, JoystickDirection::Down), InputAction::Turbo);
        }

        if cfg!(debug_assertions) {
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::Q)), InputAction::DebugLoadBoard);
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::D1)), InputAction::DebugBreaker(Color::Red));
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::D2)), InputAction::DebugBreaker(Color::Green));
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::D3)), InputAction::DebugBreaker(Color::Blue));
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::D4)), InputAction::DebugBreaker(Color::Yellow));
            inputs.insert(InputButton::Piston(Button::Keyboard(Key::D5)), InputAction::DebugAttack);
        }

        HumanPlayer {
            input_map: inputs,
            held: HashMap::new(),
        }
    }

    pub fn update(&mut self, e: &GameWindow, board: &mut Board) {
        let mut events: LinkedList<InputEvent> = LinkedList::new();
        let key_repeat = 0.05;
        let initial_key_repeat = 0.2;

        e.update(|args| {
            let mut to_delete = LinkedList::new();
            for (button, state) in self.held.iter_mut() {
                match *state {
                    PressState::Initial(t) => {
                        let t = t + args.dt;

                        *state = if t > initial_key_repeat {
                            events.push_back(InputEvent::Press(*button));
                            PressState::Repeat(0.0)
                        } else {
                            PressState::Initial(t)
                        }
                    },
                    PressState::Repeat(t) => {
                        let t = t + args.dt;

                        *state = if t > key_repeat {
                            events.push_back(InputEvent::Press(*button));
                            PressState::Repeat(0.0)
                        } else {
                            PressState::Repeat(t)
                        }
                    },
                    PressState::Release => {
                        to_delete.push_back(*button);
                        events.push_back(InputEvent::Release(*button));
                    }
                }
            }

            for button in to_delete {
                self.held.remove(&button);
            }
        });

        if let Some(JoystickAxisArgs { axis, position, id }) = e.joystick_axis_args() {
            let dead_zone = 0.8;

            if position.abs() > dead_zone {
                let direction = JoystickDirection::from_axis(axis, position);

                match direction {
                    Some(direction) => {
                        let button = InputButton::Joystick(id, direction);
                        self.held.insert(button, PressState::Initial(0.0));
                        events.push_back(InputEvent::Press(button));
                    },
                    None => {}
                }
            } else {
                // Generate up events for all held directions for the axis.
                for (button, state) in self.held.iter_mut() {
                    match button {
                        &InputButton::Joystick(held_id, direction) if
                            id == held_id && axis == direction.to_axis() => {

                            *state = PressState::Release;
                        },
                        _ => {}
                    }
                }
            }
        }

        if let Some(button) = e.release_args() {
            let button = InputButton::Piston(button);
            self.held.insert(button, PressState::Release);
            events.push_back(InputEvent::Release(button));
        }
        if let Some(button) = e.press_args() {
            let button = InputButton::Piston(button);
            self.held.insert(button, PressState::Initial(0.0));
            events.push_back(InputEvent::Press(button));
        }

        for event in events {
            match event {
                InputEvent::Press(button) => {
                    if let Some(action) = self.input_map.get(&button) {
                        match action {
                            &InputAction::AntiClockwise => {
                                board.rotate(Rotation::AntiClockwise);
                            },
                            &InputAction::Clockwise => {
                                board.rotate(Rotation::Clockwise);
                            },
                            &InputAction::Left => {
                                board.move_piece(|current| { current.offset(Direction::Left) });
                            },
                            &InputAction::Right => {
                                board.move_piece(|current| { current.offset(Direction::Right) });
                            },
                            &InputAction::Turbo => {
                                board.turbo(true);
                            },
                            &InputAction::DebugBreaker(color) => {
                                if cfg!(debug_assertions) {
                                    board.set_next_piece(Piece::new(
                                        Block::new(color, true),
                                        Block::new(color, true),
                                    ))
                                }

                            },
                            &InputAction::DebugAttack => {
                                if cfg!(debug_assertions) {
                                    board.attack(6);
                                }
                            },
                            &InputAction::DebugLoadBoard => {
                                if cfg!(debug_assertions) {
                                    use std::io::{BufReader,BufRead};
                                    use std::fs::File;

                                    let f = File::open("board.txt");

                                    match f {
                                        Err(e) => {
                                            println!("Could not open file: {}", e);
                                        },
                                        Ok(f) => {
                                            let reader = BufReader::new(&f);
                                            let eol: &[_] = &['\n', '\r'];
                                            let lines: Vec<_> = reader.lines().map(|x| {
                                                x.ok().unwrap()
                                                    .trim_right_matches(eol)
                                                    .to_string()
                                            }).collect();
                                            board.add_blocks(lines);
                                            board.fuse_blocks();
                                        }
                                    }
                                }
                            },
                        }
                    }
                },
                InputEvent::Release(button) => {
                    if let Some(action) = self.input_map.get(&button) {
                        match action {
                            &InputAction::Turbo => {
                                board.turbo(false)
                            },
                            _ => {},
                        }
                    }
                },
            }
        }
    }
}
