//! Keyboard handling for winit.
//!
//! TODO: Get the correct scan codes for the statics. Will still rely on the newtype.
use std::cmpd::

use wgpu::winit::{KeyboardInput, VirtualKeyCode, ElementState, ModifiersState};

/// Newtype for `KeyboardInput`. The `Eq` impl skips comparison on the scan code.
#[derive(Debug, Copy, Clone)]
pub struct KeyEvent(pub KeyboardInput);



pub static no_mod: ModifiersState = ModifiersState {
    shift: false, ctrl: false, alt: false, logo: false,
};

macro_rules! make_key_event {
    ($name:ident, $vkc:ty) => {
        pub static $name: KeyEvent = KeyEvent(KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode: Some($vkc),
            modifiers: no_mod,
            scancode: 0,
        });

        
    }
}

pub static d_left: KeyEvent = KeyEvent(KeyboardInput {
    state: ElementState::Pressed,
    virtual_keycode: Some(VirtualKeyCode::Left),
    modifiers: no_mod,
    scancode: 0,
});

pub static d_right: KeyEvent = KeyEvent(KeyboardInput {
    state: ElementState::Pressed,
    virtual_keycode: Some(VirtualKeyCode::Right),
    modifiers: no_mod,
    scancode: 0,
});

pub static d_up: KeyEvent = KeyEvent(KeyboardInput {
    state: ElementState::Pressed,
    virtual_keycode: Some(VirtualKeyCode::Up),
    modifiers: no_mod,
    scancode: 0,
});

pub static d_down: KeyEvent = KeyEvent(KeyboardInput {
    state: ElementState::Pressed,
    virtual_keycode: Some(VirtualKeyCode::Down),
    modifiers: no_mod,
    scancode: 0,
});

