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
        <template v-for="axis in axises">
          <td>{{ axis.axis_name }}</td>
          <td><input type="number" v-model="axis.speed" /></td>
          <td><input type="number" v-model="axis.index" /></td>
          <td>{{ axis.current }}</td>
          <td>{{ axis.org }}</td>
          <td>{{ axis.cw }}</td>
          <td>{{ axis.ccw }}</td>
          <td>{{ axis.setup }}</td>
          <td>{{ axis.alm }}</td>
          <td>{{ axis.pulse }}</td>
          <td>{{ axis.coin }}</td>
        </template>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, Ref } from "vue";
// import {GetAxisData, AxisData} from '@/apis/axises'
import { GetAxisData, AxisData } from "../apis/axises";

let axises: Ref<AxisData[]> = ref([]);
onMounted(async () => {
  var res = await GetAxisData();
  axises.value = res.data;
});
</script>

<style scoped></style>
