use std::net::Ipv6Addr;

use actix_web::{web, App, HttpServer};
use crud_rust::{app_config, env_config, model};

use mongodb::{
    bson::doc,
    options::{ClientOptions, IndexOptions, ServerApi, ServerApiVersion},
    Client, Database, IndexModel,
};

const ENV_PATH: &str = ".env_dev";

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    env_config::config(ENV_PATH)?;
    let db = init_db().await;
    let collection = db.collection::<model::User>(&std::env::var("COLL_NAME").unwrap());
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(collection.clone()))
            .service(web::scope("/user").configure(app_config::config_user))
    })
    .bind((Ipv6Addr::LOCALHOST, 8080))?
    .run()
    .await?;
    Ok(())
}

async fn init_db() -> Database {
    let uri = std::env::var("DATABASE_URL").unwrap();
    let database = std::env::var("DATABASE_NAME").unwrap();
    let coll = std::env::var("COLL_NAME").unwrap();

    let mut client_options = ClientOptions::parse(uri).await.unwrap();
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&database);
    // Ensure a unique index on the email field
    let collection = db.collection::<model::User>(&coll);
    let index_model = IndexModel::builder()
        .keys(doc! { "email": 1 }) // Create an ascending index on email
        .options(IndexOptions::builder().unique(true).build())
        .build();

    collection.create_index(index_model).await.unwrap();
    db
}
