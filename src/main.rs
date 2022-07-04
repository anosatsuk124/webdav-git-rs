mod error;
mod repo;

use crate::repo::Repo;
use actix_web::{get, web, App, HttpServer, Responder};
use actix_web_httpauth::headers::authorization;
use log::info;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};
use webdav_handler::actix;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let repo1 = Repo::new("./repos/anosatsuk124", "anosatsuk124").unwrap();
    let port = match std::env::var("PORT")
        .unwrap_or("8000".to_string())
        .parse::<u16>()
    {
        Ok(p) => p,
        Err(_) => panic!("unexpect number"),
    };
    let address = match std::env::var("ADDRESS") {
        Ok(addr) => addr,
        Err(_) => "127.0.0.1".to_string(),
    };

    info!("Listen: http://{}:{}", &address, &port);
    HttpServer::new(|| App::new().service(greet))
        .bind((address, port))?
        .run()
        .await
}
