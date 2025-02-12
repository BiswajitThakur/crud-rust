use std::{net::Ipv6Addr, num::NonZeroU32};

use actix_files::NamedFile;
use actix_web::{get, web, App, HttpServer, Responder};
use crud_rust::{app_config, db, env_config, model, password::Password};

const ENV_PATH: &str = ".env_dev";
const SALT: [u8; 16] = *b"\xb8\xb3\x9a\xe3\x18\x1f\xe1\xe5\x42\x1e\xbf\xd2\x8c\x66\x43\xc8";

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("frontend/index.html").await
}

#[get("/login")]
async fn login() -> impl Responder {
    NamedFile::open_async("frontend/login.html").await
}

#[get("/register")]
async fn register() -> impl Responder {
    NamedFile::open_async("frontend/register.html").await
}

#[get("/style.css")]
async fn style() -> impl Responder {
    NamedFile::open_async("frontend/style.css").await
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    env_config::config(ENV_PATH)?;

    let db = db::init_db().await;
    let collection = db.collection::<model::User>(&std::env::var("COLL_NAME").unwrap());

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(login)
            .service(register)
            .service(style)
            .service(
                web::scope("/user")
                    .app_data(web::Data::new(collection.clone()))
                    .app_data(web::Data::new(Password::new(
                        NonZeroU32::new(10321).unwrap(),
                        SALT,
                    )))
                    .configure(app_config::config_user),
            )
    })
    .bind((Ipv6Addr::LOCALHOST, 8080))?
    .run()
    .await?;
    Ok(())
}
