<script setup lang="ts">
import { NCard, NGrid, NGi } from "naive-ui";
import { Doughnut, Line } from "vue-chartjs";
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
    ArcElement,
    CategoryScale,
    LinearScale,
    LogarithmicScale,
    TimeScale,
    ChartOptions,
    TooltipItem,
} from "chart.js";
import "chartjs-adapter-date-fns";
import wasm_init, {
    wasm_pool_data,
    wasm_invite_data,
    wasm_plan_data,
    wasm_plan_size_data,
    wasm_plan_pie_data
} from "analyzer";

ChartJS.register(
    Title,
    Tooltip,
    Legend,
    BarElement,
    LineElement,
    PointElement,
    ArcElement,
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

/*** ====== Misc ====== */
let currentYear = new Date().getFullYear()

/*** ====== Chart Data Definition ====== ***/
let sizeChartData = wasm_plan_size_data(poolData, inviteData, planData);
let pieChartData = wasm_plan_pie_data(poolData, inviteData, planData, currentYear)

/*** ====== Chart Config Definition ====== ***/
const callback_tooltip_title_sizeChart = function (
    items: TooltipItem<"line">[]
) {
    return items.map((x) => sizeChartData.tooltip.title[0][x.dataIndex]);
};
const callback_tooltip_label_sizeChart = function (item: TooltipItem<"line">) {
    return sizeChartData.tooltip.label[item.datasetIndex][item.dataIndex];
};
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
    plugins: {
        legend: {
            position: "right",
            labels: {
                filter: function (item) {
                    return item.text != "none";
                },
            },
        },
        tooltip: {
            callbacks: {
                title: callback_tooltip_title_sizeChart,
                label: callback_tooltip_label_sizeChart,
            },
        },
    },
} as ChartOptions<"line">;

const callback_tooltip_label_pieChart = function (item: TooltipItem<"doughnut">) {
    return pieChartData.tooltip.label[item.datasetIndex][item.dataIndex];
};
let pieChartConfig = {
    maintainAspectRatio: false,
    plugins: {
        legend: {
            position: "right",
        },
        tooltip: {
            callbacks: {
                label: callback_tooltip_label_pieChart,
            },
        },
    },
} as ChartOptions<"doughnut">;

// ignore PNP because it is in a separate plan

</script>

<template>
    <n-grid cols="3">
        <n-gi span="2">
            <n-card title="IRCC Yearly Departmental Plan">
                <Line
                    ref="planChart"
                    :options="sizeChartConfig"
                    :data="sizeChartData"
                    :style="{
                        height: '30vh',
                        width: '100%',
                    }"
                />
            </n-card>
        </n-gi>
        <n-gi>
            <n-card :title="currentYear + ' Express Entry Quota'">
                <Doughnut
                    ref="pieChart"
                    :options="pieChartConfig"
                    :data="pieChartData"
                    :style="{
                        height: '30vh',
                        width: '100%',
                    }"
                />
            </n-card>
        </n-gi>
    </n-grid>
</template>

<style scoped>
.chartAreaWrapper {
    width: 600px;
    overflow-x: scroll;
}
</style>
