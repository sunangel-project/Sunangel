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
                <SpotPoint :coordinates="searchCenter" />
            </ol-source-vector>
        </ol-vector-layer>
    </ol-map>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import proj4 from "proj4";

import SearchCircle from "./mapElements/SearchCircle.vue"
import SpotPoint from "./mapElements/SpotPoint.vue"

const projection = ref("EPSG:3857");
const center = ref(proj4(projection.value, [9.58781, 48.81872]));
const zoom = ref(16);
const rotation = ref(0);

const searchCenter = ref(center.value);
const searchRadius = ref(2000);
</script>
