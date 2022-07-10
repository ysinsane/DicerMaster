use std::{
    string,
    sync::{Arc, Mutex},
};

#[derive(serde::Serialize)]
pub struct AxisInfo {
    axis_name: String,
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
        vec![AxisInfo {
            axis_name: String::from("X"),
            current: 20f32,
            org: true,
            cw: false,
            ccw: false,
            setup: false,
            alm: false,
            pulse: false,
            coin: false,
        }]
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
