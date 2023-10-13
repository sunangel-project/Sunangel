<template>
    <ol-feature :properties="featureProperties">
        <ol-geom-point :coordinates="coordinates"></ol-geom-point>
        <ol-style>
            <ol-style-circle :radius="radius">
                <ol-style-fill :color="fillColor" />
            </ol-style-circle>
            <ol-style-text v-if="selected" :text="index.toString()" />
        </ol-style>
    </ol-feature>
</template>

<script lang="ts" setup>
import { ref, type PropType } from 'vue';

import { project } from '../../projection'
import { type Spot } from '../../state'

const radius = ref(10);

const props = defineProps({
    spot: {
        type: Object as PropType<Spot>,
        required: true,
    },
});

const featureProperties = {
    'spot': props.spot,
};
const coordinates = ref(project(props.spot.location.lat, props.spot.location.lon));

const fillColor = ref('green');
let index = -1;
const selected = props.spot.selectedId != undefined;
if (selected) {
    fillColor.value = '#FB923C';
    index = props.spot.selectedId!;
}
</script>
