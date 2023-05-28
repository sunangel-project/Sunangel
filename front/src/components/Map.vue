<template>
    <!-- style is fucked up and here, otherwise it wont work -->
    <ol-map :loadTilesWhileAnimating="true" :loadTilesWhileInteracting="true" style="height: 1px; min-height: 99vh;">
        <ol-view ref="view" :center="center" :rotation="rotation" :zoom="zoom" />

        <ol-tile-layer>
            <ol-source-osm />
        </ol-tile-layer>
    </ol-map>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import proj4 from "proj4";

const center = ref(proj4("EPSG:3857", [9.58781, 48.81872]));
const zoom = ref(8);
const rotation = ref(0);
</script>
