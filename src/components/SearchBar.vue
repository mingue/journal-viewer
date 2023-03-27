<script setup lang="ts">
import { reactive } from "vue";

const emit = defineEmits<{
  (e: "quick-search", search: string): void;
}>();

let vm = reactive({
  quickSearch: "",
});

function search(event?: Event) {
  if (event != null) {
    event.preventDefault();
  }

  emit("quick-search", vm.quickSearch);
}
</script>

<template>
  <!-- Quick Search -->
  <nav class="navbar">
    <div class="container-fluid">
      <a class="navbar-brand" href="#">Navbar</a>
      <form class="d-flex" role="search">
        <input
          class="form-control me-2"
          type="search"
          placeholder="Quick Search"
          v-model="vm.quickSearch"
          aria-label="Search"
        />
        <button class="btn btn-outline-primary" type="submit" @click="search">Search</button>
      </form>
    </div>
  </nav>
</template>

<style scoped>
.navbar {
  background-color: #e3f2fd;
  border: 1px solid #70b1df;
}
</style>
