<template>
    <div class="bg-gray-800 rounded-md shadow-md">
        <div class="grid grid-cols-[20%_40%_1fr] gap-1 py-2 px-3 text-center">
            <p>#{{ spot.selectedId }}</p>
            <p class="font-semibold">rise</p>
            <p class="font-semibold">set</p>
            <p class="font-semibold">sun</p>
            <p>{{ renderTime(spot.events.sun.rise.time) }}</p>
            <p>{{ renderTime(spot.events.sun.set.time) }}</p>
            <p class="font-semibold">moon</p>
            <p>{{ renderTime(spot.events.moon.rise.time) }}</p>
            <p>{{ renderTime(spot.events.moon.set.time) }}</p>
        </div>
    </div>
</template>

<script lang="ts" setup>
import type { Spot } from '../../state';
import type { PropType } from 'vue';

defineProps({
    spot: {
        type: Object as PropType<Spot>,
        required: true,
    },
});

function renderTime(t: string): string {
    const date = new Date(t);
    return date.toLocaleTimeString()
}
</script>
