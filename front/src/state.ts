import { computed, reactive, ref } from "vue";

import type { Result } from "./searching";
import type { UseSubscriptionResponse } from "@urql/vue";
import { project } from "./projection";

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

interface Inputs {
    lat: number,
    lon: number,
    radius: number,
}

// TODO: store and load from cookies

export const inputs: Inputs = reactive({
    lat: 48.81872,
    lon: 9.58781,
    radius: 2000,
})

export const searchCenter = computed(() => project(inputs.lat, inputs.lon));
export const searchRadius = computed(() => inputs.radius);

// Map state

export const mapCenter = computed(() => project(inputs.lat, inputs.lon));
export const mapZoom = ref(14);

