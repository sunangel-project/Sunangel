<template>
    <ol-map @click="handleClick" @moveend="storeMapState" style="height: 100%" :loadTilesWhileAnimating="true"
        :loadTilesWhileInteracting="true">
        <ol-view ref="view" :center="mapState.center" @centerChanged="centerChanged" :zoom="mapState.zoom"
            @zoomChanged="zoomChanged" :projection="projection" />

        <ol-tile-layer>
            <ol-source-osm />
        </ol-tile-layer>

        <ol-vector-layer>
            <ol-source-vector>
                <SearchCircle :center="searchCenter" :radius="inputs.radius" />
                <SpotPoint v-for="spot_id in spots.spots.keys()" :spot_id="spot_id" />
            </ol-source-vector>
        </ol-vector-layer>
    </ol-map>
</template>

<script lang="ts" setup>
import type MapBrowserEvent from "ol/MapBrowserEvent";
import type { FeatureLike } from "ol/Feature";

import SearchCircle from "./mapElements/SearchCircle.vue"
import SpotPoint from "./mapElements/SpotPoint.vue"

import {
    searchCenter, inputs, mapState, centerChanged, zoomChanged,
    storeMapState, spots, selectedSpotIds
} from "../state"
import { projection } from "../projection"

/*
Not using the built-in selection mechanism as it does not provide the desired behavior
*/
function handleClick(event: MapBrowserEvent<PointerEvent>) {
    const map = event.map;
    map.forEachFeatureAtPixel(
        event.pixel,
        featureSelected,
    );
    map.changed();
    map.renderSync();
}

function featureSelected(feature: FeatureLike) {
    const kind = feature.get('kind');
    if (kind !== 'spot') { return; }

    const id = feature.get('id');
    if (selectedSpotIds.includes(id)) {
        let idx = selectedSpotIds.indexOf(id);
        selectedSpotIds[idx] = 'deselected';
    } else {
        selectedSpotIds.unshift(id);
    }
}
</script>
