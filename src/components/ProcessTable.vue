<script setup lang="ts">
import type { Process } from "@/model/Process";

const props = defineProps<{
  processes: Array<Process>;
  theme: String;
}>();

type ColumnViewOptions = {
  name: string;
  formatFn: (val: string) => string;
  visible: boolean;
  index: number;
  style: any;
  field: string;
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
    name: "Cmd",
    formatFn: (val: string) => {
      if (val != null) {
        return val.length > 100 ? val.substring(0, 97) + "..." : val;
      } else {
        return val;
      }
    },
    visible: true,
    style: {},
    field: "cmd",
  },
  {
    name: "Process Name",
    formatFn: null,
    visible: true,
    style: {},
    field: "process_name",
  },
  {
    name: "PSS",
    formatFn: null,
    visible: true,
    style: {},
    field: "pss_in_kb",
  },
  {
    name: "RSS",
    formatFn: null,
    visible: true,
    style: {},
    field: "rss_in_kb",
  },
  {
    name: "USS",
    formatFn: null,
    visible: true,
    style: {},
    field: "uss_in_kb",
  },
  {
    name: "Userspace ms",
    formatFn: null,
    visible: true,
    style: {},
    field: "time_userspace_miliseconds",
  },
  {
    name: "Kernel ms",
    formatFn: null,
    visible: true,
    style: {},
    field: "time_kernel_miliseconds",
  },
  {
    name: "Start Time",
    formatFn: null,
    visible: true,
    style: {},
    field: "start_time",
  },
  {
    name: "CPU%",
    formatFn: null,
    visible: true,
    style: {},
    field: "cpu_usage_percentage",
  },
  {
    name: "File Descriptors%",
    formatFn: null,
    visible: true,
    style: {},
    field: "fds",
  },
] as ColumnViewOptions[];

columnViewOptions.forEach((c, i) => {
  c.index = i;
});

const visibleColumnsCount = columnViewOptions.filter((x) => x.visible).length;
</script>

<template>
  <!-- Log table -->
  <div class="container-fluid" ref="scrollComponent">
    <table class="table table-striped table-hover table-borderless table-sm"
      :class="theme == 'dark' ? 'table-dark' : ''">
      <thead>
        <th v-for="c in columnViewOptions.filter((x) => x.visible)" :style="c.style">
          {{ c.name }}
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
</style>
