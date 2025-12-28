<script lang="ts" setup>
import { search } from "../searching";
import { interfaceState, connection, spots } from "../state";
</script>

<template>
  <div class="flex flex-col gap-1 m-2 md:m-3">
    <button v-if="!connection.connected" class="bg-red-700 search-button-shape">
      No Connection
    </button>
    <button v-else-if="spots.loading" class="bg-blue-700 search-button-shape">
      Loading Spots...
    </button>
    <button v-else-if="interfaceState.areaTooLarge" class="bg-red-700 search-button-shape">
      Area Too Large - Please Zoom In
    </button>
    <button v-else class="bg-green-700 search-button-shape" @click="search">
      Find Benches in Area
    </button>
  </div>
</template>


<style>
@import "tailwindcss";

.search-button-shape {
  @apply py-2 my-2 mx-1 font-semibold rounded-lg shadow-md;
}
</style>
