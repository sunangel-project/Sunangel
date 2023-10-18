<template>
    <div class="flex flex-row place-content-evenly text-green-700 text-sm m-2 md:m-3">
        <p> frontend {{ version }} </p>
        <p :class="(connection.apiVersion) ? '' : 'text-red-700'">
            API {{ connection.apiVersion ?? 'unreachable' }}
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
