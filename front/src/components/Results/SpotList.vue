<template>
    <div class="flex flex-col m-2 gap-2 md:gap-3">
        <div v-for="[index, id] of  selectedSpotIds.entries()">
            <SpotElement v-if="id !== 'deselected'" :index="index" :spot_id="id" />
        </div>
    </div>
</template>

<script lang="ts" setup>
import SpotElement from "./SpotElement.vue"

import { selectedSpotIds } from "../../state"
</script>
