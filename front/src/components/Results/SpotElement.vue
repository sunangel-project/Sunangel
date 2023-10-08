<template>
    <div class="bg-gray-800 rounded-md shadow-md">
        <div class="grid grid-cols-3 gap-1 py-2 px-3">
            <p>sun:</p>
            <p>{{ renderTime(spot.events.sun.rise.time) }}</p>
            <p>{{ renderTime(spot.events.sun.set.time) }}</p>
            <p>moon:</p>
            <p>{{ renderTime(spot.events.moon.rise.time) }}</p>
            <p>{{ renderTime(spot.events.moon.set.time) }}</p>
        </div>
    </div>
</template>

<script lang="ts" setup>
import type { PropType } from "vue";

import { type Spot } from "../../state";

defineProps({
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
