import { invoke } from "@tauri-apps/api/tauri";

export interface AxisData {
  axis_name: string;
  speed: number;
  index: number;
  current: number;
  io_status:AxisIoStatus;
}

interface AxisIoStatus{
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
export async function GetAxisData(): Promise<Response<AxisData[]>> {
  return invoke("get_all_axis_data");
}

export async function GetAxisDataByName(axis_name:string): Promise<Response<AxisData>> {
  return invoke("get_axis_data");
}

