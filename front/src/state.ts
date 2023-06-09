import { computed, reactive, watch } from "vue";

import type { Result } from "./searching";
import type { UseSubscriptionResponse } from "@urql/vue";
import { project } from "./projection";

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

const defaultInputs: Inputs = {
    lat: 48.81872,
    lon: 9.58781,
    radius: 2000,
}
export const inputs: Inputs = reactive(
    loadObjectFromLocal('search.inputs', defaultInputs),
)
export const searchCenter = computed(() => project(inputs.lat, inputs.lon));

watch(inputs, (inputs) => {
    storeObjectLocal('search.inputs', inputs);
});

// Map state

interface MapState {
    center: number[],
    zoom: number,
}

const defaultMapState: MapState = {
    center: project(defaultInputs.lat, defaultInputs.lon),
    zoom: 14,
};
export const mapState: MapState = reactive(
    loadObjectFromLocal('map.state', defaultMapState),
);

let mapStateToStore = defaultMapState;
export function centerChanged(center: number[]) {
    mapStateToStore.center = center;
}
export function zoomChanged(zoom: number) {
    mapStateToStore.zoom = zoom;
}
export function storeMapState() {
    storeObjectLocal('map.state', mapStateToStore);
}

// Utils

function storeObjectLocal(name: string, object: any) {
    const objectString = JSON.stringify(object);
    localStorage.setItem(name, objectString);
}

function hasSameProps(a: any, b: any) {
    var aKeys = Object.keys(a).sort();
    var bKeys = Object.keys(b).sort();
    return JSON.stringify(aKeys) === JSON.stringify(bKeys);
}

function loadObjectFromLocal<T>(name: string, deflt: T): T {
    const objectString = localStorage.getItem(name) ?? '{}';
    const object = JSON.parse(objectString);
    if (hasSameProps(object, deflt)) {
        return object;
    } else {
        return deflt;
    }
}
