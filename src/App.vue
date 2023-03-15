<script setup lang="ts">
import { reactive } from "vue";
import { invoke } from "@tauri-apps/api";

let vm = reactive({
  logs: {} as JournalEntries,
});

type JournalEntries = {
  headers: Array<string>;
  rows: Array<Array<string>>;
};

const JournalQuery = {
  fields: [
    "PRIORITY",
    "_SOURCE_REALTIME_TIMESTAMP",
    "_COMM",
    "MESSAGE",
    "_TRANSPORT",
  ],
  priority: 7,
  offset: 0,
  limit: 100,
};

type ColumnViewOptions = {
  name: string;
  formatFn: (val: string) => string;
  visible: boolean;
  index: number;
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
    formatFn: (val: string) => " ",
    visible: true,
  },
  {
    name: "Time",
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
  },
  {
    name: "Process",
    formatFn: null,
    visible: true,
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
  },
  {
    name: "Transport",
    formatFn: null,
    visible: false,
  },
] as ColumnViewOptions[];

columnViewOptions.forEach((c, i) => {
  c.index = i;
});

invoke<JournalEntries>("get_logs", {
  query: JournalQuery,
}).then((response) => {
  vm.logs = response;
});

const getRowClass = (row: Array<string>) => `priority-${row[0]}`;
</script>

<template>
  <header></header>
  <main>
    <div class="container-fluid">
      <table class="table table-striped table-hover table-borderless table-sm">
        <thead>
          <th v-for="c in columnViewOptions.filter(x => x.visible)">
            {{ c.name }}
          </th>
        </thead>
        <tbody class="table-group-divider">
          <tr v-for="row in vm.logs.rows" :class=getRowClass(row)>
            <td v-for="c in columnViewOptions.filter(x => x.visible)">
              <div :title="row[c.index]">
                {{ c.formatFn != null ? c.formatFn(row[c.index]) : row[c.index] }}
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </main>
</template>

<style scoped>
.priority-0 {
  font-weight: 500;
  background-color: black;
  color: white;
}

.priority-0 td:first-child div {
  width: 4px;
  height: 24px;
}

.priority-1 {
  font-weight: 500;
  background-color: rgb(116, 26, 26);
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

.priority-6 td:first-child div {
  width: 4px;
  height: 24px;
}

.priority-7 {
  color: #aaa;
}

.priority-7 td:first-child div {
  width: 4px;
  height: 24px;
}
</style>
