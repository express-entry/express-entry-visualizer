<script setup lang="ts">
import { ref, Ref } from "vue";
import { NCard, NGrid, NGridItem } from "naive-ui";
import { Line } from "vue-chartjs";
import zoomPlugin from "chartjs-plugin-zoom";
import { FocusScale } from "../composables/FocusScale";
import {
    Chart as ChartJS,
    Title,
    Tooltip,
    Legend,
    BarElement,
    LineElement,
    PointElement,
    CategoryScale,
    LinearScale,
    LogarithmicScale,
    TimeScale,
    ChartOptions,
} from "chart.js";
import "chartjs-adapter-date-fns";
import wasm_init, {
    wasm_pool_data,
    wasm_invite_data,
    wasm_plan_data,
    wasm_plan_size_data,
} from "analyzer";

ChartJS.register(
    Title,
    Tooltip,
    Legend,
    BarElement,
    LineElement,
    PointElement,
    CategoryScale,
    LinearScale,
    LogarithmicScale,
    TimeScale,
    FocusScale,
    zoomPlugin
);

await wasm_init();
let poolData = await wasm_pool_data();
let inviteData = await wasm_invite_data();
let planData = await wasm_plan_data();

/*** ====== Chart Data Definition ====== ***/
let sizeChartData = wasm_plan_size_data(poolData, inviteData, planData);

/*** ====== Chart Config Definition ====== ***/
let sizeChartConfig = {
    maintainAspectRatio: false,
    scales: {
        x: {
            type: "time",
        },
        y: {
            type: "linear",
        },
    },
    plugins: {},
} as ChartOptions<"line">;

// ignore PNP because it is in a separate plan

let planChart: Ref<typeof Line> = ref();
let controllingChart = null;
</script>

<template>
    <n-grid cols="3">
        <n-grid-item span="2">
            <n-card title="IRCC Departmental Plan">
                <Line
                    ref="planChart"
                    @mouseover="controllingChart = planChart"
                    @mouseleave="controllingChart = null"
                    :options="sizeChartConfig"
                    :data="sizeChartData"
                    :style="{
                        height: '30vh',
                        width: '100%',
                    }"
                />
            </n-card>
        </n-grid-item>
    </n-grid>
</template>

<style scoped>
.chartAreaWrapper {
    width: 600px;
    overflow-x: scroll;
}
</style>
