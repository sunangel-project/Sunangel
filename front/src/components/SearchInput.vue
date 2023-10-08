<script lang="ts" setup>
import { search } from '../searching'
import { inputs } from "../state"
</script>

<template>
    <h2>Search Input</h2>

    <div>
        Latitude:
        <input v-model="inputs.lat" type="number">
    </div>

    <div>
        Longitude:
        <input v-model="inputs.lon" type="number">
    </div>

    <div>
        Search Radius:
        <input v-model="inputs.radius" type="number">
    </div>

    <button class="py-2 px-4 font-semibold rounded-lg shadow-md" @click="search">Search</button>
</template>
