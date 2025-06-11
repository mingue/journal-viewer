<script setup lang="ts">
import type { Process } from "@/model/Process";
import type { ProcessQuery } from "@/model/ProcessQuery";
import { ref, computed } from "vue";

const props = defineProps<{
  processes: Array<Process>;
  theme: String;
}>();

const emit = defineEmits<{
  (e: "sorted", q: ProcessQuery): void;
}>();

type ColumnViewOptions = {
  name: string;
  formatFn: (val: string | number) => string;
  visible: boolean;
  index: number;
  style: any;
  field: string;
  info: string | undefined;
};

const formatSize = (val: number) => {
  if (val >= 1024 * 1024) {
    return `${(val / (1024 * 1024)).toFixed(2)} GB`;
  } else if (val >= 1024) {
    return `${(val / 1024).toFixed(2)} MB`;
  } else {
    return `${val} KB`;
  }
};
const formatTime = (val: number) => {
  if (val >= 86400000) {
    return `${(val / 86400000).toFixed(2)} d`;
  } else if (val >= 3600000) {
    return `${(val / 3600000).toFixed(2)} h`;
  } else if (val >= 60000) {
    return `${(val / 60000).toFixed(2)} min`;
  } else if (val >= 1000) {
    return `${(val / 1000).toFixed(2)} sec`;
  } else {
    return `${val} ms`;
  }
};

const columnViewOptions = [
  {
    name: "Pid",
    formatFn: null,
    visible: true,
    style: { width: "5rem", "text-align": "right", "padding-right": "5px" },
    field: "pid",
  },
  {
    name: "Process Name",
    formatFn: null,
    visible: true,
    style: {},
    field: "process_name",
  },
  {
    name: "CPU%",
    formatFn: (val: number) => {
      if (val != null) {
        return val.toFixed(2);
      } else {
        return val;
      }
    },
    visible: true,
    style: { width: "6rem", "text-align": "right", "padding-right": "5px" },
    field: "cpu_usage_percentage",
  },
  {
    name: "PSS",
    formatFn: formatSize,
    visible: true,
    style: { width: "8rem", "text-align": "right", "padding-right": "5px" },
    field: "pss_in_kb",
    info: "Proportional Set Size: dedicated process memory + proportional part of shared libraries"
  },
  {
    name: "RSS",
    formatFn: formatSize,
    visible: true,
    style: { width: "8rem", "text-align": "right", "padding-right": "5px" },
    field: "rss_in_kb",
    info: "Resident Set Size: dedicated process memory + shared libraries"
  },
  {
    name: "USS",
    formatFn: formatSize,
    visible: true,
    style: { width: "8rem", "text-align": "right", "padding-right": "5px" },
    field: "uss_in_kb",
    info: "Proportional Set Size: dedicated process memory, excluding shared libraries"
  },
  {
    name: "Userspace time",
    formatFn: formatTime,
    visible: true,
    style: { width: "10rem", "text-align": "right", "padding-right": "5px" },
    field: "time_userspace_miliseconds",
  },
  {
    name: "Kernel time",
    formatFn: formatTime,
    visible: true,
    style: { width: "8rem", "text-align": "right", "padding-right": "5px" },
    field: "time_kernel_miliseconds",
  },
  {
    name: "No. File Desc",
    formatFn: null,
    visible: true,
    style: { width: "10rem", "text-align": "right", "padding-right": "5px" },
    field: "fds",
  },
  {
    name: "Start Time",
    formatFn: null,
    visible: true,
    style: { width: "8rem", "text-align": "right", "padding-right": "5px" },
    field: "start_time",
  },
  {
    name: "Cmd",
    formatFn: (val: string) => {
      if (val != null) {
        return val.length > 50 ? val.substring(0, 47) + "..." : val;
      } else {
        return val;
      }
    },
    visible: true,
    style: {},
    field: "cmd",
  },
] as ColumnViewOptions[];

columnViewOptions.forEach((c, i) => {
  c.index = i;
});

const sortColumn = ref<string | null>(null);
const sortOrder = ref<"asc" | "desc">("asc");

function toggleSort(column: string) {
  if (sortColumn.value === column) {
    sortOrder.value = sortOrder.value === "asc" ? "desc" : "asc";
  } else {
    sortColumn.value = column;
    sortOrder.value = "asc";
  }

  emit("sorted", {
    sortBy: sortColumn.value.toString(),
    sortOrder: sortOrder.value.toString()
  })
}

</script>

<template>
  <!-- Log table -->
  <div class="container-fluid" ref="scrollComponent">
    <table class="table table-striped table-hover table-borderless table-sm"
      :class="theme == 'dark' ? 'table-dark' : ''">
      <thead>
        <th v-for="c in columnViewOptions.filter((x) => x.visible)" :style="c.style" @click="toggleSort(c.field)"
          style="cursor: pointer;" :title="c.info != null ? c.info : c.name">
          {{ c.name }}
          <span v-if="sortColumn === c.field">
            <i :class="sortOrder === 'asc' ? 'bi bi-arrow-up' : 'bi bi-arrow-down'"></i>
          </span>
        </th>
      </thead>
      <tbody class="table-group-divider">
        <template v-for="row in processes" :key="row.pid">
          <tr>
            <td v-for="c in columnViewOptions.filter((x) => x.visible)" :style="c.style">
              <div :title="row[c.field]">
                {{ c.formatFn != null ? c.formatFn(row[c.field]) : row[c.field] }}
              </div>
            </td>
          </tr>
        </template>
      </tbody>
    </table>
  </div>
</template>

<style scoped>
th {
  cursor: pointer;
}
</style>
