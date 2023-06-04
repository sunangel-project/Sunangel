<template>
    <!-- style is fucked up and here, otherwise it wont work -->
    <ol-map :loadTilesWhileAnimating="true" :loadTilesWhileInteracting="true" style="height: 1px; min-height: 100vh;">
        <ol-view ref="view" :center="center" :rotation="rotation" :zoom="zoom" :projection="projection" />

        <ol-tile-layer>
            <ol-source-osm />
        </ol-tile-layer>

        <ol-vector-layer>
            <ol-source-vector>
                <ol-feature>
                    <ol-geom-circle :center="center" :radius="radius"></ol-geom-circle>
                    <ol-style>
                        <ol-style-stroke color="blue" :width="3"></ol-style-stroke>
                        <ol-style-fill color="rgba(20,0,100,0.1)"></ol-style-fill>
                    </ol-style>
                </ol-feature>
            </ol-source-vector>
        </ol-vector-layer>
    </ol-map>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import proj4 from "proj4";

const projection = ref("EPSG:3857");
const center = ref(proj4(projection.value, [9.58781, 48.81872]));
//const zoom = ref(13);
const zoom = ref(16);
const rotation = ref(0);
const radius = ref(2000);
</script>
