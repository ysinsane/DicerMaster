use std::string::String;

#[derive(serde::Serialize, Debug)]
pub struct MoveParam {
    pub axis_name: String,
    pub speed: Option<f32>,
    pub destination: Option<f32>,
}

#[derive(serde::Serialize, Debug)]
pub struct AxisInfo {
    pub axis_name: String,
    pub speed: f32,
    pub index: f32,
    pub current: f32,
    pub io_status: AxisIoStatus
}

pub struct AxisConifg{
    pub axis_name: String,
    pub speed: f32,
    pub index: f32,
    pub max_work_speed: f32,
    pub init_position: f32,
}

#[derive(serde::Serialize, Debug)]
pub struct AxisIoStatus{
    pub org: bool,
    pub cw: bool,
    pub ccw: bool,
    pub setup: bool,
    pub alm: bool,
    pub pulse: bool,
    pub coin: bool,
}

pub struct MotionConfig {
    pub axis_configs: Vec<AxisConifg>
}

pub enum MotionError {
    ConnectionError,
}
pub trait Motion {
    fn abs_move(move_params: Vec<MoveParam>) -> Result<(), MotionError>;
    fn wait_axises(move_params: Vec<String>) -> Result<(), MotionError>;
    fn get_all_axis_data(&self) -> Vec<AxisInfo>;
    fn get_axis_posion(axis_name:String) -> Result<f32, MotionError>;
    fn get_axis_io(axis_name:String) -> Result<AxisIoStatus, MotionError>;
    /// 获取单根轴的位置、IO信息
    ///
    /// # Errors
    ///
    /// This function will return an error if 轴的位置IO信息无法读取
    fn get_axis_data(&self) -> Result<AxisInfo, MotionError>;

    /// 初始化轴模块：加载配置，回零各轴
    ///
    /// # Errors
    ///
    /// This function will return an error if 控制器连接失败
    fn init_config(&self, config: MotionConfig) -> Result<(), MotionError>;
}

pub struct MotionModule;

#[allow(dead_code)]
#[allow(unused_variables)]
impl Motion for MotionModule {
    fn abs_move(move_params: Vec<MoveParam>) -> Result<(), MotionError> {
        println!("Axises Moving: {:?}", move_params);
        todo!()
    }

    fn get_all_axis_data(&self) -> Vec<AxisInfo> {
        todo!()
    }

    fn get_axis_data(&self) -> Result<AxisInfo, MotionError> {
        todo!()
    }

    fn wait_axises(axis_names: Vec<String>) -> Result<(), MotionError> {
        println!("Axises Moving: {:?}", axis_names);
        todo!()
    }

    fn get_axis_posion(axis_name:String) -> Result<f32, MotionError> {
        todo!()
    }

    fn get_axis_io(axis_name:String) -> Result<AxisIoStatus, MotionError> {
        todo!()
    }

    fn init_config(&self, config: MotionConfig) -> Result<(), MotionError> {
        todo!()
    }
}
