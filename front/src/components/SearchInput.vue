<script lang="ts">
import searching from '../searching'

export default {
    data() {
        return {
            lat: 48.74517,
            lon: 9.10666,
            radius: 2000,
        }
    },
    methods: {
        search: function () {
            searching.search(this.lat, this.lon, this.radius)
        }
    }
}
</script>

<template>
    <h2>Search Input</h2>

    <div>
        Latitude:
        <input v-model="lat">
    </div>

    <div>
        Longitude:
        <input v-model="lon">
    </div>

    <div>
        Search Radius:
        <input v-model="radius" type="number">
    </div>

    <button @click="search">Search</button>
</template>
