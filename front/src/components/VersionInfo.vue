<template>
    <div class="flex flex-row place-content-evenly text-green-700 m-2 md:m-3">
        <p> front: {{ version }} </p>
        <p> API: {{ connection.apiVersion }} </p>
        <p> back: {{ connection.backendVersion }} </p>
    </div>
</template>

<script setup lang="ts">
import { version } from '../main';
import { connection } from '../state';
</script>
