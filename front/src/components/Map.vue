<template>
    <!-- style is fucked up and here, otherwise it wont work -->
    <ol-map :loadTilesWhileAnimating="true" :loadTilesWhileInteracting="true" style="height: 1px; min-height: 100vh;">
        <ol-view ref="view" :center="center" :rotation="rotation" :zoom="zoom" :projection="projection" />

        <ol-tile-layer>
            <ol-source-osm />
        </ol-tile-layer>

        <ol-vector-layer>
            <ol-source-vector>
                <SearchCircle :center="searchCenter" :radius="searchRadius" />
                <SpotPoint v-for="spot in spots.spots" :spot="spot" />
            </ol-source-vector>
        </ol-vector-layer>
    </ol-map>
</template>

<script lang="ts" setup>
import { computed, ref } from "vue";

import SearchCircle from "./mapElements/SearchCircle.vue"
import SpotPoint from "./mapElements/SpotPoint.vue"
import { inputs, spots } from "../state"
import { projection, project } from "../projection"

const center = computed(() => project(inputs.lat, inputs.lon));
const zoom = ref(15);
const rotation = ref(0);

const searchCenter = center;
const searchRadius = computed(() => inputs.radius);
</script>
