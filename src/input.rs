//! Input processing. Using the command pattern but instead of returning an `action`, will
//! return a transform to be applied.
use std::collections::HashMap;
use std::ops::Neg;

use winit::event::{KeyboardInput, VirtualKeyCode, ElementState};
use cgmath::{Vector3, Zero, Rad, Deg};

pub type Camera = Vector3<f32>;
pub type RotY = Rad<f32>;
pub type RotX = Rad<f32>;

static SET_CMPX: u16 = 0b0000_0000_0000_0001;
static SET_CMPY: u16 = 0b0000_0000_0000_0010;
static SET_CMPZ: u16 = 0b0000_0000_0000_0100;
static SET_CMNX: u16 = 0b0000_0000_0001_0000;
static SET_CMNY: u16 = 0b0000_0000_0010_0000;
static SET_CMNZ: u16 = 0b0000_0000_0100_0000;
static SET_RSPX: u16 = 0b0000_0001_0000_0000;
static SET_RSPY: u16 = 0b0000_0010_0000_0000;
static SET_RSNX: u16 = 0b0000_0100_0000_0000;
static SET_RSNY: u16 = 0b0000_1000_0000_0000;

static MSK_CMPX: u16 = 0b1111_1111_1111_1110;
static MSK_CMPY: u16 = 0b1111_1111_1111_1101;
static MSK_CMPZ: u16 = 0b1111_1111_1111_1011;
static MSK_CMNX: u16 = 0b1111_1111_1110_1111;
static MSK_CMNY: u16 = 0b1111_1111_1101_1111;
static MSK_CMNZ: u16 = 0b1111_1111_1011_1111;
static MSK_RSPX: u16 = 0b1111_1110_1111_1111;
static MSK_RSPY: u16 = 0b1111_1101_1111_1111;
static MSK_RSNX: u16 = 0b1111_1011_1111_1111;
static MSK_RSNY: u16 = 0b1111_0111_1111_1111;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    CameraMovePX,
    CameraMovePY,
    CameraMovePZ,
    CameraMoveNX,
    CameraMoveNY,
    CameraMoveNZ,
    RotateShapePX,
    RotateShapePY,
    RotateShapeNX,
    RotateShapeNY,
}

impl Action {
    pub fn bitset(&self) -> u16 {
        match self {
            Action::CameraMovePX =>  SET_CMPX,
            Action::CameraMovePY =>  SET_CMPY,
            Action::CameraMovePZ =>  SET_CMPZ,
            Action::CameraMoveNX =>  SET_CMNX,
            Action::CameraMoveNY =>  SET_CMNY,
            Action::CameraMoveNZ =>  SET_CMNZ,
            Action::RotateShapePX => SET_RSPX,
            Action::RotateShapePY => SET_RSPY,
            Action::RotateShapeNX => SET_RSNX,
            Action::RotateShapeNY => SET_RSNY,
        }
    }

    pub fn bitmask(&self) -> u16 {
        match self {
            Action::CameraMovePX =>  MSK_CMPX,
            Action::CameraMovePY =>  MSK_CMPY,
            Action::CameraMovePZ =>  MSK_CMPZ,
            Action::CameraMoveNX =>  MSK_CMNX,
            Action::CameraMoveNY =>  MSK_CMNY,
            Action::CameraMoveNZ =>  MSK_CMNZ,
            Action::RotateShapePX => MSK_RSPX,
            Action::RotateShapePY => MSK_RSPY,
            Action::RotateShapeNX => MSK_RSNX,
            Action::RotateShapeNY => MSK_RSNY,
        }
    }
}

pub trait ActionState {
    fn on(&mut self, action: Action);
    fn off(&mut self, action: Action);
    fn camera_increment(&self, increment: f32) -> Camera;
    fn x_rotation_increment(&self, increment: f32) -> Rad<f32>;
    fn y_rotation_increment(&self, increment: f32) -> Rad<f32>;
}

impl ActionState for u16 {
    fn on(&mut self, action: Action) {
        *self |= action.bitset();
    }

    fn off(&mut self, action: Action) {
        *self &= action.bitmask();
    }

    fn camera_increment(&self, increment: f32) -> Camera {
        let mut camera = Camera::zero();

        if *self & SET_CMPX > 0 { camera.x = increment; }
        if *self & SET_CMNX > 0 { camera.x = increment.neg(); }
        if *self & SET_CMPY > 0 { camera.y = increment; }
        if *self & SET_CMNY > 0 { camera.y = increment.neg(); }
        if *self & SET_CMPZ > 0 { camera.z = increment; }
        if *self & SET_CMNZ > 0 { camera.z = increment.neg(); }

        camera
    }

    fn x_rotation_increment(&self, increment: f32) -> RotX {
        if *self & SET_RSPX > 0 { return Deg(increment).into() }
        if *self & SET_RSNX > 0 { return Deg(increment.neg()).into() }

        Rad(0f32)
    }

    fn y_rotation_increment(&self, increment: f32) -> RotY {
        if *self & SET_RSPY > 0 { return Deg(increment).into() }
        if *self & SET_RSNY > 0 { return Deg(increment.neg()).into() }

        Rad(0f32)
    }
}

/// Which keypresses carry out which which actions and by how much.
pub struct Bindings {
    bindings: HashMap<VirtualKeyCode, Action>,
    camera_increment: f32,
    x_rotation_increment: f32,
    y_rotation_increment: f32,
}

impl Bindings {
    pub fn new(
        camera_increment: f32, x_rotation_increment: f32, y_rotation_increment: f32,
    ) -> Self {
        Bindings {
            bindings: HashMap::new(),
            camera_increment,
            x_rotation_increment,
            y_rotation_increment,
        }
    }

    pub fn bind(&mut self, vkc: VirtualKeyCode, action: Action) -> Option<Action> {
        self.bindings.insert(vkc, action)
    }

    pub fn unbind(&mut self, vkc: &VirtualKeyCode) -> Option<Action> {
        self.bindings.remove(vkc)
    }
}

impl Default for Bindings {
    fn default() -> Self {
        let mut bindings = Bindings::new(0.1f32, 0.5f32, 0.5f32);
        bindings.bind(VirtualKeyCode::W, Action::CameraMoveNY);
        bindings.bind(VirtualKeyCode::S, Action::CameraMovePY);
        bindings.bind(VirtualKeyCode::A, Action::CameraMovePX);
        bindings.bind(VirtualKeyCode::D, Action::CameraMoveNX);
        bindings.bind(VirtualKeyCode::Left, Action::RotateShapePY);
        bindings.bind(VirtualKeyCode::Right, Action::RotateShapeNY);
        bindings.bind(VirtualKeyCode::Up, Action::RotateShapePX);
        bindings.bind(VirtualKeyCode::Down, Action::RotateShapeNX);

        bindings
    }
}

pub fn handle_keyboard<T: ActionState>(
    event: &KeyboardInput, bindings: &Bindings, state: &mut T,
) -> Option<(Camera, RotX, RotY)> {
    let ci = bindings.camera_increment;
    let xri = bindings.x_rotation_increment;
    let yri = bindings.y_rotation_increment;
    let vkc = event.virtual_keycode
        .unwrap_or(VirtualKeyCode::Escape); // Escape is already caught beforehand.

    bindings.bindings
        .get(&vkc)
        .map(|action| {
            match event.state {
                ElementState::Pressed => state.on(*action),
                ElementState::Released => state.off(*action),
            }
            (
                state.camera_increment(ci),
                state.x_rotation_increment(xri),
                state.y_rotation_increment(yri),
            )
        })
}

