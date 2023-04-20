<script setup lang="ts">
import { reactive, onMounted } from "vue";
import { invoke } from "@tauri-apps/api";
import type { JournalEntries } from "./model/JournalEntries";
import SummaryBar from "./components/SummaryBar.vue";
import LogTable from "./components/LogTable.vue";
import SearchBar from "./components/SearchBar.vue";
import FilterSidebar from "./components/FilterSidebar.vue";
import type { Filter } from "./model/Filter";

let vm = reactive({
  logs: {} as JournalEntries,
  isSidebarCollapsed: true,
  priority: "5",
  services: [] as string[],
  quickSearch: "",
  theme: "",
  transports: ["syslog", "journal", "stdout"],
  datetimeTo: "",
  datetimeFrom: "",
});

let journalQuery = {
  fields: ["PRIORITY", "__REALTIME", "_COMM", "MESSAGE", "_TRANSPORT"],
  priority: parseInt(vm.priority),
  services: [] as string[],
  quickSearch: vm.quickSearch,
  limit: 50,
  resetPosition: true,
  transports: [] as string[],
  datetimeTo: "",
  datetimeFrom: "",
};

let loadingLogs = false;

function getLogs(event?: Event) {
  if (event != null) {
    event.preventDefault();
  }

  vm.isSidebarCollapsed = true;

  journalQuery.priority = parseInt(vm.priority);
  journalQuery.quickSearch = vm.quickSearch;
  journalQuery.resetPosition = true;
  journalQuery.services = vm.services;
  journalQuery.transports = vm.transports;
  journalQuery.datetimeFrom = vm.datetimeFrom;
  journalQuery.datetimeTo = vm.datetimeTo;

  loadingLogs = true;

  invoke<JournalEntries>("get_logs", {
    query: journalQuery,
  })
    .then((response) => {
      loadingLogs = false;
      vm.logs = response;
    })
    .catch(() => {
      loadingLogs = false;
    });
}

function loadNextLogs() {
  if (loadingLogs) {
    return;
  }

  loadingLogs = true;

  journalQuery.resetPosition = false;

  invoke<JournalEntries>("get_logs", {
    query: journalQuery,
  })
    .then((response) => {
      vm.logs = {
        ...response,
        rows: vm.logs.rows.concat(response.rows),
      };
      loadingLogs = false;
    })
    .catch(() => {
      loadingLogs = false;
    });
}

function quickSearch(search: string) {
  vm.quickSearch = search;
  getLogs();
}

function filter(filter: Filter) {
  vm.priority = filter.priority;
  vm.services = filter.services;
  vm.transports = filter.transports;
  vm.datetimeTo = filter.datetimeTo;
  vm.datetimeFrom = filter.datetimeFrom;
  getLogs();
}

onMounted(() => {
  const match = window.matchMedia("(prefers-color-scheme: dark)");

  if (match) {
    vm.theme = "dark";
    document.getElementsByTagName("html")[0].style = "background-color: #222;";
  }

  getLogs();
});
</script>

<template>
  <header></header>
  <main :class="vm.theme">
    <SummaryBar />
    <SearchBar @quick-search="quickSearch" />
    <!-- Main Content -->
    <div class="d-flex">
      <FilterSidebar :theme="vm.theme" :priority="vm.priority" :transports="vm.transports" @filter="filter" />
      <div class="flex-fill">
        <LogTable :logs="vm.logs" @load-more="loadNextLogs" />
      </div>
    </div>
  </main>
</template>

<style scoped>
main.dark {
  background-color: #222;
}
</style>
