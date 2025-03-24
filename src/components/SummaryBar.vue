<script setup lang="ts">
import { reactive } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { JournalEntries } from "@/model/JournalEntries";
import { formatEpoch } from "@/common/DateFormatter";

const emit = defineEmits<{
  (e: "toggle-theme"): void;
}>();

const MAX_PERIOD_DAYS = 5;
const BLOCK_TIME_DURATION_SECONDS = 900;

let vm = reactive({
  logSummaryEntries: {} as Record<string, number>,
  isDarkThemeOn: true,
});

let summaryQuery = {
  priority: parseInt("6"),
};

type EntriesPerBlockOfTime = Record<string, number>;

let maxSummaryValue = 0;

invoke<JournalEntries>("get_summary", {
  query: summaryQuery,
}).then((response) => {
  // Set timestamp to blocks of 15m
  let logEntries = response.rows.map((r) => Math.floor(Math.floor(parseInt(r[0]) / 1_000_000) / BLOCK_TIME_DURATION_SECONDS));

  // Count entries per block of time
  let itemsPerBlock: EntriesPerBlockOfTime = {};
  logEntries.forEach(function (x) {
    itemsPerBlock[x] = (itemsPerBlock[x] || 0) + 1;
  });

  // Fill empty blocks with 0 value as there might be no log entries for a block of time
  let keysStr = Object.keys(itemsPerBlock);
  let keys = keysStr.map((x) => parseInt(x));
  let itCount = 0;

  for (let index = keys[0]; index <= keys[keys.length - 2]; index++) {
    itCount++;

    // Don't fill more than x days of blanks in case there is something weird with the data coming from systemd
    // Like records in 2078 for some reason recorded on my system to avoid memory leaks
    if (itCount > MAX_PERIOD_DAYS * 24 * (BLOCK_TIME_DURATION_SECONDS / 60)) break;

    if (!keys.includes(index)) {
      itemsPerBlock[`${index}`] = 0;
    }
  }

  // Return to epoch time in ms with their count
  let itemsPerTimestampBlock: EntriesPerBlockOfTime = {};
  for (const k in itemsPerBlock) {
    const blockTimestamp = parseInt(k) * 1000 * BLOCK_TIME_DURATION_SECONDS;

    if (isNaN(blockTimestamp)) {
      continue;
    }

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

function toggleTheme() {
  vm.isDarkThemeOn = !vm.isDarkThemeOn;
  emit("toggle-theme");
}
</script>

<template>
  <h4 class="float-start">Latest entries</h4>
  <div style="padding-top: 6px" class="d-inline-block">
    <i class="bi bi-info-circle-fill d-inline-block"
      title="Summary of log entries from the last 5 days up to 10k entries"></i>
  </div>
  <div style="padding-top: 6px; padding-right: 20px; cursor: pointer" class="float-end d-inline-block"
    @click="toggleTheme">
    <i class="bi bi-lightbulb d-inline-block" title="Toggle theme" v-if="vm.isDarkThemeOn"></i>
    <i class="bi bi-lightbulb-fill d-inline-block" title="Toggle theme" v-if="!vm.isDarkThemeOn"></i>
  </div>
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
      :title="`Date: ${formatEpoch(k)}, Value: ${v}`">
      <div class="summary-value" :style="{ height: (v / maxSummaryValue) * 100 + '%' }">&nbsp;</div>

      <div class="summary-x-legend" :class="{ 'd-none': index % 10 != 0 }">
        {{ getXLegendDate(k, index) }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.summary-bar {
  height: 100px;
  margin: 1rem;
  margin-bottom: 4rem;
  background-color: #eee;
  position: relative;
  width: 98%;
}

h4 {
  padding-top: 4px;
  padding-left: 16px;
  padding-right: 10px;
}

main.dark h4 {
  color: #eee;
}

main .bi {
  padding-top: 4px;
}

main.dark .bi {
  color: #eee;
}

main.dark .summary-bar {
  background-color: #444;
}

.summary-y-legend {
  position: absolute;
  left: 0;
  height: 100%;
  width: 100%;
}

main.dark .summary-y-legend {
  color: #ddd;
}

.summary-y-legend .y-legend {
  border-bottom: 1px solid #aaa;
  font-size: 0.8rem;
}

.summary-cell {
  max-width: 1.5rem;
  position: relative;
}

.summary-cell:hover {
  background-color: #ddd;
  opacity: 0.6;
}

main.dark .summary-cell:hover {
  background-color: #666;
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

main.dark .summary-value {
  background-color: rgb(192, 78, 78);
  border: 1px solid rgb(224, 14, 14);
}

.summary-cell:hover .summary-value {
  background-color: rgb(255, 0, 0);
  opacity: 1;
}

.summary-x-legend {
  position: absolute;
  bottom: -50px;
  rotate: 30deg;
  font-size: 0.8rem;
  text-align: left;
  width: 100px;
}

main.dark .summary-x-legend {
  color: #ddd;
}
</style>
