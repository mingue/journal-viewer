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
    "_SOURCE_REALTIME_TIMESTAMP",
    "PRIORITY",
    "_COMM",
    "MESSAGE",
    "_TRANSPORT",
  ],
  priority: 5,
  offset: 0,
  limit: 100,
};

invoke<JournalEntries>("get_logs", {
  query: JournalQuery,
}).then((response) => {
  vm.logs = response;
});
</script>

<template>
  <header></header>
  <main>
    <div class="container-fluid">
      <table class="table table-striped table-hover table-borderless table-sm">
        <thead>
          <th v-for="header in vm.logs.headers">
            {{ header }}
          </th>
        </thead>
        <tbody class="table-group-divider">
          <tr v-for="logEntry in vm.logs.rows">
            <td v-for="field in logEntry">
              <span :title="field">
                {{
                  field.length > 500 ? field.substring(0, 500) + "..." : field
                }}
              </span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </main>
</template>

<!-- <style scoped>

</style> -->
