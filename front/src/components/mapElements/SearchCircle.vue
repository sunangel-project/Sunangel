<template>
    <ol-feature>
        <ol-geom-circle :center="center" :radius="radius"></ol-geom-circle>
        <ol-style>
            <ol-style-stroke color="blue" :width="3"></ol-style-stroke>
            <ol-style-fill color="rgba(20,0,100,0.1)"></ol-style-fill>
        </ol-style>
    </ol-feature>
</template>

<script lang="ts" setup>
const props = defineProps({
    center: {
        type: Array,
        default: [0, 0],
        validator(value: number[]): boolean {
            return value.length == 2
        }
    },
    radius: {
        type: Number,
        default: 2000,
        validator(value: number): boolean {
            return value > 0
        }
    }
})
</script>
