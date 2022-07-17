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
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, Ref } from "vue";
// import {GetAxisData, AxisData} from '@/apis/axises'
import { GetAxisData, InitAxisConfig, AxisInfo, GetAxisConfigs, AxisConifg } from "../apis/axises";
interface DisplayData {
  axis_info: AxisInfo | null,
  axis_config: AxisConifg

}
let axises = ref(new Map<String, DisplayData>);
onMounted(async () => {
  await InitAxisConfig();
  var res = await GetAxisConfigs();
  res.data.forEach(element => {
    axises.value.set(element.axis_name, {
      axis_config: element,
      axis_info: null
    })
  });
  await RefreshAxisData();
});



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
</script>

<style scoped>
</style>
