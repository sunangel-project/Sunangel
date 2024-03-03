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
import { ModalsContainer, useModal } from 'vue-final-modal'

import SearchInput from './components/SearchInput.vue'
import SpotList from './components/Results/SpotList.vue'
import VersionInfo from './components/VersionInfo.vue'
import Map from './components/Map.vue'

import { setupGraphQLClient, fetchBackendVersions } from './graphql';
import { setupSpotsSubscription } from './searching';
import { dontShowPrivacyPopupAgain, showPrivacyPopup } from './state'

setupGraphQLClient();

fetchBackendVersions();
setupSpotsSubscription();

import PrivacyPopup from './components/PrivacyPopup.vue';
if (showPrivacyPopup) {
    const { open, close } = useModal({
        component: PrivacyPopup,
        attrs: {
            onConfirm() { // TODO fix
                dontShowPrivacyPopupAgain();
                close();
            },
        }
    });
    open();
}
</script>
