import { createApp } from 'vue'
import OpenLayersMap from 'vue3-openlayers';
import 'vue3-openlayers/dist/vue3-openlayers.css';
import 'vue-final-modal/style.css'
import urql, { cacheExchange, fetchExchange, subscriptionExchange } from '@urql/vue';


import App from './App.vue';
import './assets/main.css';
import { SubscriptionClient } from 'subscriptions-transport-ws';
import { createVfm } from 'vue-final-modal'

let protocol = "ws";
let apiHost = "localhost";
if (process.env.NODE_ENV == "production") {
    protocol = "wss";
    apiHost = "sunnapi.cloudsftp.de";
}

// TODO: handle connection fail
// TODO: url as extra variable
const subscriptionClient = new SubscriptionClient(
    `${protocol}://${apiHost}:6660/subscriptions`,
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
app.use(createVfm());
app.mount('#app');
