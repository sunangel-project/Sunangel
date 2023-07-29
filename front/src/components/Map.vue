<template>
    <!-- style is fucked up and here, otherwise it wont work -->
    <ol-map :loadTilesWhileAnimating="true" :loadTilesWhileInteracting="true" @moveend="storeMapState"
        style="height: 1px; min-height: 100vh;">
        <ol-view ref="view" :center="mapState.center" @centerChanged="centerChanged" :zoom="mapState.zoom"
            @zoomChanged="zoomChanged" :projection="projection" />

        <ol-tile-layer>
            <ol-source-osm />
        </ol-tile-layer>

        <!--- select spots --->
        <ol-interaction-select @select="featureSelected" :condition="selectCondition" :filter="selectInteactionFilter">
            <SpotPointStyle :selected="true" />
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
import SpotPointStyle from "./mapElements/SpotPointStyle.vue"
import { searchCenter, inputs, mapState, centerChanged, zoomChanged, storeMapState, spots, selectedSpotIds } from "../state"
import { projection } from "../projection"

import { inject } from "vue";
const selectConditions = inject("ol-selectconditions");
const selectCondition = selectConditions.click;

function featureSelected(event: any) {
    for (const selectedObj of event.selected) {
        const id = selectedObj.values_.id;
        selectedSpotIds.add(id);
    }
    for (const deselectedObj of event.deselected) {
        const id = deselectedObj.values_.id;
        selectedSpotIds.delete(id);
    }
}

function selectInteactionFilter(feature: any) {
    return feature.values_.kind === 'spot';
};
</script>
