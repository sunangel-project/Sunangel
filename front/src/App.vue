<template>
    <div class="h-screen grid grid-rows-[35%_1fr] md:grid-rows-1 md:grid-cols-[70%_1fr]">
        <div>
            <Map />
        </div>
        <div class="flex flex-col">
            <SearchInput />
            <SpotList class="grow overflow-auto" />
            <VersionInfo />
        </div>
    </div>

    <ModalsContainer />
</template>

<script setup lang="ts">
import SearchInput from './components/SearchInput.vue'
import SpotList from './components/Results/SpotList.vue'
import VersionInfo from './components/VersionInfo.vue'
import Map from './components/Map.vue'

import { ModalsContainer } from 'vue-final-modal'

import { setupGraphQLClient, fetchBackendVersions } from './graphql';
import { setupSpotsSubscription } from './searching';

setupGraphQLClient();

fetchBackendVersions();
setupSpotsSubscription();
</script>
