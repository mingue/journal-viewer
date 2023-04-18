<script setup lang="ts">
import type { Filter } from "@/model/Filter";
import type { Unit } from "@/model/Unit";
import { invoke } from "@tauri-apps/api";
import { onMounted, reactive } from "vue";
import Multiselect from "@vueform/multiselect";

type ServiceOptions = {
  value: Unit;
  label: string;
};

let vm = reactive({
  isSidebarCollapsed: true,
  priority: "6",
  services: [] as Unit[],
  servicesOptions: [] as ServiceOptions[],
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

function filter(event: Event) {
  if (event != null) {
    event.preventDefault();
  }

  emit("filter", {
    priority: vm.priority,
    services: vm.services.map((x) => x.unit_file),
  });
  toggleSidebar(event);
}

onMounted(() => {
  getServices();
});
</script>

<template>
  <div class="flex">
    <!-- Filter menu -->
    <div class="d-flex flex-column flex-shrink-0" @click="toggleSidebar">
      <ul class="nav nav-pills nav-flush flex-column mb-auto text-center">
        <li class="nav-item">
          <a
            href="#"
            class="nav-link py-3 border-bottom rounded-0"
            :class="{ active: !vm.isSidebarCollapsed }"
            aria-current="page"
            data-bs-toggle="tooltip"
            data-bs-placement="right"
            aria-label="Home"
            data-bs-original-title="Home"
          >
            <i class="bi bi-funnel"></i>
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
        <div id="priorityHelp" class="form-text">Higher or equal to</div>
      </div>
      <div class="mb-3">
        <label for="service" class="form-label">Service</label>
        <Multiselect
          v-model="vm.services"
          :options="vm.servicesOptions"
          mode="tags"
          :close-on-select="false"
          :searchable="true"
        />
        <div id="priorityHelp" class="form-text">View logs only for the services selected</div>
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
}

main.dark .filter-content .form-select {
  background-color: #444;
  color: #ddd;
}

main.dark .filter-content .btn {
  color: #ddd;
}
</style>
<style src="@vueform/multiselect/themes/default.css"></style>
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
</style>
