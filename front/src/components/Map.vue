<template>
    <!-- style is fucked up and here, otherwise it wont work -->
    <ol-map :loadTilesWhileAnimating="true" :loadTilesWhileInteracting="true" @moveend="storeMapState"
        style="height: 1px; min-height: 100vh;">
        <ol-view ref="view" :center="mapState.center" @centerChanged="centerChanged" :zoom="mapState.zoom"
            @zoomChanged="zoomChanged" :projection="projection" />

        <ol-tile-layer>
            <ol-source-osm />
        </ol-tile-layer>

        <ol-interaction-select @select="featureSelected" :condition="selectCondition" :filter="selectInteactionFilter">
            <ol-style>
                <ol-style-circle :radius="radius">
                    <ol-style-fill :color="'blue'"></ol-style-fill>
                </ol-style-circle>
            </ol-style>
        </ol-interaction-select>

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

import { inject, ref } from "vue";
const selectConditions = inject("ol-selectconditions");
const selectCondition = selectConditions.click;

function featureSelected(event) {
    console.log(event)
}

function selectInteactionFilter(feature: any) {
    // TODO: filter for search spots
    return true;
};

const radius = ref(10); // duplicate
</script>
