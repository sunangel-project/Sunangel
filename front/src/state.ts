import { computed, onMounted, reactive, ref, watch } from "vue";

import type { Result } from "./searching";
import type { UseSubscriptionResponse } from "@urql/vue";
import { invertProject, project } from "./projection";
import { serialize } from "v8";

// TODO: all data top level

interface SpotsState {
    loading: Boolean,
    spots: Result[],
    subscription: UseSubscriptionResponse | undefined,
}

export const spots: SpotsState = reactive({
    loading: false,
    spots: [],
    subscription: undefined,
});

// Search input state

interface Inputs {
    lat: number,
    lon: number,
    radius: number,
}

export const inputs: Inputs = reactive({
    lat: 48.81872,
    lon: 9.58781,
    radius: 2000,
})

watch(inputs, (newVal) => {
    localStorage.setItem('search.inputs', JSON.stringify(newVal));
});

export const searchCenter = computed(() => project(inputs.lat, inputs.lon));
export const searchRadius = computed(() => inputs.radius);

// Map state

const mapState = reactive({
    center: project(inputs.lat, inputs.lon),
    zoom: 14,
});

export const mapCenter = computed(() => mapState.center);
export const mapZoom = computed(() => mapState.zoom);

export function storeMapCenter(center: number[]) {
    localStorage.setItem('map.state.center', JSON.stringify(center));
}
export function storeMapZoom(zoom: number) {
    localStorage.setItem('map.state.zoom', zoom.toString());
}

// Load data from local storage

export function restoreState() { // TODO: sanity checks
    // Inputs
    const inputsString = localStorage.getItem('search.inputs');
    if (inputsString != null) {
        const inputsValues = JSON.parse(inputsString);
        inputs.lat = inputsValues.lat;
        inputs.lon = inputsValues.lon;
        inputs.radius = inputsValues.radius;
    }

    // Map
    const centerString = localStorage.getItem('map.state.center');
    if (centerString != null) {
        const center = JSON.parse(centerString);
        mapState.center = center;
    }
    const zoomString = localStorage.getItem('map.state.zoom')
    if (zoomString != null) {
        mapState.zoom = parseFloat(zoomString);
    }
}
