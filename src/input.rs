use sdl2::{keyboard::KeyboardState, mouse::RelativeMouseState, EventPump};

pub struct Input<'a> {
    pub keyboard: KeyboardState<'a>,
    pub mouse: RelativeMouseState,
}

impl<'a> Input<'a> {
    pub fn from_pump(event_pump: &'a EventPump) -> Input<'a> {
        Input {
            keyboard: event_pump.keyboard_state(),
            mouse: event_pump.relative_mouse_state(),
        }
    }
}
