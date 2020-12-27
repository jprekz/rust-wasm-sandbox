#[cfg(not(target_arch = "wasm32"))]
use glutin as winit;

pub use winit::event::WindowEvent as Event;

pub use winit::{
    dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize, Pixel, Position, Size},
    event::{
        AxisId, DeviceId, ElementState, Force, KeyboardInput, ModifiersState, MouseButton,
        MouseScrollDelta, ScanCode, Touch, TouchPhase, VirtualKeyCode,
    },
    window::Theme,
};
