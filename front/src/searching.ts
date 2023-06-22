import {
    cacheExchange,
    Client,
    fetchExchange,
    gql,
    subscriptionExchange,
} from "@urql/vue";
import { SubscriptionClient } from "subscriptions-transport-ws";

interface HorizonEvent {
    altitude: number;
    azimuth: number;
    time: string;
}

interface Result {
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

// TODO: handle connection fail
const subscriptionClient = new SubscriptionClient(
    "ws://localhost:6660/subscriptions",
    { reconnect: false },
);

const client = new Client({
    url: "http://localhost:6660/graphql",
    exchanges: [
        cacheExchange,
        fetchExchange,
        subscriptionExchange({
            forwardSubscription: (request) => subscriptionClient.request(request),
        }),
    ],
});

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

function search(lat: number, lon: number, radius: number) {
    const unsubscribe = client
        .subscription(
            query,
            {
                lat: lat,
                lon: lon,
                radius: radius,
            },
        )
        .subscribe(raw => {
            let result = raw.data?.spots.spot; // TODO: test for spots, spot, data?
            console.log(result);
        });
}

export default {
    search: search,
};
