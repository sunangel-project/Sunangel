<template>
    <!-- style is fucked up and here, otherwise it wont work -->
    <ol-map :loadTilesWhileAnimating="true" :loadTilesWhileInteracting="true" @moveend="storeMapState"
        style="height: 1px; min-height: 100vh;">
        <ol-view ref="view" :center="mapState.center" @centerChanged="centerChanged" :zoom="mapState.zoom"
            @zoomChanged="zoomChanged" :projection="projection" />

        <ol-tile-layer>
            <ol-source-osm />
        </ol-tile-layer>

        <ol-vector-layer>
            <ol-source-vector>
                <SearchCircle :center="searchCenter" :radius="inputs.radius" />
                <SpotPoint v-for="spot in spots.spots" :spot="spot" />
            </ol-source-vector>
        </ol-vector-layer>
    </ol-map>
</template>

<script lang="ts" setup>
import SearchCircle from "./mapElements/SearchCircle.vue"
import SpotPoint from "./mapElements/SpotPoint.vue"
import { searchCenter, inputs, mapState, centerChanged, zoomChanged, storeMapState, spots } from "../state"
import { projection } from "../projection"
</script>
