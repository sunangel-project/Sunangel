import { ref } from "vue";
import proj4 from "proj4";

const projectionIdentifier = "EPSG:3857";

export const projection = ref(projectionIdentifier);
export function project(lat: number, lon: number): number[] {
    return proj4(projectionIdentifier, [lon, lat])
}
