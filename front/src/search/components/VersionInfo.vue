<template>
    <div class="flex flex-row place-content-evenly text-green-700 text-sm m-1 md:m-2">
        <p> frontend {{ version }} </p>
        <p class="text-neutral-400">
            <RouterLink to="/privacy">Privacy</RouterLink>
        </p>
        <p :class="(connection.backendVersion) ? '' : 'text-red-700'">
            backend {{ connection.backendVersion ?? 'unreachable' }}
        </p>
    </div>
</template>

<script setup lang="ts">
import { version } from '../../main';
import { connection } from '../state';
</script>
