import { gql, useSubscription } from '@urql/vue';
import { v4 as uuidv4 } from 'uuid';
import { inputs, spots, type Spot, type Result } from "./state";

export function search() {
    if (spots.loading) { // TODO: set true here and set false when receiving responses
        return // TODO: warning
    }

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
subscription spot($lat: Float!, $lon: Float!, $radius: Int!) {
  spots(query: { location: { lat: $lat, lon: $lon }, radius: $radius }) {
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
            variables: inputs,
            pause: true,
        },
        (_, result) => {
            if (typeof result === "object") { // TODO: type safety!
                const spot = spotFromResult(result.spots.spot);
                spots.spots.push(spot);
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
