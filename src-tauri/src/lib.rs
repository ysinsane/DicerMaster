pub mod motion;
pub mod motion_driver;

use std::sync::{Arc, Mutex};

use motion::MotionModule;
pub use motion::{AxisInfo, Motion, MotionError};

pub type AxisManagerUsing = MotionModule;

pub struct AppState {
    pub axis_manager: Arc<Mutex<AxisManagerUsing>>,
}