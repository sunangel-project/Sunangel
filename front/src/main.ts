import { createApp } from 'vue';
import { createRouter, createWebHashHistory } from 'vue-router';

import OpenLayersMap from 'vue3-openlayers';
import 'vue3-openlayers/dist/vue3-openlayers.css';
import 'vue-final-modal/style.css'

import App from './App.vue';
import './index.css';
import { createVfm } from 'vue-final-modal'

const app = createApp(App);

import Search from './search/Search.vue';
const About = app.component('about', { template: '<div>About</div>' });

const routes = [
    { path: '/', component: Search },
    { path: '/about', component: About },
]

const router = createRouter({
    history: createWebHashHistory(),
    routes,
})
app.use(router);


export const version = "v0.0.5"

app.use(OpenLayersMap);
app.use(createVfm());
app.mount('#app');
