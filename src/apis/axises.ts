import { invoke } from "@tauri-apps/api/tauri";

export interface AxisConifg {
  axis_name: String;
  speed: number;
  index: number;
  max_work_speed: number;
  init_position: number;
}

export interface AxisInfo {
  axis_name: String;
  current: number;
  io_status: AxisIoStatus;
}

interface AxisIoStatus {
  org: boolean;
  cw: boolean;
  ccw: boolean;
  setup: boolean;
  alm: boolean;
  pulse: boolean;
  coin: boolean;
}
interface Response<T> {
  message: string;
  code: number;
  data: T;
}

export async function GetAxisData(): Promise<Response<AxisInfo[]>> {
  let command_name = "get_all_axis_data";
  var result = try_invoke<Response<AxisInfo[]>>(command_name);
  return result;
}

function try_invoke<T>(command_name: string): Promise<T> {
  return new Promise(async (resolve, reject) => {
    try {
      var r: T = await invoke(command_name);
      resolve(r);
    } catch (error: any) {
      if (error.code) {
        alert(error.message); // todo: 换成更友好UI
        resolve(error);
      } else {
        reject(error);
      }
    }
  });
}

export async function GetAxisDataByName(
  axis_name: string
): Promise<Response<AxisInfo>> {
  return invoke("get_axis_data");
}

export async function GetAxisConfigs(): Promise<Response<AxisConifg[]>> {
  return try_invoke<Response<AxisConifg[]>>("get_axis_configs");
}

export async function InitAxisConfig(): Promise<Response<AxisConifg[]>> {
  return try_invoke<Response<AxisConifg[]>>("init_axis_config");
}
