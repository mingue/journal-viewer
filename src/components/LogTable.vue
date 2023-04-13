<script setup lang="ts">
import { onMounted, onUnmounted, reactive, ref } from "vue";
import type { JournalEntries } from "@/model/JournalEntries";

const props = defineProps<{
  logs: JournalEntries;
}>();

const emit = defineEmits<{
  (e: "load-more"): void;
}>();

let vm = reactive({
  tableTheme: "",
});

const scrollComponent = ref<Element | null>(null);

type ColumnViewOptions = {
  name: string;
  formatFn: (val: string) => string;
  visible: boolean;
  index: number;
  style: any;
};

const dateFormat = {
  month: "numeric",
  day: "numeric",
  hour: "numeric",
  minute: "numeric",
  second: "numeric",
  hour12: false,
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
    formatFn: (epochTime: string) => {
      if (epochTime != null) {
        try {
          return new Date(parseInt(epochTime) / 1000).toLocaleString(undefined, dateFormat);
        } catch (error) {
          return epochTime;
        }
      }
      return epochTime;
    },
    visible: true,
    style: { width: "8rem" },
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
  const match = window.matchMedia("(prefers-color-scheme: dark)");

  if (match) {
    vm.tableTheme = "table-dark";
  }

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
</script>

<template>
  <!-- Log table -->
  <div class="container-fluid" ref="scrollComponent">
    <table class="table table-striped table-hover table-borderless table-sm" :class="vm.tableTheme">
      <thead>
        <th v-for="c in columnViewOptions.filter((x) => x.visible)" :style="c.style">
          {{ c.name }}
        </th>
      </thead>
      <tbody class="table-group-divider">
        <tr v-for="row in logs.rows" :class="getRowClass(row)">
          <td v-for="c in columnViewOptions.filter((x) => x.visible)" :style="c.style">
            <div :title="row[c.index]">
              {{ c.formatFn != null ? c.formatFn(row[c.index]) : row[c.index] }}
            </div>
          </td>
        </tr>
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

.priority-3 {
}

.priority-3 td:first-child div {
  width: 4px;
  height: 24px;
  background-color: red;
}

.priority-4 {
}

.priority-4 td:first-child div {
  width: 4px;
  height: 24px;
  background-color: rgb(255, 136, 0);
}

.priority-5 {
}

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
main.dark .table-striped > tbody > tr.priority-6:nth-of-type(odd) > * {
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
main.dark .table-striped > tbody > tr.priority-7:nth-of-type(odd) > * {
  color: #666;
}

.priority-7 td:first-child div {
  width: 4px;
  height: 24px;
}
</style>
