<template>
    <ol-feature :properties="featureProperties">
        <ol-geom-point :coordinates="coordinates"></ol-geom-point>
        <SpotPointStyle />
    </ol-feature>
</template>

<script lang="ts" setup>
import { ref } from 'vue';

import { project } from "../../projection"

import SpotPointStyle from './SpotPointStyle.vue';

const props = defineProps({
    spot: {
        type: Object,
    },
});

const spot = props.spot!;

const featureProperties = {
    'kind': 'spot',
    'id': spot.id,
};
const coordinates = ref(project(spot.location.lat, spot.location.lon));
</script>
