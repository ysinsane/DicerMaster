import { invoke, InvokeArgs } from "@tauri-apps/api/tauri";

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
interface Response<T = null> {
  message: string;
  code: number;
  data: T;
}

export interface MoveParam {
  axis_name: String;
  speed: number | null;
  destination: number | null;
}

export async function GetAxisData(): Promise<Response<AxisInfo[]>> {
  let command_name = "get_all_axis_data";
  var result = try_invoke<Response<AxisInfo[]>>(command_name);
  return result;
}

function try_invoke<T>(
  command_name: string,
  args?: InvokeArgs | undefined
): Promise<T> {
  return new Promise(async (resolve, reject) => {
    try {
      var r: T = await invoke(command_name, args);
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

export async function InitAxisConfig(): Promise<Response> {
  return try_invoke<Response>("init_axis_config");
}
export async function AbsMove(moveParams: MoveParam[]): Promise<Response> {
  return try_invoke<Response>("abs_move", { moveParams });
}
export async function WaitAxises(axisNames: String[]): Promise<Response> {
  return try_invoke<Response>("wait_axises", { axisNames });
}
export async function StopAxis(axisName: String): Promise<Response> {
  return try_invoke<Response>("stop_axis", { axisName });
}
