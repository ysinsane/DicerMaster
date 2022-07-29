use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

pub struct MotionDriverError {
    pub code: i32,
}

pub trait MotionDriver: Sync + Send {
    fn abs_move(&self, axis_name: String, target: f64, speed: f64) -> Result<(), MotionDriverError>;
    fn get_stop_reason(&self, axis_name: String) -> Result<(), MotionDriverError>;
    fn stop(&self, axis_name: String) -> Result<(), MotionDriverError>;
    fn get_position(&self, axis_name: &str) -> Result<f64, MotionDriverError>;
    fn check_done(&self, axis_name:&str)->Result<bool, MotionDriverError>;
}

pub struct VirtualMotionDriver {
    axises: Arc<Mutex<RefCell<HashMap<String, (f64, bool)>>>>,
}

impl VirtualMotionDriver {
    pub fn new() -> Self {
        VirtualMotionDriver {
            axises: Default::default(),
        }
    }
    fn update_postion(axises: Arc<Mutex<RefCell<HashMap<String, (f64, bool)>>>>, axis_name: &str, target: f64) {
        let axises = axises.lock().unwrap();
        let mut pos = axises.borrow_mut();
        let position = pos.entry(axis_name.to_string()).or_insert((0.0, false));
        *position = (target, position.1);
    }
    fn is_moving(axises: &Arc<Mutex<RefCell<HashMap<String, (f64, bool)>>>>, axis_name: &str) -> bool {
        let axises = axises.lock().unwrap();
        let mut pos = axises.borrow_mut();
        let position = pos.entry(axis_name.to_string()).or_insert((0.0, false));
        println!("轴{axis_name}是否在动：{}", position.1);
        position.1
    }
    fn unset_moving(axises: &Arc<Mutex<RefCell<HashMap<String, (f64, bool)>>>>, axis_name: &str) {
        let axises = axises.lock().unwrap();
        let mut pos = axises.borrow_mut();
        let position = pos.entry(axis_name.to_string()).or_insert((0.0, false));
        *position = (position.0, false);
    }
    fn set_moving(axises: &Arc<Mutex<RefCell<HashMap<String, (f64, bool)>>>>, axis_name: &str) {
        let axises = axises.lock().unwrap();
        let mut pos = axises.borrow_mut();
        let position = pos.entry(axis_name.to_string()).or_insert((0.0, false));
        *position = (position.0, true);
    }
}

impl MotionDriver for VirtualMotionDriver {
    fn abs_move(&self, axis_name: String, target: f64, speed: f64) -> Result<(), MotionDriverError> {
        println!("绝对移动{axis_name}，目标：{target}，速度：{speed}");

        let mut current = self.get_position(&axis_name)?;
        let axises = Clone::clone(&self.axises);
        Self::set_moving(&axises, &axis_name);
        std::thread::spawn(move || -> Result<(), MotionDriverError> {
            let direction = if current < target { 1.0 } else { -1.0 };
            while current != target && Self::is_moving(&axises, &axis_name) {
                std::thread::sleep(Duration::from_millis(100));
                let mut new_target = current + speed / 10.0 * direction;
                if new_target * direction > target * direction {
                    new_target = target
                }
                current = new_target;
                Self::update_postion(Clone::clone(&axises), &axis_name, new_target);
            }
            Self::unset_moving(&axises, &axis_name);
            Ok(())
        });
        Ok(())
    }

    fn get_stop_reason(&self, axis_name: String) -> Result<(), MotionDriverError> {
        println!("获取{axis_name}停止原因");
        Ok(())
    }

    fn stop(&self, axis_name: String) -> Result<(), MotionDriverError> {
        // smc_stop
        println!("停止{axis_name}轴");

        Self::unset_moving(&self.axises, &axis_name);
        Ok(())
    }

    fn get_position(&self, axis_name: &str) -> Result<f64, MotionDriverError> {
        //smc_get_position_unit
        let position = self
            .axises
            .lock()
            .unwrap()
            .borrow_mut()
            .entry(axis_name.to_string())
            .or_insert((0.0, false))
            .clone();
        println!("获取{axis_name}轴的位置:{}", position.0);
        Ok(position.0)
    }

    fn check_done(&self, axis_name:&str)->Result<bool, MotionDriverError>{
        //smc_check_done
        let moving = Self::is_moving(&self.axises, axis_name);
        Ok(!moving)
    }
}
