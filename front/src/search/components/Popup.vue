<script setup lang="ts">
import { VueFinalModal } from 'vue-final-modal'

defineProps({
    title: String,
    message: String,
});
</script>

<template>
    <VueFinalModal class="flex justify-center items-center"
        content-class="flex flex-col max-w-xl mx-4 p-4 bg-white dark:bg-gray-900 border dark:border-gray-700 rounded-lg space-y-2"
        overlay-transition="vfm-fade" content-transition="vfm-fade">
        <h1 class="text-2xl">{{ title }}</h1>
        <p>
            {{ message }}
        </p>
    </VueFinalModal>
</template>
