use std::{collections::HashMap, sync::Arc};

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder};
use serde::Deserialize;

use crate::finder::{FindResult, Finder};

pub mod args;

#[derive(Deserialize)]
struct FindItemsReq {
    items: Vec<String>,
}

async fn service_find_items(
    req_data: web::Json<FindItemsReq>,
    finders: web::Data<Arc<Vec<Box<dyn Finder>>>>,
) -> impl Responder {
    web::Json({})
}

pub async fn start_server(
    bind: &str,
    port: u16,
    allow_cors: bool,
    services: HashMap<String, Arc<Vec<Box<dyn Finder>>>>,
) -> std::io::Result<()> {
    HttpServer::new(move || {
        let mut cors = Cors::default();
        if allow_cors {
            cors = cors
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .supports_credentials()
                .max_age(3600);
        }
        let mut app = App::new().wrap(cors);

        for (route, service) in services.iter() {
            let service_data = web::Data::new(service.clone());
            app = app.service(
                web::resource(route)
                    .app_data(service_data)
                    .route(web::post().to(service_find_items)),
            );
        }
        app
    })
    .workers(2)
    .bind((bind, port))?
    .run()
    .await
}
