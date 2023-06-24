import {
    cacheExchange,
    Client,
    fetchExchange,
    gql,
    provideClient,
    subscriptionExchange,
    useQuery,
} from "@urql/vue";
import { SubscriptionClient } from "subscriptions-transport-ws";
import { provide } from "vue";

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



function search(lat: number, lon: number, radius: number) {
    /*
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
        */
}

export default {
    search: search,
};
