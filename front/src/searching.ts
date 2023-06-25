import { gql, useSubscription } from '@urql/vue';
import { inputs, spots } from "./state";

interface HorizonEvent {
    altitude: number;
    azimuth: number;
    time: string;
}

export interface Result {
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
            if (typeof s === "object") {
                spots.spots.push(s.spots.spot)
            } else {
                console.log('was not correct type')
            }
        },
    );
}
