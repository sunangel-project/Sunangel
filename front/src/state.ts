import { reactive } from "vue";

import type { Result } from "./searching";
import type { UseSubscriptionResponse } from "@urql/vue";

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

export const inputs: Inputs = reactive({
    lat: 48.81872,
    lon: 9.58781,
    radius: 2000,
})
