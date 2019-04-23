#![deny(missing_docs)]

//! A Piston library for tracking key/mouse press events
//! from windows for use in update loops

extern crate input;

use input::Button;
use std::collections::HashMap;

/// Master struct for the button controller.
/// Consumes button data from any source
/// (though most commonly window event loop generated press events)
/// and tracks them in an update loop friendly form.
#[derive(Clone)]
pub struct ButtonController {
    current_buttons: HashMap<Button, bool>,
    last_buttons: HashMap<Button, bool>,
}

impl ButtonController {
    /// Creates a new ButtonController.
    pub fn new() -> ButtonController {
        ButtonController {
            current_buttons: HashMap::new(),
            last_buttons: HashMap::new(),
        }
    }

    /// Copies the current input state into the last known button state.
    /// Call this in your update loop logic, once per update.
    pub fn update(&mut self) {
        for (button, state) in &self.current_buttons {
            self.last_buttons.insert(*button, *state);
        }
    }

    /// Tracks that a button is currently pressed.
    /// The most common way to do so is passing in Button contents
    /// from PressEvents generated from your relevant window class.
    pub fn register_press(&mut self, button: &Button) {
        self.current_buttons.insert(*button, true);
    }

    /// Tracks that a button is currently un-pressed
    /// (or notes that a previously pressed button is now not pressed).
    /// The most common way to do so is passing in Button contents
    /// from ReleaseEvents generated from your relevant window class.
    pub fn register_release(&mut self, button: &Button) {
        self.current_buttons.insert(*button, false);
    }

    /// Checks if a button is pressed as of the current update.
    /// Will return false if a button has never been registered as pressed.
    pub fn current_pressed(&self, button: &Button) -> bool {
        if let Some(state) = self.current_buttons.get(button) {
            *state
        } else {
            false
        }
    }

    /// Checks if a button is pressed as of the last update.
    /// Will return false if a button has never been registered as pressed.
    pub fn last_pressed(&self, button: &Button) -> bool {
        if let Some(state) = self.last_buttons.get(button) {
            *state
        } else {
            false
        }
    }

    /// Checks for a combination of a button press/release across
    /// the current and last update.
    /// Most useful for determining whether a button was just pressed
    /// or released.
    pub fn pressed_state(&self,
                         button: &Button,
                         last_state: bool,
                         current_state: bool)
                         -> bool {
        self.last_pressed(button) == last_state && self.current_pressed(button) == current_state
    }
}
