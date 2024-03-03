import { ref } from "vue";
import proj4 from "proj4";

const projectionIdentifier = "EPSG:3857";

export const projection = ref(projectionIdentifier);
export function project(lat: number, lon: number): number[] {
    if (typeof lat === 'number' && typeof lon === 'number') {
        return proj4(projectionIdentifier, [lon, lat]);
    } else {
        return [0, 0]; // TODO: try to parse strings if inputs are strings?
    }
}
export function invertProject(input: number[]): { lat: number, lon: number } {
    let out = proj4(projectionIdentifier, "WGS84", input);
    return {
        lat: out[1],
        lon: out[0],
    };
}
