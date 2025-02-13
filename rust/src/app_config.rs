use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{web, HttpResponse, Responder};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};
use serde::Serialize;

use crate::{
    model::{CreateUser, User},
    password::Password,
};

pub fn config_user(cfg: &mut web::ServiceConfig) {
    let rate_limit_config = GovernorConfigBuilder::default()
        .seconds_per_request(60)
        .burst_size(3)
        .finish()
        .unwrap();

    cfg.route("/id/{id}", web::get().to(get_user_by_id))
        .route("/email/{email}", web::get().to(get_user_by_email))
        .service(
            web::scope("")
                .wrap(Governor::new(&rate_limit_config))
                .route("/create", web::post().to(create_user))
                .route("/id/{id}", web::delete().to(delete_user_by_id))
                .route("/id/{id}", web::put().to(update_user_by_id))
                .route("/email/{email}", web::delete().to(delete_user_by_email))
                .route("/email/{email}", web::put().to(update_user_by_email)),
        );
}

#[derive(Serialize)]
struct MyError {
    error: &'static str,
}

async fn create_user(
    user: web::Json<CreateUser>,
    collection: web::Data<Collection<User>>,
    hasher: web::Data<Password>,
) -> impl Responder {
    let hasher: &Password = hasher.as_ref();
    let user: User = (user.into_inner(), hasher).into();
    match collection.insert_one(&user).await {
        Ok(_) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::Conflict().json(MyError {
            error: "faild to create",
        }),
    }
}

async fn get_user_by_email(
    email: web::Path<String>,
    collection: web::Data<Collection<User>>,
) -> impl Responder {
    match collection
        .find_one(doc! { "email" : &email.into_inner()})
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::Ok().body("Not Found"),
        Err(_) => HttpResponse::InternalServerError().body("Error"),
    }
}
async fn get_user_by_id(
    id: web::Path<String>,
    collection: web::Data<Collection<User>>,
) -> impl Responder {
    let object_id = match ObjectId::parse_str(id.into_inner()) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID format"),
    };
    match collection.find_one(doc! { "_id": object_id }).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::Ok().body("Not Found"),
        Err(_) => HttpResponse::InternalServerError().body("Error"),
    }
}
async fn delete_user_by_email(
    email: web::Path<String>,
    collection: web::Data<Collection<User>>,
) -> impl Responder {
    match collection
        .delete_one(doc! { "email": email.into_inner() })
        .await
    {
        Ok(v) if v.deleted_count > 0 => HttpResponse::Ok().finish(),
        Ok(_) => HttpResponse::NotFound().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn delete_user_by_id(
    id: web::Path<String>,
    collection: web::Data<Collection<User>>,
) -> impl Responder {
    let object_id = match ObjectId::parse_str(id.into_inner()) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID format"),
    };
    match collection.delete_one(doc! { "_id": object_id }).await {
        Ok(v) if v.deleted_count > 0 => HttpResponse::Ok().finish(),
        Ok(_) => HttpResponse::NotFound().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
async fn update_user_by_email(
    email: web::Path<String>,
    collection: web::Data<Collection<User>>,
    hasher: web::Data<Password>,
) -> impl Responder {
    HttpResponse::Ok().body(format!("Update Email: {:?}", email))
}
async fn update_user_by_id(
    user: web::Path<String>,
    collection: web::Data<Collection<User>>,
    // hasher: web::Data<Password>,
) -> impl Responder {
    HttpResponse::Ok().body(format!("Update Id: {:?}", user))
}
