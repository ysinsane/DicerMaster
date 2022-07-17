#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use app::{
    motion::{AxisConifg, MotionConfig},
    AppState, AxisInfo, Motion, MotionError,
};

#[derive(serde::Serialize)]
struct CustomResponse<T> {
    message: String,
    code: i32,
    data: Option<T>,
}

impl<T: Default> Default for CustomResponse<T> {
    fn default() -> Self {
        CustomResponse {
            message: String::from("success"),
            code: 200,
            data: Some(T::default()),
        }
    }
}

#[derive(serde::Serialize)]
struct AppError {
    message: String,
    code: i32,
}

impl From<MotionError> for AppError {
    fn from(err: MotionError) -> Self {
        AppError {
            code: 40000,
            message: format!("{:?}", err),
        }
    }
}

#[tauri::command]
async fn get_all_axis_data(state: tauri::State<'_, AppState>) -> Result<CustomResponse<Vec<AxisInfo>>, AppError> {
    let arc_clone = Arc::clone(&state.axis_manager);
    let axis_manager = arc_clone.lock().unwrap();
    let data = axis_manager.get_all_axis_data()?;
    let res = CustomResponse {
        message: String::from("success"),
        code: 200,
        data: Some(data),
    };
    Ok(res)
}

#[tauri::command]
async fn get_axis_configs(state: tauri::State<'_, AppState>) -> Result<CustomResponse<Vec<AxisConifg>>, AppError> {
    let arc_clone = Arc::clone(&state.axis_manager);
    let axis_manager = arc_clone.lock().unwrap();
    let data = axis_manager.get_axis_configs();
    let res = CustomResponse {
        message: String::from("success"),
        code: 200,
        data: Some(data),
    };
    Ok(res)
}

#[tauri::command]
async fn init_axis_config(state: tauri::State<'_, AppState>) -> Result<CustomResponse<()>, AppError> {
    let arc_clone = Arc::clone(&state.axis_manager);
    let mut axis_manager = arc_clone.lock().unwrap();

    let axis_names = vec!["X", "Y1", "Y2", "Z1", "Z2"];

    let x = axis_names
        .iter()
        .map(|name| AxisConifg {
            axis_name: name.to_string(),
            speed: 2000.00001,
            index: 10.1,
            max_work_speed: 12.1,
            init_position: 0.0,
        })
        .collect();

    let motion_config = MotionConfig { axis_configs: x };

    axis_manager.init_config(motion_config)?;
    let res = CustomResponse {
        message: String::from("success"),
        code: 200,
        data: None,
    };
    Ok(res)
}

fn main() {
    tauri::Builder::default()
        .any_thread()
        .manage(AppState {
            axis_manager: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            get_all_axis_data,
            get_axis_configs,
            init_axis_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
