use std::{
    sync::{Arc, Mutex},
};

#[derive(serde::Serialize, Debug)]
pub struct AxisInfo {
    axis_name: String,
    speed: f32,
    index: f32,
    current: f32,
    org: bool,
    cw: bool,
    ccw: bool,
    setup: bool,
    alm: bool,
    pulse: bool,
    coin: bool,
}

pub trait AxisManage {
    fn get_axis_data(&self) -> Vec<AxisInfo>;
}

#[derive(Default)]
pub struct AxisManager;

pub struct VirtualManager;

impl AxisManage for VirtualManager {
    fn get_axis_data(&self) -> Vec<AxisInfo> {
        let axis_names = vec!["X", "Y1", "Y2", "Z1", "Z2", "TT"];
        let mut axis_data = vec![];
        for axis_name in axis_names {
            axis_data.push(AxisInfo {
                axis_name: String::from(axis_name),
                speed: 10.0,
                index:5.0,
                current: 21f32,
                org: true,
                cw: false,
                ccw: false,
                setup: false,
                alm: false,
                pulse: false,
                coin: false,
            })
        }
        axis_data
    }
}
impl Default for VirtualManager {
    fn default() -> Self {
        VirtualManager {}
    }
}

#[cfg(feature = "virtual_motion")]
pub type AxisDriverU = AxisManager;
#[cfg(not(feature = "virtual_motion"))]
pub type AxisDriverU = VirtualManager;

pub struct AppState {
    pub axis_manager: Arc<Mutex<AxisDriverU>>,
}
