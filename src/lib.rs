extern crate input;

use input::Button;
use std::collections::HashMap;


#[derive(Clone)]
pub struct ButtonController {
    current_buttons: HashMap<Button, bool>,
    last_buttons: HashMap<Button, bool>,
}

impl ButtonController {
    pub fn new() -> ButtonController {
        ButtonController {
            current_buttons: HashMap::new(),
            last_buttons: HashMap::new(),
        }
    }

    pub fn update(&mut self) {
        for (button, state) in &self.current_buttons {
            self.last_buttons.insert(*button, *state);
        }
    }
    pub fn register_press(&mut self, button: &Button) {
        self.current_buttons.insert(*button, true);
    }

    pub fn register_release(&mut self, button: &Button) {
        self.current_buttons.insert(*button, false);
    }

    pub fn current_pressed(&mut self, button: &Button) -> bool {
        if let Some(state) = self.current_buttons.get(button) {
            *state
        } else {
            false
        }
    }

    pub fn last_pressed(&mut self, button: &Button) -> bool {
        if let Some(state) = self.current_buttons.get(button) {
            *state
        } else {
            false
        }
    }

    pub fn pressed_state(&mut self, button: &Button, last_state: bool, current_state: bool) -> bool {
        self.last_pressed(button) == last_state && self.current_pressed(button) == current_state
    }

}
