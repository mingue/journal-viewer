<script setup lang="ts">
import { reactive, onMounted, onUnmounted, ref } from "vue";
import { invoke } from "@tauri-apps/api";

let vm = reactive({
  logs: {} as JournalEntries,
  logSummaryEntries: {} as Record<string, number>,
  isSidebarCollapsed: true,
  priority: "6",
  quickSearch: "",
});
const scrollComponent = ref(null);

type JournalEntries = {
  headers: Array<string>;
  rows: Array<Array<string>>;
};

let journalQuery = {
  fields: ["PRIORITY", "_SOURCE_REALTIME_TIMESTAMP", "_COMM", "MESSAGE", "_TRANSPORT"],
  priority: parseInt(vm.priority),
  quickSearch: vm.quickSearch,
  offset: 0,
  limit: 100,
};

type ColumnViewOptions = {
  name: string;
  formatFn: (val: string) => string;
  visible: boolean;
  index: number;
};

type EntriesPerBlockOfTime = Record<string, number>;

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

let loadingLogs: false;

function getLogs(event?: Event) {
  if (event != null) {
    event.preventDefault();
  }

  vm.isSidebarCollapsed = true;

  journalQuery.offset = 0;
  journalQuery.priority = parseInt(vm.priority);
  journalQuery.quickSearch = vm.quickSearch;

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

getLogs();

onMounted(() => {
  window.addEventListener("scroll", handleScroll);
});

onUnmounted(() => {
  window.removeEventListener("scroll", handleScroll);
});

const handleScroll = () => {
  if (loadingLogs) {
    return;
  }

  if (scrollComponent.value.getBoundingClientRect().bottom < window.innerHeight) {
    loadNextLogs();
  }
};

function loadNextLogs() {
  journalQuery.offset += journalQuery.limit;

  invoke<JournalEntries>("get_logs", {
    query: journalQuery,
  }).then((response) => {
    vm.logs = {
      ...response,
      rows: vm.logs.rows.concat(response.rows),
    };
  });
}

let maxSummaryValue = 0;

invoke<JournalEntries>("get_logs_summary", {
  query: journalQuery,
}).then((response) => {
  // Set timestamp to blocks of 15m
  let logEntries = response.rows.map((r) => Math.floor(Math.floor(parseInt(r[0]) / 1_000_000) / 900));

  // Count entries per block of time
  let itemsPerBlock: EntriesPerBlockOfTime = {};
  logEntries.forEach(function (x) {
    itemsPerBlock[x] = (itemsPerBlock[x] || 0) + 1;
  });

  // Fill empty blocks with 0 vakue as there might be no log entries for a block of time
  let keysStr = Object.keys(itemsPerBlock);
  let keys = keysStr.map((x) => parseInt(x));

  for (let index = keys[0]; index < keys[keys.length - 2]; index++) {
    if (!keys.includes(index)) {
      itemsPerBlock[`${index}`] = 0;
    }
  }

  // Return to epoch time in ms with their count
  let itemsPerTimestampBlock: EntriesPerBlockOfTime = {};
  for (const k in itemsPerBlock) {
    const blockTimestamp = parseInt(k) * 1000 * 900;
    let value = itemsPerBlock[k];
    if (value > maxSummaryValue) {
      maxSummaryValue = value;
    }
    itemsPerTimestampBlock[blockTimestamp] = itemsPerBlock[k];
  }

  maxSummaryValue = maxSummaryValue * 1.2;

  // Expose data to VM
  vm.logSummaryEntries = itemsPerTimestampBlock;
});

const getRowClass = (row: Array<string>) => `priority-${row[0]}`;

const getSummaryDate = (x: string) => {
  try {
    return new Date(parseInt(x)).toLocaleString(undefined, dateFormat);
  } catch (error) {
    return x;
  }
};

const xLegendShortFormat = {
  hour: "numeric",
  minute: "numeric",
  hour12: false,
};
const xLegendDateFormat = {
  month: "numeric",
  day: "numeric",
  hour: "numeric",
  minute: "numeric",
  hour12: false,
};
const getXLegendDate = (x: string, index: number) => {
  try {
    let date = new Date(parseInt(x));
    if (index == 0 || date.getHours() < Math.floor(10 / 4)) {
      return date.toLocaleString(undefined, xLegendDateFormat);
    }
    return date.toLocaleString(undefined, xLegendShortFormat);
  } catch (error) {
    return x;
  }
};

function toggleSidebar(event: Event) {
  event.preventDefault();

  if (vm.isSidebarCollapsed) {
    vm.isSidebarCollapsed = false;
  } else {
    vm.isSidebarCollapsed = true;
  }
}
</script>

<template>
  <header></header>
  <main>
    <!-- Summary bar -->
    <div class="d-flex container-fluid summary-bar justify-content-end">
      <div class="summary-y-legend d-flex flex-column">
        <div class="flex-fill y-legend">
          {{ Math.round((maxSummaryValue * 100) / 100) }}
        </div>
        <div class="flex-fill y-legend">&nbsp;</div>
        <div class="flex-fill y-legend">
          {{ Math.round((maxSummaryValue * 50) / 100) }}
        </div>
        <div class="flex-fill y-legend">&nbsp;</div>
      </div>
      <div class="flex-fill summary-cell" v-for="(v, k, index) in vm.logSummaryEntries"
        :title="`Date: ${getSummaryDate(k)}, Value: ${v}`">
        <div class="summary-value" :style="{ height: (v / maxSummaryValue) * 100 + '%' }">&nbsp;</div>

        <div class="summary-x-legend" :class="{ visible: index % 10 == 0, invisible: index % 10 != 0 }">
          {{ getXLegendDate(k, index) }}
        </div>
      </div>
    </div>
    <!-- Quick Search -->
    <nav class="navbar bg-body-tertiary">
      <div class="container-fluid" style="border: 1px solid #ddd; padding: 1rem">
        <a class="navbar-brand">Quick Search</a>
        <form class="d-flex" role="search">
          <input class="form-control me-2" type="search" v-model="vm.quickSearch" aria-label="Search" />
          <button class="btn btn-outline-primary" type="submit" @click="getLogs">Search</button>
        </form>
      </div>
    </nav>
    <!-- Main Content -->
    <div class="d-flex">
      <div class="flex">
        <!-- Filter menu -->
        <div class="d-flex flex-column flex-shrink-0" @click="toggleSidebar">
          <ul class="nav nav-pills nav-flush flex-column mb-auto text-center">
            <li class="nav-item">
              <a href="#" class="nav-link py-3 border-bottom rounded-0" :class="{ active: !vm.isSidebarCollapsed }"
                aria-current="page" data-bs-toggle="tooltip" data-bs-placement="right" aria-label="Home"
                data-bs-original-title="Home">
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
          <button type="submit" class="btn btn-outline-primary" @click="getLogs">Filter</button>
        </form>
      </div>
      <div class="flex-fill">
        <!-- Log table -->
        <div class="container-fluid" ref="scrollComponent">
          <table class="table table-striped table-hover table-borderless table-sm">
            <thead>
              <th v-for="c in columnViewOptions.filter((x) => x.visible)">
                {{ c.name }}
              </th>
            </thead>
            <tbody class="table-group-divider">
              <tr v-for="row in vm.logs.rows" :class="getRowClass(row)">
                <td v-for="c in columnViewOptions.filter((x) => x.visible)">
                  <div :title="row[c.index]">
                    {{ c.formatFn != null ? c.formatFn(row[c.index]) : row[c.index] }}
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>
.priority-0 {
  font-weight: 500;
  background-color: black;
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
  background-color: rgb(116, 26, 26);
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

.summary-bar {
  height: 100px;
  margin: 1rem;
  margin-bottom: 5rem;
  padding-left: 1rem;
  padding-right: 2rem;
  background-color: #eee;
  position: relative;
}

.summary-y-legend {
  position: absolute;
  left: 0;
  height: 100%;
  width: 100%;
}

.summary-y-legend .y-legend {
  border-bottom: 1px solid #aaa;
  font-size: 0.4rem;
}

.summary-cell {
  max-width: 1.5rem;
  position: relative;
}

.summary-cell:hover {
  background-color: #ddd;
  opacity: 0.6;
}

.summary-value {
  background-color: rgb(238, 133, 133);
  border: 1px solid rgb(228, 96, 96);
  vertical-align: bottom;
  position: absolute;
  bottom: 0;
  left: 20%;
  width: 60%;
  margin: 0 auto;
}

.summary-cell:hover .summary-value {
  background-color: rgb(255, 0, 0);
  opacity: 1;
}

.summary-x-legend {
  position: absolute;
  bottom: -50px;
  rotate: 30deg;
  font-size: 0.6rem;
  text-align: left;
  width: 100px;
}

.filter-content {
  width: 20rem;
  padding: 1rem;
}
</style>
