<template>
    <ul>
        <li v-for="spot in selectedSpots">
            <SpotElement :spot="spot" />
        </li>
    </ul>
</template>

<script lang="ts" setup>
import SpotElement from "./SpotElement.vue"

import { computed } from "vue"
import { spots, selectedSpotIds } from "../../state"

const selectedSpots = computed(
    () => spots.spots.filter((spot) => selectedSpotIds.has(spot.id))
)
</script>
