<template>
    <ol-map :loadTilesWhileAnimating="true" :loadTilesWhileInteracting="true" style="height: 1px; min-height: 99vh;">
        <ol-view ref="view" :center="center" :rotation="rotation" :zoom="zoom" />

        <ol-tile-layer>
            <ol-source-osm />
        </ol-tile-layer>
    </ol-map>
</template>

<script lang="ts" setup>
import { ref } from "vue";

const center = ref([48.81872, 9.58781]);
// TODO: project coordinates with https://github.com/proj4js/proj4js
const zoom = ref(8);
const rotation = ref(0);
</script>
