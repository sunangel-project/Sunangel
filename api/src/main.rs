use std::time::Duration;

use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware::{self, Logger},
    web::{self, Data},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};

use juniper_actix::{graphql_handler, playground_handler, subscriptions::subscriptions_handler};
use juniper_graphql_ws::ConnectionConfig;
use log::info;

use crate::api::{schema, Context, Schema};

pub mod api;
pub mod messaging;
pub mod structs;

async fn playground() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", Some("/subscriptions")).await
}

async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let context = Context::new().await; // TODO: create context once in main, then create two
                                        // closures for graphql and subscriptions referencing the
                                        // same context

    graphql_handler(&schema, &context, req, payload).await
}

async fn subscriptions(
    req: HttpRequest,
    stream: web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let context = Context::new().await;
    let schema = schema.into_inner();
    let config = ConnectionConfig::new(context);
    // set the keep alive interval to 15 secs so that it doesn't timeout in playground
    // playground has a hard-coded timeout set to 20 secs
    let config = config.with_keep_alive_interval(Duration::from_secs(15));

    subscriptions_handler(req, stream, schema, config).await
}

#[actix_web::main]
async fn main() -> Result<(), async_nats::Error> {
    env_logger::init();

    info!("Server running on http://localhost:6660, playground: http://localhost:6660/playground");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema()))
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET"])
                    .allow_any_header()
                    //.allowed_headers(vec![header::ACCEPT, header::AUTHORIZATION])
                    //.allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::resource("/subscriptions").route(web::get().to(subscriptions)))
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql))
                    .route(web::get().to(graphql)),
            )
            .service(web::resource("/playground").route(web::get().to(playground)))
            .default_service(web::to(|| async {
                HttpResponse::Found()
                    .append_header((header::LOCATION, "/playground"))
                    .finish()
            }))
    })
    .bind("0.0.0.0:6660")?
    .run()
    .await?;

    Ok(())
}
