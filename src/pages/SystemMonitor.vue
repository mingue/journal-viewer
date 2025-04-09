<script setup lang="ts">
import { reactive, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import ProcessTable from "@/components/ProcessTable.vue";
import type { Process } from "@/model/Process";

const props = defineProps<{
  theme: String;
}>();

let procRefresh: number;

let vm = reactive({
  processes: [] as Process[],
});

function getSystemStatus(event?: Event) {
  if (event != null) {
    event.preventDefault();
  }

  invoke("get_system_status")
    .then((response) => {
      console.log("SystemStatus");
      console.log(JSON.stringify(response));
    })
    .catch(() => {
      console.error("SystemStatus error");
    });

  invoke("get_processes")
    .then((response: any) => {
      vm.processes = response;
    })
    .catch(() => {
      console.error("Processes error");
    });
}

onMounted(() => {
  getSystemStatus();
  procRefresh = setInterval(getSystemStatus, 5000);
});

onUnmounted(() => {
  clearInterval(procRefresh);
});
</script>

<template>
  <h2>System Monitor</h2>
  <ProcessTable :processes="vm.processes" :theme="theme"></ProcessTable>
</template>

<style scoped>
main.dark h2 {
  color: #eee;
}
</style>
