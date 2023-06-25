import { createApp } from 'vue'
import OpenLayersMap from 'vue3-openlayers';
import 'vue3-openlayers/dist/vue3-openlayers.css';
import urql, { cacheExchange, fetchExchange, subscriptionExchange } from '@urql/vue';


import App from './App.vue'
import './assets/main.css'
import { SubscriptionClient } from 'subscriptions-transport-ws';

// TODO: handle connection fail
const subscriptionClient = new SubscriptionClient(
    "ws://localhost:6660/subscriptions",
    { reconnect: false },
);

const app = createApp(App);
app.use(OpenLayersMap);
app.use(urql, {
    url: "http://localhost:6660/graphql",
    exchanges: [
        cacheExchange,
        fetchExchange,
        subscriptionExchange({
            forwardSubscription: (request) => subscriptionClient.request(request),
        }),
    ],
});
app.mount('#app')

