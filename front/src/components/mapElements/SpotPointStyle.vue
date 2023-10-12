<template>
    <ol-style class="text-white">
        <ol-style-circle :radius="radius">
            <ol-style-fill :color="fillColor"></ol-style-fill>
        </ol-style-circle>
        <ol-style-text v-if="selected" text="index">{{ index }}</ol-style-text>
    </ol-style>
</template>

<script lang="ts" setup>
import { ref } from 'vue';

import { selectedSpotIds } from '../../state';

const radius = ref(10);

const props = defineProps({
    spot_id: {
        type: String,
        required: true,
    },
});

const fillColor = ref('green');
let index = -1;

const selected = selectedSpotIds.includes(props.spot_id);
if (selected) {
    fillColor.value = '#FB923C';
    index = selectedSpotIds.indexOf(props.spot_id) + 1;
}
</script>
