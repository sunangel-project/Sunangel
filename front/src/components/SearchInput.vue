<script lang="ts" setup>
import { search } from '../searching'
import { inputs } from "../state"
</script>

<template>
    <div class="flex flex-col gap-1 m-2 md:m-3">
        <p class="text-2xl font-medium">Search Input</p>

        <div class="grid grid-cols-2 gap-1.5">
            <p>Latitude:</p>
            <input class="number-input-form" v-model="inputs.lat" type="number">

            <p>Longitude:</p>
            <input class="number-input-form" v-model="inputs.lon" type="number">

            <p>Search Radius:</p>
            <input class="number-input-form" v-model="inputs.radius" type="number">
        </div>

        <button class="bg-green-700 py-2 my-2 mx-1 font-semibold rounded-lg shadow-md" @click="search">Search</button>
    </div>
</template>

<style>
.number-input-form {
    @apply bg-gray-700;
}
</style>
