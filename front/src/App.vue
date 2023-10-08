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
    <div class="bg-gray-900 grid grid-cols-4">
        <div class="col-span-3">
            <Map />
        </div>
        <div>
            <SearchInput />
            <SpotList />
        </div>
    </div>

    <!--ModalsContainer /-->
</template>
