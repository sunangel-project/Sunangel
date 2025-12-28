import { ref } from "vue";
import proj4 from "proj4";

export interface Coordinates {
    lat: number,
    lon: number,
}

const projectionIdentifier = "EPSG:3857";

export const projection = ref(projectionIdentifier);

export function project(coordinates: Coordinates): number[] {
    return proj4(
        projectionIdentifier,
        [coordinates.lon, coordinates.lat],
    );
}

export function invertProject(input: number[]): Coordinates {
    let out = proj4(projectionIdentifier, "WGS84", input);
    return {
        lat: out[1]!,
        lon: out[0]!,
    };
}
