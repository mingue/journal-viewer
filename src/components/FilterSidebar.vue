<script setup lang="ts">
import type { Filter } from "@/model/Filter";
import { reactive } from "vue";

let vm = reactive({
  isSidebarCollapsed: true,
  priority: "6",
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

function filter(event: Event) {
  if (event != null) {
    event.preventDefault();
  }

  emit("filter", { priority: vm.priority });
  toggleSidebar(event);
}
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
      <button type="submit" class="btn btn-outline-primary" @click="filter">Filter</button>
    </form>
  </div>
</template>

<style scoped>
.filter-content {
  width: 600px;
  padding: 1rem;
}
</style>
