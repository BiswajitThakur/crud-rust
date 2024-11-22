use std::str::FromStr;

/*
use actix_web::{get, web, App, HttpServer};

struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    format!("Hello {}", &data.app_name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: "My App".to_string(),
            }))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
*/
use crud_rust::DataBase;
use uuid::Uuid;

fn main() {
    let dbs = DataBase::open("/home/eagle/development/crud-rust/database/dbs.sqlite").unwrap();
    for (index, user) in dbs.iter_users().enumerate() {
        println!("{}: {}", index, user);
    }
    let id = Uuid::from_str("9ed710e5-7e0e-406e-b57a-c9297531cd07").unwrap();
    if let Some(user) = dbs.find_by_id(id).unwrap() {
        println!("User found: {:?}", user);
    }
    if let Some(user) = dbs.find_by_email("bt@gmail.com").unwrap() {
        println!("User found: {:?}", user);
    }
}
