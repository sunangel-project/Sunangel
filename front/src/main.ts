import { createApp } from 'vue';
import { createRouter, createWebHashHistory } from 'vue-router';

import OpenLayersMap from 'vue3-openlayers';
import 'vue3-openlayers/dist/vue3-openlayers.css';
import 'vue-final-modal/style.css'

import './index.css';
import { createVfm } from 'vue-final-modal'

export const version = "v0.1.0"

import App from './App.vue';
import Search from './search/Search.vue';
import Privacy from './privacy/Privacy.vue';
import Datenschutz from './privacy/Datenschutz.vue';

const routes = [
    { path: '/', component: Search },
    { path: '/privacy', component: Privacy },
    { path: '/datenschutz', component: Datenschutz },
]

const router = createRouter({
    history: createWebHashHistory(),
    routes,
})

const app = createApp(App);
app.use(router);
app.use(OpenLayersMap);
app.use(createVfm());
app.mount('#app');
