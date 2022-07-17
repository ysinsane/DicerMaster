use std::string::String;

#[derive(serde::Serialize, Debug)]
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
}

impl Default for MotionConfig {
    fn default() -> Self {
        Self { axis_configs: Default::default() }
    }
}

#[derive(Debug)]
pub enum MotionError {
    ConnectionError,
    NotInitialnized
}
pub type MotionResult<T> = Result<T, MotionError>;
pub trait Motion {
    fn abs_move(move_params: Vec<MoveParam>) -> Result<(), MotionError>;
    fn wait_axises(move_params: Vec<String>) -> Result<(), MotionError>;
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
}

pub struct MotionModule {
    motion_config: MotionConfig,
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Motion for MotionModule {
    fn abs_move(move_params: Vec<MoveParam>) -> Result<(), MotionError> {
        println!("Axises Moving: {:?}", move_params);
        todo!()
    }

    fn wait_axises(axis_names: Vec<String>) -> Result<(), MotionError> {
        println!("Axises Moving: {:?}", axis_names);
        todo!()
    }

    fn get_all_axis_data(&self) -> MotionResult<Vec<AxisInfo>>{
        let mut axis_data = vec![];
        for axis_cfg in self.get_axis_configs() {
            axis_data.push(self.get_axis_data(&axis_cfg.axis_name)?);
        }
        Ok(axis_data)
    }

    fn get_axis_posion(&self, axis_name: &str) -> Result<f64, MotionError> {
        Ok(0.0)
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
}

impl Default for MotionModule {
    fn default() -> Self {
        Self { motion_config: Default::default() }
    }
}