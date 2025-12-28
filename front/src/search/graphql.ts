import {
    cacheExchange,
    fetchExchange,
    subscriptionExchange,
    Client,
    provideClient,
    gql,
    useQuery,
    mapExchange,
} from "@urql/vue";
import {
    createClient as createWSClient,
    type SubscribePayload,
} from "graphql-ws";
import { connection, spots } from "./state";

import { useModal } from "vue-final-modal";
import Popup from "./components/Popup.vue";
import type { GraphQLError } from "@0no-co/graphql.web";

function displayError(message: string) {
    const { open } = useModal({
        component: Popup,
        attrs: {
            title: "Error",
            message,
        },
    });
    open();
}

function displayConnectionError() {
    displayError("Couldn't connect to the backend... Please try again later");
}

function displayIntenalServerError(errors: GraphQLError[]) {
    let message = "Internal Server Error";
    if (errors.length > 0) {
        const extensions = JSON.parse(errors[0]!.extensions.toString());
        message = extensions.reason;
    }
    displayError(message);
}

export function setupGraphQLClient(): void {
    let protocol = "ws";
    let httpProtocol = "http";
    let apiHost = "localhost";
    let port = "6660";
    //apiHost = "192.168.2.123";
    if (process.env.NODE_ENV == "production") {
        protocol = "wss";
        httpProtocol = "https";
        apiHost = "sunnapi.cloudsftp.de";
        port = "443";
    }

    const wsClient = createWSClient({
        url: `${protocol}://${apiHost}:${port}/subscriptions`,
    });

    const subExchange = subscriptionExchange({
        forwardSubscription(operation) {
            return {
                subscribe: (sink) => {
                    const dispose = wsClient.subscribe(
                        operation as SubscribePayload,
                        sink,
                    );
                    return {
                        unsubscribe: dispose,
                    };
                },
            };
        },
    });

    const client = new Client({
        url: `${httpProtocol}://${apiHost}:${port}/graphql`,
        exchanges: [
            mapExchange({
                onError: (error) => {
                    if (error.networkError) {
                        console.log(error.networkError);
                        connection.connected = false;
                        displayConnectionError();
                    } else if (error.graphQLErrors) {
                        console.log(error.graphQLErrors);
                        spots.loading = false;
                        displayIntenalServerError(error.graphQLErrors);
                    }
                },
            }),
            cacheExchange,
            fetchExchange,
            subExchange,
        ],
    });

    provideClient(client);
}

export function fetchBackendVersions() {
    let query = gql`
        query versions {
            apiVersion
            backendVersion
        }
    `;

    useQuery({ query }).then((result) => {
        if (result.error.value) {
            return;
        }

        connection.connected = true;
        connection.apiVersion = result.data.value.apiVersion;
        connection.backendVersion = result.data.value.backendVersion;
    });
}
