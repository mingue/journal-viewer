<script setup lang="ts">
import type { Filter } from "@/model/Filter";
import type { Unit } from "@/model/Unit";
import { invoke } from "@tauri-apps/api";
import { onMounted, reactive } from "vue";
import Multiselect from "@vueform/multiselect";
import VueDatePicker from "@vuepic/vue-datepicker";
import type { Boot } from "@/model/Boot";
import { formatEpoch } from "@/common/DateFormatter";
import "@vuepic/vue-datepicker/dist/main.css";
import "@vueform/multiselect/themes/default.css";

const props = defineProps<{
  transports: string[];
  priority: string;
  theme: string;
}>();

type SelectOption<T> = {
  value: T;
  label: string;
};

let vm = reactive({
  isSidebarCollapsed: true,
  isRefreshing: false,
  priority: props.priority,
  services: [] as Unit[],
  servicesOptions: [] as SelectOption<Unit>[],
  transports: props.transports,
  transportOptions: [] as SelectOption<string>[],
  notMoreRecentThan: null,
  notOlderThan: null,
  datetimeFrom: "",
  datetimeTo: "",
  boots: [] as Boot[],
  bootsOptions: [] as SelectOption<Boot>[],
});

const emit = defineEmits<{
  (e: "filter", filter: Filter): void;
}>();

function toggleSidebar(event: Event) {
  event.preventDefault();

  if (vm.isSidebarCollapsed) {
    vm.isSidebarCollapsed = false;
  } else {
    vm.isSidebarCollapsed = true;
  }
}

function getServices() {
  invoke<Array<Unit>>("get_services")
    .then((response) => {
      vm.servicesOptions = response.map((x) => ({
        value: x,
        label: x.unit_file.replace(".service", ""),
      }));
    })
    .catch((err) => {
      console.error(err);
    });
}

function getBoots() {
  invoke<Array<Boot>>("get_boots")
    .then((response) => {
      vm.bootsOptions = response.map((x) => ({
        value: x,
        label: `${x.index.toString()} - from: ${formatEpoch((parseInt(x.first_entry.toString()) / 1000).toString())}, to: ${formatEpoch((parseInt(x.last_entry.toString()) / 1000).toString())}`,
      }));
    })
    .catch((err) => {
      console.error(err);
    });
}

function refresh(event: Event) {
  vm.isRefreshing = true;
  filterInternal(event);
  setTimeout(() => {
    vm.isRefreshing = false;
  }, 200);
}

function filter(event: Event) {
  filterInternal(event);
  toggleSidebar(event);
}

function filterInternal(event: Event) {
  if (event != null) {
    event.preventDefault();
  }

  if (vm.datetimeFrom == null) {
    vm.datetimeFrom = "";
  }

  if (vm.datetimeTo == null) {
    vm.datetimeTo = "";
  }

  emit("filter", {
    priority: vm.priority,
    services: vm.services.map((x) => x.unit_file),
    transports: vm.transports,
    datetimeFrom: vm.datetimeFrom,
    datetimeTo: vm.datetimeTo,
    bootIds: vm.boots.map((x) => x.boot_id),
  });
}

onMounted(() => {
  getServices();
  getBoots();
  vm.transportOptions = [
    { value: "audit", label: "Audit" },
    { value: "driver", label: "Driver" },
    { value: "syslog", label: "Syslog" },
    { value: "journal", label: "Journal" },
    { value: "stdout", label: "Stdout" },
    { value: "kernel", label: "Kernel" },
  ];
});
</script>

<template>
  <div class="flex">
    <!-- Filter menu -->
    <div class="d-flex flex-column flex-shrink-0">
      <ul class="nav nav-pills nav-flush flex-column mb-auto text-center">
        <li class="nav-item" @click="toggleSidebar" title="Toggle Filter">
          <a href="#" class="nav-link py-3 rounded-0" :class="{ active: !vm.isSidebarCollapsed }" aria-current="page"
            data-bs-toggle="tooltip" data-bs-placement="right" aria-label="Home" data-bs-original-title="Home">
            <i class="bi bi-funnel"></i>
          </a>
        </li>
        <li class="nav-item" @click="refresh" title="Refresh">
          <a href="#" class="nav-link py-3 rounded-0" :class="{ active: vm.isRefreshing }" aria-current="page"
            data-bs-toggle="tooltip" data-bs-placement="right" aria-label="Home" data-bs-original-title="Home">
            <i class="bi bi-arrow-clockwise"></i>
          </a>
        </li>
      </ul>
    </div>
  </div>
  <div class="flex filter-content" :class="{ 'd-none': vm.isSidebarCollapsed }">
    <!-- Filter content -->
    <form>
      <div class="mb-3">
        <label for="priority" class="form-label">Priority</label>
        <select id="priority" v-model="vm.priority" class="form-select" aria-describedby="priorityHelp">
          <option value="0">0 - Emergency</option>
          <option value="1">1 - Alert</option>
          <option value="2">2 - Critical</option>
          <option value="3">3 - Error</option>
          <option value="4">4 - Warning</option>
          <option value="5">5 - Notice</option>
          <option value="6">6 - Informational</option>
          <option value="7">7 - Debug</option>
        </select>
        <div class="form-text">Higher or equal to</div>
      </div>
      <div class="mb-3">
        <label for="recent" class="form-label">From timestamp</label>
        <VueDatePicker id="recent" v-model="vm.datetimeFrom" text-input v-bind:dark="props.theme == 'dark'" />
        <div class="form-text">Exclude results more recent than the date indicated</div>
      </div>
      <div class="mb-3">
        <label for="oldest" class="form-label">Until timestamp</label>
        <VueDatePicker id="oldest" v-model="vm.datetimeTo" text-input v-bind:dark="props.theme == 'dark'" />
        <div class="form-text">Exclude results older than the date indicated</div>
      </div>
      <div class="mb-3">
        <label for="service" class="form-label">Services</label>
        <Multiselect v-model="vm.services" :options="vm.servicesOptions" mode="tags" :close-on-select="false"
          :searchable="true" />
        <div class="form-text">View logs only for the services selected</div>
      </div>
      <div class="mb-3">
        <label for="transport" class="form-label">Transport</label>
        <Multiselect v-model="vm.transports" :options="vm.transportOptions" mode="tags" :close-on-select="false"
          :searchable="true" />
        <div class="form-text">Select Transports</div>
      </div>
      <div class="mb-3">
        <label for="boot" class="form-label">Boots</label>
        <Multiselect v-model="vm.boots" :options="vm.bootsOptions" mode="tags" :close-on-select="false"
          :searchable="true" />
        <div class="form-text">View logs only for the boots selected</div>
      </div>

      <button type="submit" class="btn btn-outline-primary" @click="filter">Filter</button>
    </form>
  </div>
</template>

<style scoped>
.filter-content {
  width: 600px;
  padding: 1rem;
}

main.dark .filter-content {
  color: #ddd;
  border-right: 1px solid white;
  border-left: 1px solid white;
}

main.dark .filter-content .form-select {
  background-color: #444;
  color: #ddd;
}

main.dark .filter-content .form-control {
  background-color: #444;
  color: #ddd;
}

main.dark .filter-content .btn {
  color: #ddd;
}
</style>
<style>
main.dark .multiselect {
  background-color: #444;
}

main.dark .multiselect-tags-search {
  background-color: #444;
  color: white;
}

main.dark .multiselect-dropdown {
  background-color: #444;
}

main.dark .multiselect-option.is-pointed {
  background-color: #999;
}

main.dark .multiselect-tag {
  background-color: #2a589c;
}

main.dark .dp__input {
  background-color: #444;
  border-color: 1px solid white;
}

main.dark .dp__action.dp__select {
  color: #2a589c;
}
</style>
