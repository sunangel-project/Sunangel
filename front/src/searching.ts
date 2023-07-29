import { gql, useSubscription } from '@urql/vue';
import { v4 as uuidv4 } from 'uuid';
import { inputs, spots, type Spot, type HorizonEvent } from "./state";

interface Result {
    selected: boolean;
    kind: string;
    location: {
        lat: number;
        lon: number;
    };
    events: {
        sun: {
            rise: HorizonEvent;
            set: HorizonEvent;
        };
    };
}

export function search() {
    if (spots.loading) { // TODO: set true here and set false when receiving responses
        return // TODO: warning
    }

    spots.spots = []
    spots.subscription?.executeSubscription()
}

export function setupSpotsSubscription() {
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
        (_, s) => {
            if (typeof s === "object") { // TODO: type safety!
                spots.spots.push(spotFromResult(s.spots.spot));
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
