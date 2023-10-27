import { computed, reactive, watch } from "vue";

import type { UseSubscriptionResponse } from "@urql/vue";
import { project } from "./projection";

export interface HorizonEvent {
    altitude: number;
    azimuth: number;
    time: string;
}

export interface HorizonEventCollection {
    rise: HorizonEvent;
    set: HorizonEvent;
}

export interface Result {
    kind: string;
    location: {
        lat: number;
        lon: number;
    };
    events: {
        sun?: HorizonEventCollection;
        moon?: HorizonEventCollection;
    };
}

export interface Spot extends Result {
    id: string;
    selectedId?: number;
}

interface SpotsState {
    loading: boolean;
    spots: Spot[];
    subscription: UseSubscriptionResponse | undefined;
    nextSelectedId: number,
}

export const spots: SpotsState = reactive({
    loading: false,
    spots: [],
    subscription: undefined,
    nextSelectedId: 1,
});

// Connection state

export interface Connection {
    connected: boolean,
    apiVersion?: string,
    backendVersion?: string,
}

export const connection: Connection = reactive({
    connected: false,
});

// Search input state

interface Inputs {
    lat: number;
    lon: number;
    radius: number;
}

const defaultInputs: Inputs = {
    lat: 48.81872,
    lon: 9.58781,
    radius: 2000,
};
export const inputs: Inputs = reactive(
    loadObjectFromLocal("search.inputs", defaultInputs),
);
export const searchCenter = computed(() => project(inputs.lat, inputs.lon));

watch(inputs, (inputs) => {
    storeObjectLocal("search.inputs", inputs);
});

// Additional state for the query

export interface Time {
    time: string,
    timezone: string,
}

export const time: Time = reactive({
    time: (new Date()).toISOString(),
    timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
})

// Map state

interface MapState {
    center: number[];
    zoom: number;
}

const defaultMapState: MapState = {
    center: project(defaultInputs.lat, defaultInputs.lon),
    zoom: 14,
};
export const mapState: MapState = reactive(
    loadObjectFromLocal("map.state", defaultMapState),
);

let mapStateToStore = defaultMapState;
export function centerChanged(center: number[]) {
    mapStateToStore.center = center;
}
export function zoomChanged(zoom: number) {
    mapStateToStore.zoom = zoom;
}
export function storeMapState() {
    storeObjectLocal("map.state", mapStateToStore);
}

// Privacy popup state

const defaultShowPrivacyPopup = true;
export const showPrivacyPopup = loadBoolFromLocal(
    'popup.privacy.show',
    defaultShowPrivacyPopup,
);
export function dontShowPrivacyPopupAgain() {
    storeBoolLocal('popup.privacy.show', false);
}

// Utils

function storeObjectLocal(name: string, object: any) {
    const objectString = JSON.stringify(object);
    localStorage.setItem(name, objectString);
}
function storeBoolLocal(name: string, object: boolean) {
    storeObjectLocal(name, object);
}

function hasSameProps(a: any, b: any) {
    var aKeys = Object.keys(a).sort();
    var bKeys = Object.keys(b).sort();
    return JSON.stringify(aKeys) === JSON.stringify(bKeys);
}

function loadObjectFromLocal<T>(name: string, deflt: T): T {
    const objectString = localStorage.getItem(name) ?? "{}";
    const object = JSON.parse(objectString);
    if (hasSameProps(object, deflt)) {
        return object;
    } else {
        return deflt;
    }
}

function loadBoolFromLocal(name: string, deflt: boolean): boolean {
    const boolString = localStorage.getItem(name) ?? JSON.stringify(deflt);
    return JSON.parse(boolString);
}
