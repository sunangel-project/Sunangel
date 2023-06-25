<template>
    <ol-feature>
        <ol-geom-point :coordinates="coordinates"></ol-geom-point>
        <ol-style>
            <ol-style-circle :radius="radius">
                <ol-style-fill :color="fillColor"></ol-style-fill>
            </ol-style-circle>
        </ol-style>
    </ol-feature>
</template>

<script lang="ts" setup>
import { ref } from 'vue';

import { projection } from "../../projection"
import proj4 from 'proj4';

const radius = ref(10);
const fillColor = ref("green");

const props = defineProps({
    spot: {
        type: Object,
    },
})

const spot = props.spot!;
const coordinates = ref(proj4(projection.value, [spot.location.lon, spot.location.lat]))

</script>
