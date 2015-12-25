use piston_window::*;
use std::collections::{HashMap,LinkedList};

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

enum PressState {
    Initial(f64),
    Repeat(f64),
    Release,
}

pub struct InputController {
    //input_map: HashMap<InputButton, InputAction>,
    held: HashMap<InputButton, PressState>,
}

impl InputController {
    fn map(&mut self, e: &GameWindow) -> LinkedList<InputEvent> {
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
    }
}
