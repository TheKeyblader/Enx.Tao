use interoptopus::ffi;
use tao::event::DeviceId;

#[ffi(opaque, name = "DeviceId")]
pub struct DeviceIdFFI {
    pub inner: DeviceId,
}

#[ffi(name = "KeyEvent")]
pub struct KeyEventFFI {}

#[ffi(name = "RawKeyEvent")]
pub struct RawKeyEventFFI {}

#[ffi(name = "Touch")]
pub struct TouchFFI {}

#[ffi(name = "DeviceEvent")]
pub enum DeviceEventFFI {}

#[ffi(name = "ElementState")]
pub enum ElementStateFFI {}

#[ffi(name = "Event")]
pub enum EventFFI {}

#[ffi(name = "Force")]
pub enum ForceFFI {}

#[ffi(name = "MouseButton")]
pub enum MouseButtonFFI {}

#[ffi(name = "MouseScrollDelta")]
pub enum MouseScrollDeltaFFI {}

#[ffi(name = "StartCause")]
pub enum StartCauseFFI {}

#[ffi(name = "TouchPhase")]
pub enum TouchPhaseFFI {}

#[ffi(name = "WindowEvent")]
pub enum WindowEventFFI {}
