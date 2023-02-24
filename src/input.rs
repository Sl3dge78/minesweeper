use sdl2::{keyboard::KeyboardState, mouse::MouseState, mouse::RelativeMouseState, EventPump};

pub struct Input<'a> {
    pub keyboard: KeyboardState<'a>,
    pub rel_mouse: RelativeMouseState,
    previous_mouse: MouseState,
    pub mouse: MouseState,
}

impl<'a> Input<'a> {
    pub fn from_pump(event_pump: &'a EventPump) -> Input {
        Input {
            keyboard : event_pump.keyboard_state(),
            rel_mouse : event_pump.relative_mouse_state(),
            previous_mouse : event_pump.mouse_state(),
            mouse : event_pump.mouse_state(),
        }
    }

    pub fn update(&mut self, event_pump: &'a EventPump) {
        self.keyboard = event_pump.keyboard_state();
        self.rel_mouse = event_pump.relative_mouse_state();
        self.previous_mouse = self.mouse;
        self.mouse = event_pump.mouse_state();
    }
}
