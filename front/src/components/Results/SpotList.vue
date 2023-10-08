<template>
    <div class="flex flex-col m-2 gap-2 md:gap-3">
        <div v-for="spot in selectedSpots">
            <SpotElement :spot="spot" />
        </div>
    </div>
</template>

<script lang="ts" setup>
import SpotElement from "./SpotElement.vue"

import { computed } from "vue"
import { spots, selectedSpotIds } from "../../state"

const selectedSpots = computed(
    () => spots.spots.filter((spot) => selectedSpotIds.has(spot.id))
)
</script>
