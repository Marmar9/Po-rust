use std::{fs::File, io::BufReader};
// use actix_files::Files;
use actix_web::{
    http::header::ContentType, middleware, web, App, HttpRequest, HttpResponse, HttpServer,
};
use connection_pool::RedisConnectionPool;
use log::debug;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};


mod routes;
mod utils;
mod tests;

use  routes::get_routes::{user, users};
use routes::edit_routes::{user_patch , user_delete , user_post , user_put};

use crate::utils::user_functions::User;
mod connection_pool;
/// simple handle
// #[get("/")]
async fn index(req: HttpRequest) -> HttpResponse {
    debug!("{req:?}");

    HttpResponse::Ok().content_type(ContentType::html()).body(
        "<!DOCTYPE html><html><body>\
            <p>Welcome to your TLS-secured homepage!</p>\
        </body></html>",
    )
}
// #[get("/users")]
// async fn users(req: HttpRequest) -> HttpResponse {
//     debug!("{req:?}");

//     HttpResponse::Ok().content_type(ContentType::html()).body(
//         "<!DOCTYPE html><html><body>\
//             <p>Welcome to your TLS-secured homepage!</p>\
//         </body></html>",
//     )
// }
struct AppState {
    connection_pool: RedisConnectionPool,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Redis
    let connection_pool = connection_pool::RedisConnectionPool::new(10).await;

    // let mut connection = connection_pool.get_connection().await;

    // let  result: String = connection.set("aaa", "bbb".to_string()).await.unwrap();

    
    // Server config
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let config = load_rustls_config();
    log::info!("starting HTTPS server at https://localhost:443");

    HttpServer::new(move|| {
        App::new()
        .app_data(web::Data::new(AppState {
            connection_pool: connection_pool.clone(),
        }))
            .wrap(middleware::Logger::default())

            .service(web::resource("/").to(index))  
            .service(web::resource("/users").route(web::get().to(users)))
            .service(web::resource("/users/{id}")
            .route(web::get().to(user))
            .route(web::patch().to(user_patch))
            .route(web::post().to(user_post))
            .route(web::put().to(user_put))
            .route(web::delete().to(user_delete)))

    })
    // .bind_rustls_021("127.0.0.1:443", config)?
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn load_rustls_config() -> rustls::ServerConfig {


    
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}



