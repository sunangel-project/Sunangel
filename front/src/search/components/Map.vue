<template>
  <ol-map @click="handleClick" @moveend="storeMapState" style="height: 100%" :loadTilesWhileAnimating="true"
    :loadTilesWhileInteracting="true">
    <ol-view ref="view" :center="mapState.center" @change:center="centerChanged" :zoom="mapState.zoom"
      @change:resolution="resolutionChanged" :projection="projection" />

    <ol-tile-layer>
      <ol-source-osm />
    </ol-tile-layer>

    <ol-vector-layer :key="vectorLayerKey">
      <ol-source-vector>
        <SpotPoint v-for="spot in spots.spots" :spot="spot" />
      </ol-source-vector>
    </ol-vector-layer>
  </ol-map>
</template>

<script lang="ts" setup>
import type MapBrowserEvent from "ol/MapBrowserEvent";
import type { FeatureLike } from "ol/Feature";

import SearchCircle from "./mapElements/SearchCircle.vue";
import SpotPoint from "./mapElements/SpotPoint.vue";

import {
  mapState,
  centerChanged,
  resolutionChanged,
  storeMapState,
  spots,
  type Spot,
} from "../state";
import { projection } from "../projection";
import { ref } from "vue";

const vectorLayerKey = ref(0);

/*
Not using the built-in selection mechanism as it does not provide the desired behavior
*/
function handleClick(event: MapBrowserEvent<PointerEvent>) {
  event.map.forEachFeatureAtPixel(event.pixel, featureSelected);
  vectorLayerKey.value++;
}

function featureSelected(feature: FeatureLike) {
  const spot = feature.get("spot") as Spot;
  if (!spot) {
    return;
  }

  if (!spot.selectedId) {
    spot.selectedId = spots.nextSelectedId;
    spots.nextSelectedId++;
  } else {
    spot.selectedId = undefined;
  }
}
</script>
