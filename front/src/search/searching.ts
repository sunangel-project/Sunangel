import { gql, useSubscription } from '@urql/vue';
import { v4 as uuidv4 } from 'uuid';
import { inputs, time, spots, type Spot, type Result } from "./state";
import { toRefs } from 'vue';

export function search() {
    if (spots.loading) { return; }
    spots.loading = true;

    time.time = (new Date()).toISOString();

    spots.spots = [];
    spots.subscription?.executeSubscription();
}

export function setupSpotsSubscription() {
    let horizonEventsCollectionQuery = `
rise {
  time
  altitude
  azimuth
}
set {
  time
  altitude
  azimuth
}
`
    let query = gql`
subscription spot($time: DateTime!, $timezone: TimeZone!, $lat: Float!, $lon: Float!, $radius: Int!) {
  spots(query: { time: $time, timezone: $timezone, location: { lat: $lat, lon: $lon }, radius: $radius }) {
    status
    spot {
      location {
        lat
        lon
      }
      kind
      events {
        sun {
          ${horizonEventsCollectionQuery}
        }
        moon {
          ${horizonEventsCollectionQuery}
        }
      }
    }
  }
}
`;

    spots.subscription = useSubscription(
        {
            query: query,
            variables: {
                ...toRefs(inputs),
                ...toRefs(time),
            },
            pause: true,
        },
        (_, result) => {
            if (typeof result === "object") { // TODO: type safety!
                const spot = spotFromResult(result.spots.spot);
                spots.spots.push(spot);

                if (result.spots.status === "FINISHED") {
                    spots.loading = false;
                }
            } else {
                console.log('was not correct type');
            }
        },
    );
}

function spotFromResult(result: Result): Spot {
    const id = uuidv4();
    return { ...result, id };
}
