<script setup lang="ts">
import { reactive, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import ProcessTable from "@/components/ProcessTable.vue";
import type { Process } from "@/model/Process";
import type { ProcessQuery } from "@/model/ProcessQuery";

const props = defineProps<{
  theme: String;
}>();

let procRefresh: number;
let lastQuery: ProcessQuery = {
  sortBy: "cpu_usage_percentage",
  sortOrder: "desc"
};
const REFRESH_FREQ = 3000;

let vm = reactive({
  processes: [] as Process[],
});

function getSystemStatus(query: ProcessQuery, event?: Event) {
  if (event != null) {
    event.preventDefault();
  }

  invoke("get_system_status")
    .then((response) => {
      console.log("SystemStatus");
      console.log(JSON.stringify(response));
    })
    .catch((err) => {
      console.error("SystemStatus error: " + err);
    });

  invoke("get_processes", { query: query })
    .then((response: any) => {
      vm.processes = response;
    })
    .catch((err) => {
      console.error("Processes error: " + err);
    });
}

onMounted(() => {
  getSystemStatus(lastQuery);
  procRefresh = setInterval(() => getSystemStatus(lastQuery), REFRESH_FREQ);
});

onUnmounted(() => {
  clearInterval(procRefresh);
});

function sort(query: ProcessQuery) {
  clearInterval(procRefresh);
  lastQuery = query;
  getSystemStatus(lastQuery);
  procRefresh = setInterval(() => getSystemStatus(lastQuery), REFRESH_FREQ);
}
</script>

<template>
  <h2>System Monitor</h2>
  <ProcessTable :processes="vm.processes" :theme="theme" @sorted="sort"></ProcessTable>
</template>

<style scoped>
main.dark h2 {
  color: #eee;
}
</style>
