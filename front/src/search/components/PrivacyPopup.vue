<script setup lang="ts">
import { VueFinalModal } from 'vue-final-modal'

defineEmits(['confirm'])
</script>

<template>
    <VueFinalModal class="flex justify-center items-center"
        content-class="flex flex-col max-w-xl mx-4 p-4 bg-white dark:bg-gray-900 border dark:border-gray-700 rounded-lg space-y-4"
        overlay-transition="vfm-fade" content-transition="vfm-fade">
        <h1 class="text-2xl">Privacy Notice</h1>
        <p>
            This website stores personal data such as your IP address.
            Also the location data you choose to enter is stored.
            By using this website, you agree to this.
            Details can be accessed at any time via the link labeled as "Privacy" at the bottom of this website.
        </p>
        <button class="bg-green-700 px-5 py-1 rounded-lg shadow-md" @click="$emit('confirm')">
            Don't show again
        </button>
    </VueFinalModal>
</template>
