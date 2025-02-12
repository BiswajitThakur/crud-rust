use mongodb::{
    bson::doc,
    options::{ClientOptions, IndexOptions, ServerApi, ServerApiVersion},
    Client, Database, IndexModel,
};

use crate::model;

pub async fn init_db() -> Database {
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
