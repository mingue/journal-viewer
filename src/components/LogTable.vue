<script setup lang="ts">
import { onMounted, onUnmounted, reactive, ref } from "vue";
import type { JournalEntries, JournalEntry } from "@/model/JournalEntries";
import { invoke } from "@tauri-apps/api";
import { formatEpoch } from "@/common/DateFormatter";

const props = defineProps<{
  logs: JournalEntries;
  theme: String;
}>();

const emit = defineEmits<{
  (e: "load-more"): void;
}>();

let vm = reactive({
  expandedRowTimestamp: "",
  expandedEntry: null as JournalEntry | null,
});

const scrollComponent = ref<Element | null>(null);

type ColumnViewOptions = {
  name: string;
  formatFn: (val: string) => string;
  visible: boolean;
  index: number;
  style: any;
};

const columnViewOptions = [
  {
    name: "", // Priority
    formatFn: () => " ",
    visible: true,
    style: {},
  },
  {
    name: "Timestamp",
    formatFn: (d) => formatEpoch(parseInt(d) / 1000, true),
    visible: true,
    style: { width: "10rem" },
  },
  {
    name: "Process",
    formatFn: null,
    visible: true,
    style: {},
  },
  {
    name: "Message",
    formatFn: (val: string) => {
      if (val != null) {
        return val.length > 500 ? val.substring(0, 497) + "..." : val;
      } else {
        return val;
      }
    },
    visible: true,
    style: {},
  },
  {
    name: "Transport",
    formatFn: null,
    visible: true,
    style: {},
  },
] as ColumnViewOptions[];

columnViewOptions.forEach((c, i) => {
  c.index = i;
});

onMounted(() => {
  window.addEventListener("scroll", handleScroll);
});

onUnmounted(() => {
  window.removeEventListener("scroll", handleScroll);
});

const handleScroll = () => {
  if (scrollComponent.value != null && scrollComponent.value.getBoundingClientRect().bottom < window.innerHeight) {
    emit("load-more");
  }
};

const getRowClass = (row: Array<string>) => `priority-${row[0]}`;
const visibleColumnsCount = columnViewOptions.filter((x) => x.visible).length;

function toggleFullRecord(timestamp: string) {
  if (vm.expandedRowTimestamp == timestamp) {
    vm.expandedRowTimestamp = "";
    vm.expandedEntry = null;
    return;
  }

  invoke<JournalEntry>("get_full_entry", {
    timestamp: Number.parseInt(timestamp),
  })
    .then((response: any) => {
      vm.expandedEntry = response;
      vm.expandedRowTimestamp = timestamp;
      console.log(response);
    })
    .catch((e) => {
      console.error(e);
    });
}
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
        <template v-for="row in logs.rows">
          <tr :class="getRowClass(row)" @click="toggleFullRecord(row[1])" style="cursor: pointer;">
            <td v-for="c in columnViewOptions.filter((x) => x.visible)" :style="c.style">
              <div :title="row[c.index]">
                {{ c.formatFn != null ? c.formatFn(row[c.index]) : row[c.index] }}
              </div>
            </td>
          </tr>
          <tr v-if="vm.expandedRowTimestamp == row[1]">
            <td :colspan="visibleColumnsCount">
              <table class="full-entry">
                <tr v-for="(item, index) in vm.expandedEntry?.headers">
                  <th>{{ vm.expandedEntry?.headers[index] }}</th>
                  <td>{{ vm.expandedEntry?.values[index] }}</td>
                </tr>
              </table>
            </td>
          </tr>
        </template>
      </tbody>
    </table>
  </div>
</template>

<style scoped>
.priority-0 {
  font-weight: 500;
  background-color: rgb(39, 39, 39);
  color: white;
}

.priority-0 td div {
  color: white;
}

.priority-0 td:first-child div {
  width: 4px;
  height: 24px;
}

.priority-1 {
  font-weight: 500;
  background-color: rgb(131, 60, 60);
  color: white;
}

.priority-1 td div {
  color: white;
}

.priority-1 td:first-child div {
  width: 4px;
  height: 24px;
}

.priority-2 {
  font-weight: 500;
}

.priority-2 td:first-child div {
  width: 4px;
  height: 24px;
  background-color: red;
}

.priority-3 {}

.priority-3 td:first-child div {
  width: 4px;
  height: 24px;
  background-color: red;
}

.priority-4 {}

.priority-4 td:first-child div {
  width: 4px;
  height: 24px;
  background-color: rgb(255, 136, 0);
}

.priority-5 {}

.priority-5 td:first-child div {
  width: 4px;
  height: 24px;
  background-color: rgb(0, 204, 255);
}

.priority-6 {
  color: #666;
}

main.dark .priority-6 {
  color: #bbb;
}

main.dark .table-striped>tbody>tr.priority-6:nth-of-type(odd)>* {
  color: #bbb;
}

.priority-6 td:first-child div {
  width: 4px;
  height: 24px;
}

.priority-7 {
  color: #aaa;
}

main.dark .priority-7 {
  color: #666;
}

main.dark .table-striped>tbody>tr.priority-7:nth-of-type(odd)>* {
  color: #666;
}

.priority-7 td:first-child div {
  width: 4px;
  height: 24px;
}

.full-entry tr td {
  padding-left: 20px;
}
</style>
