<template>
  <div class="axis-control">
    <table>
      <thead>
        <th>Axis Name</th>
        <th>Speed(mm/s)</th>
        <th>Index(mm)</th>
        <th>Current(mm)</th>
        <th>ORG</th>
        <th>CW</th>
        <th>CCW</th>
        <th>SETUP</th>
        <th>ALM</th>
        <th>PULSE</th>
        <th>COIN</th>
      </thead>
      <tbody>
        <tr v-for="[key, axis] in axises">
          <td>{{ key }}</td>
          <td><input type="number" v-model="axis.axis_config.speed" /></td>
          <td><input type="number" v-model="axis.axis_config.index" /></td>
          <td>{{ axis.axis_info?.current }}</td>
          <td>{{ axis.axis_info?.io_status.org }}</td>
          <td>{{ axis.axis_info?.io_status.cw }}</td>
          <td>{{ axis.axis_info?.io_status.ccw }}</td>
          <td>{{ axis.axis_info?.io_status.setup }}</td>
          <td>{{ axis.axis_info?.io_status.alm }}</td>
          <td>{{ axis.axis_info?.io_status.pulse }}</td>
          <td>{{ axis.axis_info?.io_status.coin }}</td>
        </tr>
      </tbody>
    </table>
    <div class="ctr-btn">
      <button @mousedown="SpeedMove(true)" @mouseup="Stop">Move +</button>
      <button @mousedown="SpeedMove(false)" @mouseup="Stop">Move -</button>
      <button @mousedown="IndexMove(true)" @mouseup="StopStepping">Index +</button>
      <button @mousedown="IndexMove(false)" @mouseup="StopStepping">Index -</button>
      <button @click="RefreshAxisData">Refresh</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
// import {GetAxisData, AxisData} from '@/apis/axises'
import { GetAxisData, StopAxis, InitAxisConfig, AxisInfo, GetAxisConfigs, AxisConifg, AbsMove, WaitAxises } from "../apis/axises";
interface DisplayData {
  axis_info: AxisInfo | null,
  axis_config: AxisConifg

}
let selected_axis_name = ref(new String())
let axises = ref(new Map<String, DisplayData>);
let handle: any;
let index_moving = false;
onMounted(async () => {
  await InitAxisConfig();
  var res = await GetAxisConfigs();
  selected_axis_name.value = res.data[0].axis_name;

  res.data.forEach(element => {
    axises.value.set(element.axis_name, {
      axis_config: element,
      axis_info: null
    })
  });
  handle = setInterval(async () => {
    await RefreshAxisData();
  }, 1300);
});
onUnmounted(() => {
  clearInterval(handle)
})
async function RefreshAxisData() {
  var res = await GetAxisData();
  res.data.forEach(element => {
    let val = axises.value.get(element.axis_name)
    if (val) {
      val.axis_info = element
      axises.value.set(element.axis_name, val)
    }
  });
}

async function SpeedMove(pluse: Boolean) {
  let axis_config = axises.value.get(selected_axis_name.value)?.axis_config
  if (axis_config) {
    await AbsMove([{
      axis_name: axis_config.axis_name,
      speed: axis_config.speed,
      destination: pluse ? 500 : 0 // ToDo: ?????????????????????????????????
    }])
  }
}

async function IndexMove(pluse: Boolean) {
  index_moving = true
  let i = 0
  while (index_moving) {
    console.log("?????????");
    await StepMove();
    i += 1
    console.log("?????????", i);

  }

  async function StepMove() {
    let axis_config = axises.value.get(selected_axis_name.value)?.axis_config;
    let axis_info = axises.value.get(selected_axis_name.value)?.axis_info;

    if (axis_config && axis_info) {
      let destination = axis_info.current + (pluse ? 1 : -1) * axis_config.index
      await AbsMove([{
        axis_name: axis_config.axis_name,
        speed: axis_config.speed,
        destination: destination
      }]);
      await b();
      async function b() {
        console.log("??????????????????????????????");
        
        let r = await WaitAxises([selected_axis_name.value])
        if (!r.data)
          window.setTimeout(b, 1000)
      }
    }
  }
}

function StopStepping() {
  index_moving = false
}

async function Stop() {
  let axis_config = axises.value.get(selected_axis_name.value)?.axis_config
  if (axis_config)
    await StopAxis(axis_config.axis_name)
}
</script>

<style scoped>
</style>
