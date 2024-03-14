mod external_calls;

use crate::network::external_calls::{is_alive, doors_of_durin};

use std::collections::HashMap;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Settings {
    domain: String,
    port: u16,
}

pub async fn external_connection() -> std::io::Result<()> {
    let settings = get_settings();
    
    println!("Starting server...");
    println!("Listening on: http://{}:{}", settings.domain, settings.port);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .service(is_alive)
            .service(doors_of_durin)
    })
    .bind((settings.domain, settings.port))?
    .run()
    .await
}

fn get_settings() -> Settings {
    let settings = Config::builder()
        .add_source(File::with_name("Development"))
        .build()
        .unwrap()
        .try_deserialize::<HashMap<String, HashMap<String, String>>>()
        .unwrap();
    
    let network = settings.get("network").unwrap();

    let domain: String = network.get("domain").unwrap().clone();
    let port: u16 = network.get("port").unwrap().parse().unwrap();

    Settings { domain, port }
}