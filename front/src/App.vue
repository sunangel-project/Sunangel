<template>
    <div class="grid">
        <div class="map">
            <Map />
        </div>
        <div class="input">
            <SearchInput />
        </div>
    </div>
</template>

<script setup lang="ts">
import SearchInput from './components/SearchInput.vue'
import Map from './components/Map.vue'
import { gql, useQuery, useSubscription } from '@urql/vue';

let query = gql`
subscription spot($lat: Float!, $lon: Float!, $radius: Int!) {
  spots(query: { location: { lat: $lat, lon: $lon }, radius: $radius }) {
    status
    spot {
      location {
        lat
        lon
      }
      kind
      events {
        sun {
          rise {
            time
            altitude
            azimuth
          }
          set {
            time
            altitude
            azimuth
          }
        }
      }
    }
  }
}
`;


let result = useSubscription({
    query: query,
    variables:
    {
        lat: 48.81872,
        lon: 9.58781,
        radius: 2000,
    },
})

const sleep = (ms: number) => {
    return new Promise(resolve => setTimeout(resolve, ms))
}

console.log(result)
console.log(result.fetching)
sleep(10000).then(() => {
    console.log(result.data.value)
    console.log(result)
})

</script>

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
