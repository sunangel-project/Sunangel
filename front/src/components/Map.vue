<template>
    <!-- style is fucked up and here, otherwise it wont work -->
    <ol-map :loadTilesWhileAnimating="true" :loadTilesWhileInteracting="true" style="height: 1px; min-height: 100vh;">
        <ol-view ref="view" :center="mapCenter" @centerChanged="storeMapCenter" :zoom="mapZoom" @zoomChanged="storeMapZoom"
            :projection="projection" />

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
import SearchCircle from "./mapElements/SearchCircle.vue"
import SpotPoint from "./mapElements/SpotPoint.vue"
import { searchCenter, searchRadius, mapCenter, storeMapCenter, mapZoom, storeMapZoom, spots } from "../state"
import { projection } from "../projection"
</script>
