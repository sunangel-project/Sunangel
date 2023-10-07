<template>
    sunrise: {{ renderTime(spot.events.sun.rise.time) }} <br />
    sunset: {{ renderTime(spot.events.sun.set.time) }}
    moonrise: {{ renderTime(spot.events.moon.rise.time) }} <br />
    moonset: {{ renderTime(spot.events.moon.set.time) }}
</template>

<script lang="ts" setup>
import type { PropType } from "vue";

import { type Spot } from "../../state";

const props = defineProps({
    spot: {
        type: Object as PropType<Spot>,
        required: true,
    }
});

function renderTime(t: string): string {
    const date = new Date(t);
    return date.toLocaleTimeString()
}
</script>
