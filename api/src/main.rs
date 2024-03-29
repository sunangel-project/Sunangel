use std::time::Duration;

use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware::{self, Logger},
    web::{self, Data},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};

use juniper_actix::{graphql_handler, playground_handler, subscriptions};
use juniper_graphql_ws::ConnectionConfig;
use log::info;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

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
    let context = Context::new().await;
    graphql_handler(&schema, &context, req, payload).await
}

async fn subscriptions(
    req: HttpRequest,
    stream: web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let schema = schema.into_inner();
    let context = Context::new().await;
    let config = ConnectionConfig::new(context);
    // set the keep alive interval to 15 secs so that it doesn't timeout in playground
    // playground has a hard-coded timeout set to 20 secs
    let config = config.with_keep_alive_interval(Duration::from_secs(15));

    subscriptions::ws_handler(req, stream, schema, config).await
}

#[actix_web::main]
async fn main() -> Result<(), async_nats::Error> {
    env_logger::init();

    let context = Context::new().await;

    // Create certificates for test purposes:openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'
    let mut acceptor_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    acceptor_builder.set_private_key_file("key.pem", SslFiletype::PEM)?;
    acceptor_builder.set_certificate_chain_file("cert.pem")?;

    info!("Server running on http://localhost:6660, playground: http://localhost:6660/playground");

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema()))
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(["POST", "GET"])
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
    });

    server = if context.production {
        info!("Detected production environment - enabling SSL");
        server.bind_openssl("0.0.0.0:6660", acceptor_builder)?
    } else {
        info!("Non-production environment - SSL disabled");
        server.bind("0.0.0.0:6660")?
    };

    server.run().await?;

    Ok(())
}
