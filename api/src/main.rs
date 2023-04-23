use std::sync::Arc;

use futures::FutureExt;
use juniper_graphql_ws::ConnectionConfig;
use juniper_warp::{playground_filter, subscriptions::serve_graphql_ws};
use warp::Filter;

use crate::api::{schema, Context};

pub mod api;
pub mod messaging;
pub mod structs;

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    env_logger::init();

    let log = warp::log("warp_server");

    let context = Context::new().await;
    let context2 = context.clone();

    let state = warp::any().map(move || context.clone());
    let graphql_filter = juniper_warp::make_graphql_filter(schema(), state.boxed());

    let root_node = Arc::new(schema());

    println!(
        "Server running on http://localhost:6660,\nplayground: http://localhost:6660/playground"
    );

    let routes = (warp::path("subscriptions")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let root_node = root_node.clone();
            let ctx = context2.clone();

            ws.on_upgrade(move |websocket| async move {
                serve_graphql_ws(websocket, root_node, ConnectionConfig::new(ctx))
                    .map(|r| {
                        if let Err(err) = r {
                            println!("Websocket error: {err}")
                        }
                    })
                    .await
            })
        }))
    .map(|reply| {
        // TODO #584: remove this workaround
        // still needed? https://github.com/graphql-rust/juniper/issues/584
        warp::reply::with_header(reply, "Sec-WebSocket-Protocol", "graphql-ws")
    })
    .or(warp::post().and(warp::path("graphql")).and(graphql_filter))
    .or(warp::get()
        .and(warp::path("playground"))
        .and(playground_filter("/graphql", Some("/subscriptions"))))
    .with(log);

    warp::serve(routes).run(([127, 0, 0, 1], 6660)).await;

    Ok(())
}
