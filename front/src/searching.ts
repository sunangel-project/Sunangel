import { gql, Client, cacheExchange, fetchExchange, subscriptionExchange } from '@urql/vue';
import { SubscriptionClient } from 'subscriptions-transport-ws';

// TODO: handle connection fail
const subscriptionClient = new SubscriptionClient('ws://localhost:6660/subscriptions', { reconnect: false });

const client = new Client({
    url: 'http://localhost:6660/graphql',
    exchanges: [
        cacheExchange,
        fetchExchange,
        subscriptionExchange({
            forwardSubscription: request => subscriptionClient.request(request),
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
`

function search(lat: number, lon: number, radius: number) {
    const unsubscribe = client
        .subscription(
            query,
            {
                lat: lat,
                lon: lon,
                radius: radius
            }
        )
        .subscribe(result => {
            console.log(result);
        })
}

export default {
    search: search
}
