<template>
    <div class="flex flex-col m-2 gap-2 md:gap-3">
        <SpotElement v-for="spot of selectedSpots" :spot="spot" />
    </div>
</template>

<script lang="ts" setup>
import { spots } from "@/state";
import SpotElement from "./SpotElement.vue"

import { computed } from "vue";

const selectedSpots = computed(
    () => spots.spots.filter(
        spot => spot.selectedId != null
    ).sort(
        (a, b) => a.selectedId! - b.selectedId!
    )
);
</script>
