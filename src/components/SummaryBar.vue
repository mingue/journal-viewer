<script setup lang="ts">
import { reactive } from "vue";
import { invoke } from "@tauri-apps/api";
import type { JournalEntries } from "@/model/JournalEntries";

const BLOCK_TIME_DURATION_SECONDS = 900;

let vm = reactive({
  logSummaryEntries: {} as Record<string, number>,
});

let journalQuery = {
  fields: ["PRIORITY", "_SOURCE_REALTIME_TIMESTAMP", "_COMM", "MESSAGE", "_TRANSPORT"],
  priority: parseInt("6"),
  quickSearch: "",
  limit: 50,
  resetPosition: true,
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

let maxSummaryValue = 0;

invoke<JournalEntries>("get_summary", {
  query: journalQuery,
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

  for (let index = keys[0]; index < keys[keys.length - 2]; index++) {
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
</script>

<template>
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
    <div
      class="flex-fill summary-cell"
      v-for="(v, k, index) in vm.logSummaryEntries"
      :title="`Date: ${getSummaryDate(k)}, Value: ${v}`"
    >
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
</style>
