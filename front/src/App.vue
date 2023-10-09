<template>
    <div class="h-screen grid grid-rows-[45%_1fr] md:grid-rows-1 md:grid-cols-[70%_1fr]">
        <div>
            <Map />
        </div>
        <div>
            <SearchInput />
            <SpotList />
        </div>
    </div>

    <ModalsContainer />
</template>

<script setup lang="ts">
import SearchInput from './components/SearchInput.vue'
import SpotList from './components/Results/SpotList.vue'
import Map from './components/Map.vue'

import { cacheExchange, fetchExchange, subscriptionExchange, Client, provideClient } from '@urql/vue';
import { createClient as createWSClient, type SubscribePayload } from 'graphql-ws';
import { setupSpotsSubscription } from './searching';

import { ModalsContainer, useModal } from 'vue-final-modal'
import Popup from './components/Popup.vue'

const displayConnectionError = () => {
    const { open } = useModal({
        component: Popup,
        attrs: {
            title: "Error",
            message: "Couldn't connect to the backend... Please try again later.",
        },
    });
    open();
};


let protocol = "ws";
let apiHost = "localhost";
apiHost = "192.168.2.123";
if (process.env.NODE_ENV == "production") {
    protocol = "wss";
    apiHost = "sunnapi.cloudsftp.de";
}

const wsClient = createWSClient({
    url: `${protocol}://${apiHost}:6660/subscriptions`,
    on: {
        error: displayConnectionError,
    },
});

const subExchange = subscriptionExchange({
    forwardSubscription(operation) {
        return {
            subscribe: (sink) => {
                const dispose = wsClient.subscribe(
                    operation as SubscribePayload,
                    sink,
                );
                return {
                    unsubscribe: dispose,
                };
            },
        };
    },
});

const client = new Client({
    url: `http://${apiHost}:6660/graphql`,
    exchanges: [
        cacheExchange,
        fetchExchange,
        subExchange,
    ],
});

provideClient(client);

setupSpotsSubscription()
</script>
