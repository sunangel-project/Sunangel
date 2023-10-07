<script setup lang="ts">
import SearchInput from './components/SearchInput.vue'
import SpotList from './components/Results/SpotList.vue'
import Map from './components/Map.vue'

import { setupSpotsSubscription } from './searching';
setupSpotsSubscription()

/* Modal Pop-up
// Disclaimer
import { ModalsContainer, useModal } from 'vue-final-modal'
import Disclaimer from './components/Disclaimer.vue'
const { open } = useModal({
    component: Disclaimer,
})
open();
*/
</script>

<template>
    <div class="grid">
        <div class="map">
            <Map />
        </div>
        <div class="input">
            <SearchInput />
            <SpotList />
        </div>
    </div>

    <ModalsContainer />
</template>

<style scoped>
.grid {
    display: grid;
    grid-template-columns: 80% 10%;
    grid-template-rows: 100%;
}

.map {
    grid-column: 1;
    grid-row: 1;
}

.input {
    grid-column: 2;
    grid-row: 1;
}
</style>
