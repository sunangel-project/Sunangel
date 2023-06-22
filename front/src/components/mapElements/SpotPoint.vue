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

const radius = ref(10);
const fillColor = ref("green");

// different props
const props = defineProps({
    coordinates: {
        type: Array,
        default: [0, 0],
        validator(value: number[]): boolean {
            return value.length == 2
        }
    }
})
</script>
