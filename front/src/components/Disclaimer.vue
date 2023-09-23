<script setup lang="ts">
import { VueFinalModal } from 'vue-final-modal'
</script>

<template>
    <VueFinalModal class="disclaimer-modal" content-class="disclaimer-modal-content" overlay-transition="vfm-fade"
        content-transition="vfm-fade">
        <p>
            Hi, <br>
            This website currently produces wrong results. <br>
            We are working on the issue - sorry for the inconvenience.
            <br> <br>
            In the meantime you can use the CLI: <br>
            github.com/sunangel-project/sunangel-cli
        </p>
    </VueFinalModal>
</template>

<style>
.disclaimer-modal {
    display: flex;
    justify-content: center;
    align-items: center;
}

.disclaimer-modal-content {
    display: flex;
    flex-direction: column;
    padding: 1rem;
    background: var(--color-background);
    border-radius: 0.5rem;
}

.disclaimer-modal-content>*+* {
    margin: 0.5rem 0;
}

.disclaimer-modal-content h1 {
    font-size: 1.375rem;
}

.disclaimer-modal-content button {
    margin: 0.25rem 0 0 auto;
    padding: 0 8px;
    border: 1px solid;
    border-radius: 0.5rem;
}

.dark .confirm-modal-content {
    background: #000;
}
</style>
