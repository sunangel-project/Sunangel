import { computed, reactive, watch } from "vue";

import type { UseSubscriptionResponse } from "@urql/vue";
import LatLon from 'geodesy/latlon-ellipsoidal-vincenty.js';
import type { ObjectEvent } from "ol/Object";

import { project, invertProject, type Coordinates } from "./projection";

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
    location: Coordinates;
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
    nextSelectedId: number;
}

export const spots: SpotsState = reactive({
    loading: false,
    spots: [],
    subscription: undefined,
    nextSelectedId: 1,
});

// Connection state

export interface Connection {
    connected: boolean;
    apiVersion?: string;
    backendVersion?: string;
}

export const connection: Connection = reactive({
    connected: false,
});

// Interface state

interface InterfaceControls {
    followView: boolean;
}

const defaultInterfaceControls: InterfaceControls = {
    followView: false,
};

export const interfaceControls: InterfaceControls = reactive(
    loadObjectFromLocal("interface.controls", defaultInterfaceControls),
);

watch(interfaceControls, (interfaceControls) => {
    storeObjectLocal("interface.controls", interfaceControls);
});

export const areaTooLarge = computed(() => {
    const a = new LatLon(searchArea.lowerLeft.lat, searchArea.lowerLeft.lon);
    const b = new LatLon(searchArea.upperRight.lat, searchArea.upperRight.lon);

    return a.distanceTo(b) > 10_000;
})

// Map state

interface MapState {
    center: number[];
    zoom: number;
}

const defaultCenterCoordinates = { lat: 48.818, lon: 9.587 };
const defaultCenter = project(defaultCenterCoordinates);
const defaultMapState: MapState = {
    center: defaultCenter,
    zoom: 14,
};
export const mapState: MapState = reactive(
    loadObjectFromLocal("map.state", defaultMapState),
);

let mapStateToStore = defaultMapState;
export function centerChanged(event: ObjectEvent) {
    const center = event.target.getCenter();
    mapStateToStore.center = center;

    const extent: number[] = event.target.getViewStateAndExtent().extent;
    searchArea.lowerLeft = invertProject(extent.slice(0, 2));
    searchArea.upperRight = invertProject(extent.slice(2, 4));
}
export function zoomChanged(event: ObjectEvent) {
    mapStateToStore.zoom = event.target.getZoom();
}
export function storeMapState() {
    storeObjectLocal("map.state", mapStateToStore);
}

// Additional state for the query

export interface SearchArea {
    lowerLeft: Coordinates;
    upperRight: Coordinates;
}

export const searchArea = reactive({
    lowerLeft: defaultCenterCoordinates,
    upperRight: defaultCenterCoordinates,
})

export interface Time {
    time: string;
    timezone: string;
}

export const time: Time = reactive({
    time: new Date().toISOString(),
    timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
});

// Privacy popup state

const defaultShowPrivacyPopup = true;
export const showPrivacyPopup = loadBoolFromLocal(
    "popup.privacy.show",
    defaultShowPrivacyPopup,
);
export function dontShowPrivacyPopupAgain() {
    storeBoolLocal("popup.privacy.show", false);
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
