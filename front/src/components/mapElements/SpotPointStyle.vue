<template>
    <ol-style>
        <ol-style-circle :radius="radius">
            <ol-style-fill :color="fillColor"></ol-style-fill>
        </ol-style-circle>
    </ol-style>
</template>

<script lang="ts" setup>
import { ref } from 'vue';

const radius = ref(10);

const props = defineProps({
    selected: {
        type: Boolean,
    },
});

const fillColor = ref('green');
if (props.selected) {
    fillColor.value = 'blue';
}
</script>
