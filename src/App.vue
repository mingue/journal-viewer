<script setup lang="ts">
import { reactive, onMounted } from "vue";
import SystemMonitor from "./pages/SystemMonitor.vue";
import LogViewer from "./pages/LogViewer.vue";
import { invoke } from "@tauri-apps/api/core";

let vm = reactive({
  theme: "",
  activeTab: "logViewer",
  isDarkThemeOn: true,
  systemMonitorEnabled: false,
});

function toggleTheme() {
  if (vm.theme == "dark") {
    document.getElementsByTagName("html")[0].style = "height: 100%; background-color: #fff;";
    vm.theme = "";
  } else {
    document.getElementsByTagName("html")[0].style = "height: 100%; background-color: #222;";
    vm.theme = "dark";
  }

  vm.isDarkThemeOn = !vm.isDarkThemeOn;
}

function switchTab(tab: string) {
  vm.activeTab = tab;
}

onMounted(() => {
  const match = window.matchMedia("(prefers-color-scheme: dark)");
  document.getElementsByTagName("html")[0].style = "height: 100%";
  if (match) {
    vm.theme = "dark";
    vm.isDarkThemeOn = true;
    document.getElementsByTagName("html")[0].style = "height: 100%; background-color: #222;";
  }
});

invoke("get_config")
  .then((response: any) => {
    vm.systemMonitorEnabled = response.systemMonitorEnabled;
  })
  .catch((err) => {
    console.error("GetConfig error: " + err);
  });
</script>

<template>
  <header></header>
  <main :class="vm.theme">
    <div class="clearfix">
      <ul class="nav nav-underline float-start">
        <li class="nav-item">
          <a class="nav-link" :class="vm.activeTab == 'logViewer' ? 'active' : ''" aria-current="page" href="#"
            @click="switchTab('logViewer')">Log Viewer</a>
        </li>
        <li class="nav-item" v-if="vm.systemMonitorEnabled">
          <a class="nav-link" :class="vm.activeTab == 'systemMonitor' ? 'active' : ''" href="#"
            @click="switchTab('systemMonitor')">System Monitor</a>
        </li>
      </ul>
      <div class="float-end d-inline-block theme-toggle" @click="toggleTheme">
        <i class="bi bi-lightbulb d-inline-block" title="Toggle theme" v-if="vm.isDarkThemeOn"></i>
        <i class="bi bi-lightbulb-fill d-inline-block" title="Toggle theme" v-if="!vm.isDarkThemeOn"></i>
      </div>
    </div>
    <div class="content">
      <div class="content-tab" v-if="vm.activeTab == 'logViewer'">
        <LogViewer :theme="vm.theme"></LogViewer>
      </div>
      <div class="content-tab" v-if="vm.activeTab == 'systemMonitor'">
        <SystemMonitor :theme="vm.theme"></SystemMonitor>
      </div>
    </div>
  </main>
</template>

<style scoped>
main.dark {
  background-color: #222;
}

.nav {
  padding-top: 4px;
  padding-left: 16px;
  padding-right: 10px;
}

.nav-item {
  padding-right: 10px;
}

main .nav-item a {
  color: #444;
}

main .nav-item a.active {
  color: #222;
}

main.dark .nav-item a {
  color: #ddd;
}

main.dark .nav-item a.active {
  color: #eee;
}

.content {
  padding-left: 16px;
  padding-right: 20px;
  padding-bottom: 20px;
}

.content-tab {
  padding-top: 20px;
}

main .bi {
  padding-top: 4px;
}

main.dark .bi {
  color: #eee;
}

.theme-toggle {
  padding-top: 6px;
  padding-right: 20px;
  cursor: pointer;
}
</style>
