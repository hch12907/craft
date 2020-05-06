use std::collections::{ BTreeSet, BTreeMap };
use glutin::event::{ 
    ButtonId,
    DeviceId, 
    DeviceEvent,
    ElementState,
    KeyboardInput
};

pub use glutin::event::VirtualKeyCode as Key;

pub type HeldKeysIter<'a> = std::collections::btree_map::Keys<'a, Key, bool>;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Button4,
}

pub struct InputManager {
    // Is the input suspended?
    suspended: bool,
    
    // Mouse input related
    mouse_id: Option<DeviceId>,
    mouse_button: BTreeSet<MouseButton>,
    mouse_delta: (f64, f64),

    // Keyboard input related
    // * `pressed_keys` basically stores (Key, bHasNotChecked). If the key is
    //   checked before with is_key_pressed(), bHasNotChecked is set to false.
    //   More on is_key_pressed().
    pressed_keys: BTreeMap<Key, bool>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            suspended: true,
            mouse_id: None,
            mouse_button: BTreeSet::new(),
            mouse_delta: (0.0, 0.0),
            pressed_keys: BTreeMap::new(),
        }
    }

    fn keyboard_input(&mut self, input: KeyboardInput) {
        // DISCUSS: Consider using raw scancode? 
        let key = input.virtual_keycode
            .unwrap_or(Key::Yen);
        
        if let ElementState::Pressed = input.state {
            self.pressed_keys.insert(key, true);
        }
        
        if let ElementState::Released = input.state {
            self.pressed_keys.remove(&key);
        }
    }

    fn mouse_button_input(&mut self, button: ButtonId, state: ElementState) {
        let button = match button {
            1 => MouseButton::Left,
            2 => MouseButton::Middle,
            3 => MouseButton::Right,
            4 => MouseButton::Button4,
            _ => unimplemented!()
        };
        
        if let ElementState::Pressed = state {
            self.mouse_button.insert(button);
        }
        
        if let ElementState::Released = state {
            self.mouse_button.remove(&button);
        }
    }

    fn mouse_axis_input(&mut self, id: DeviceId, (x, y): (f64, f64)) {
        if let None = self.mouse_id {
            self.mouse_id = Some(id)
        };

        self.mouse_delta.0 += x;
        self.mouse_delta.1 += y;
    }

    pub fn update_inputs(&mut self, id: DeviceId, event: DeviceEvent) {
        // Skip if suspended
        if self.suspended {
            self.pressed_keys.clear();
            self.mouse_button.clear();
            return
        }

        match event {
            DeviceEvent::Key(input) => self.keyboard_input(input),
            DeviceEvent::MouseMotion { delta } => self.mouse_axis_input(id, delta),
            DeviceEvent::Button { button, state } => self.mouse_button_input(button, state),

            _ => {},
        };
    }

    pub fn get_mouse_delta(&mut self) -> (f64, f64) {
        let temp = self.mouse_delta;
        self.mouse_delta = (0.0, 0.0);
        temp
    }

    pub fn is_key_pressed(&mut self, key: Key) -> bool {
        // The map stores (Key, HasNotChecked). When the key is pressed, only
        // the first check for the key will be true until the key is released
        // and the whole process resets. iterate_held_keys() is not affected
        // by this.
        if let Some(pressed) = self.pressed_keys.get_mut(&key) {
            std::mem::replace(pressed, false)
        } else {
            false
        }
    }

    pub fn iterate_held_keys(&self) -> HeldKeysIter {
        self.pressed_keys.keys()
    }

    pub fn suspend_input(&mut self) {
        self.suspended = true
    }

    pub fn unsuspend_input(&mut self) {
        self.suspended = false
    }
}
