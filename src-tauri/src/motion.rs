use std::{string::String, time::Duration};

use crate::motion_driver::{MotionDriver, MotionDriverError};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct MoveParam {
    pub axis_name: String,
    pub speed: Option<f64>,
    pub destination: Option<f64>,
}

#[derive(serde::Serialize, Debug)]
pub struct AxisInfo {
    pub axis_name: String,
    pub current: f64,
    pub io_status: AxisIoStatus,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct AxisConifg {
    pub axis_name: String,
    pub speed: f64,
    pub index: f64,
    pub max_work_speed: f64,
    pub init_position: f64,
}

#[derive(serde::Serialize, Debug)]
pub struct AxisIoStatus {
    pub org: bool,
    pub cw: bool,
    pub ccw: bool,
    pub setup: bool,
    pub alm: bool,
    pub pulse: bool,
    pub coin: bool,
}

pub struct MotionConfig {
    pub axis_configs: Vec<AxisConifg>,
    pub connect_string: String,
}

impl Default for MotionConfig {
    fn default() -> Self {
        Self {
            axis_configs: Default::default(),
            connect_string: Default::default(),
        }
    }
}

#[derive(Debug)]
pub enum MotionError {
    ConnectionError,
    NotInitialnized,
    DriverError(i32),
}

impl From<MotionDriverError> for MotionError {
    fn from(err: MotionDriverError) -> Self {
        MotionError::DriverError(err.code)
    }
}

pub type MotionResult<T> = Result<T, MotionError>;

/// 每个函数都打开一个连接
pub trait Motion {
    fn abs_move(&self, move_params: Vec<MoveParam>) -> Result<(), MotionError>;
    fn wait_axises(&self, move_params: Vec<String>) -> Result<bool, MotionError>;
    fn stop_axis(&self, move_params: String) -> Result<(), MotionError>;
    fn get_all_axis_data(&self) -> MotionResult<Vec<AxisInfo>>;
    fn get_axis_posion(&self, axis_name: &str) -> Result<f64, MotionError>;
    fn get_axis_io(&self, axis_name: &str) -> Result<AxisIoStatus, MotionError>;
    /// 获取单根轴的位置、IO信息
    ///
    /// # Errors
    ///
    /// This function will return an error if 轴的位置IO信息无法读取
    fn get_axis_data(&self, axis_name: &str) -> Result<AxisInfo, MotionError>;

    /// 初始化轴模块：加载配置，回零各轴
    ///
    /// # Errors
    ///
    /// This function will return an error if 控制器连接失败
    fn init_config(&mut self, config: MotionConfig) -> Result<(), MotionError>;

    fn get_axis_configs(&self) -> Vec<AxisConifg>;

    fn new(driver: Box<dyn MotionDriver>) -> Self;
}

pub struct MotionModule {
    motion_config: MotionConfig,
    driver: Box<dyn MotionDriver>,
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Motion for MotionModule {
    fn abs_move(&self, move_params: Vec<MoveParam>) -> Result<(), MotionError> {
        for move_param in move_params {
            self.driver.abs_move(
                move_param.axis_name,
                move_param.destination.unwrap(),
                move_param.speed.unwrap(),
            )?
        }
        Ok(())
    }

    fn wait_axises(&self, axis_names: Vec<String>) -> Result<bool, MotionError> {
        let mut all_stop = true;
        for axis_name in &axis_names {
            let done = self.driver.check_done(&axis_name)?;
            if !done {
                all_stop = false
            }
        }
        if all_stop {
           return Ok(true)
        }
        Ok(false)
    }

    fn get_all_axis_data(&self) -> MotionResult<Vec<AxisInfo>> {
        let mut axis_data = vec![];
        for axis_cfg in self.get_axis_configs() {
            axis_data.push(self.get_axis_data(&axis_cfg.axis_name)?);
        }
        Ok(axis_data)
    }

    fn get_axis_posion(&self, axis_name: &str) -> Result<f64, MotionError> {
        let pos = self.driver.get_position(axis_name)?;
        Ok(pos)
    }

    fn get_axis_io(&self, axis_name: &str) -> Result<AxisIoStatus, MotionError> {
        Ok(AxisIoStatus {
            org: true,
            cw: true,
            ccw: true,
            setup: true,
            alm: true,
            pulse: true,
            coin: true,
        })
    }

    fn get_axis_data(&self, axis_name: &str) -> Result<AxisInfo, MotionError> {
        // TODO: 从Driver查询数据
        Ok(AxisInfo {
            axis_name: axis_name.to_string(),
            current: self.get_axis_posion(axis_name)?,
            io_status: self.get_axis_io(axis_name)?,
        })
    }

    fn init_config(&mut self, config: MotionConfig) -> Result<(), MotionError> {
        self.motion_config = config;
        Ok(())
    }

    fn get_axis_configs(&self) -> Vec<AxisConifg> {
        self.motion_config.axis_configs.clone()
    }

    fn stop_axis(&self, axis_name: String) -> Result<(), MotionError> {
        self.driver.stop(axis_name)?;
        Ok(())
    }
    fn new(driver: Box<dyn MotionDriver>) -> Self {
        Self {
            motion_config: Default::default(),
            driver: driver,
        }
    }
}
