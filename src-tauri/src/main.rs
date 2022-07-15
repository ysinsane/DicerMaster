#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use app::{AppState, AxisInfo, Motion};

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

#[allow(dead_code)]
#[allow(unused_variables)]
impl<T> CustomResponse<T> {
    fn error(message: String, code: i32) -> Self {
        CustomResponse {
            message: message,
            code: code,
            data: None,
        }
    }
}

#[tauri::command]
async fn get_all_axis_data(state: tauri::State<'_, AppState>) -> Result<CustomResponse<Vec<AxisInfo>>, ()> {
    let arc_clone = Arc::clone(&state.axis_manager);
    let axis_manager = arc_clone.lock().unwrap();
    let data = axis_manager.get_all_axis_data();
    let res = CustomResponse {
        message: String::from("success"),
        code: 200,
        data: Some(data),
    };
    Ok(res)
}

fn main() {
    tauri::Builder::default()
        .any_thread()
        .manage(AppState {
            axis_manager: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![get_all_axis_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
