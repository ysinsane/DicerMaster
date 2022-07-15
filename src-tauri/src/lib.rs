pub mod motion;

use std::sync::{Arc, Mutex};

pub use motion::{AxisInfo, Motion, MotionError};

pub struct VirtualManager;

#[allow(dead_code)]
#[allow(unused_variables)]
impl Motion for VirtualManager {
    fn abs_move(move_params: Vec<motion::MoveParam>) -> Result<(), motion::MotionError> {
        todo!()
    }

    fn get_all_axis_data(&self) -> Vec<AxisInfo> {
        let axis_names = vec!["X", "Y1", "Y2", "Z1", "Z2", "TT"];
        let mut axis_data = vec![];
        for axis_name in axis_names {
            axis_data.push(AxisInfo {
                axis_name: String::from(axis_name),
                speed: 10.0,
                index:5.0,
                current: 21f32,
                io_status: motion::AxisIoStatus { org: true, cw: true, ccw: true, setup: true, alm: true, pulse: true, coin: true }
            })
        }
        axis_data
    }

    fn get_axis_data(&self) -> Result<AxisInfo, MotionError> {
        todo!()
    }

    fn wait_axises(move_params: Vec<String>) -> Result<(), MotionError> {
        todo!()
    }

    fn get_axis_posion(axis_name:String) -> Result<f32, MotionError> {
        todo!()
    }

    fn get_axis_io(axis_name:String) -> Result<motion::AxisIoStatus, MotionError> {
        todo!()
    }

    fn init_config(&self, config: motion::MotionConfig) -> Result<(), MotionError> {
        todo!()
    }
}
impl Default for VirtualManager {
    fn default() -> Self {
        VirtualManager {}
    }
}

#[cfg(feature = "virtual_motion")]
pub type AxisManagerUsing = AxisManager;
#[cfg(not(feature = "virtual_motion"))]
pub type AxisManagerUsing = VirtualManager;

pub struct AppState {
    pub axis_manager: Arc<Mutex<AxisManagerUsing>>,
}
