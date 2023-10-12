<template>
    <ol-feature :properties="featureProperties">
        <ol-geom-point :coordinates="coordinates"></ol-geom-point>
        <SpotPointStyle :spot_id="spot_id" />
    </ol-feature>
</template>

<script lang="ts" setup>
import { ref } from 'vue';

import { project } from '../../projection'
import { spots } from '../../state'

import SpotPointStyle from './SpotPointStyle.vue';

const props = defineProps({
    spot_id: {
        type: String,
        required: true,
    },
});

const spot = spots.spots.get(props.spot_id)!;

const featureProperties = {
    'kind': 'spot',
    'id': spot.id,
};
const coordinates = ref(project(spot.location.lat, spot.location.lon));
</script>
