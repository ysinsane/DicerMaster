import { invoke } from "@tauri-apps/api/tauri";

export interface AxisData {
  axis_name: string;
  speed: number;
  index: number;
  current: number;
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
  return invoke("get_axis_data");
}
