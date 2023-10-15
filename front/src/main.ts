import { createApp } from 'vue'
import OpenLayersMap from 'vue3-openlayers';
import 'vue3-openlayers/dist/vue3-openlayers.css';
import 'vue-final-modal/style.css'

import App from './App.vue';
import './index.css';
import { createVfm } from 'vue-final-modal'

export const version = "v0.0.5"

const app = createApp(App);
app.use(OpenLayersMap);
app.use(createVfm());
app.mount('#app');
