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

import { setupSpotsSubscription } from './searching';
import { Client, provideClient } from '@urql/vue';
import { SubscriptionClient } from 'subscriptions-transport-ws';
import { cacheExchange, fetchExchange, subscriptionExchange } from '@urql/vue';

import { ModalsContainer, useModal } from 'vue-final-modal'
import Popup from './components/Popup.vue'

const displayConnectionError = () => {
    const { open } = useModal({
        component: Popup,
        attrs: {
            title: "Error",
            message: "Backend is not reachable. Please try again later.",
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

const subscriptionClient = new SubscriptionClient(
    `${protocol}://${apiHost}:6660/subscriptions`,
    { reconnect: false },
);
subscriptionClient.client.onerror = displayConnectionError;

const client = new Client({
    url: `http://${apiHost}:6660/graphql`,
    exchanges: [
        cacheExchange,
        fetchExchange,
        subscriptionExchange({
            forwardSubscription: (request) => subscriptionClient.request(request),
        }),
    ],
});

provideClient(client);

setupSpotsSubscription()
</script>
