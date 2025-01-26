use actix_web::{web, HttpResponse, Responder};
use mongodb::{
    bson::{doc, oid::ObjectId, Bson, Document},
    Collection,
};
use serde::Serialize;

use crate::model::{CreateUser, User};

pub fn config_user(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/create").route(web::post().to(create_user)))
        .service(
            web::resource("/id/{id}")
                .route(web::get().to(get_user_by_id))
                .route(web::delete().to(delete_user_by_id))
                .route(web::put().to(update_user_by_id)),
        )
        .service(
            web::resource("/email/{email}")
                .route(web::get().to(get_user_by_email))
                .route(web::delete().to(delete_user_by_email))
                .route(web::put().to(update_user_by_email)),
        );
}

#[derive(Serialize)]
struct MyError {
    error: &'static str,
}

async fn create_user(
    user: web::Json<CreateUser>,
    collection: web::Data<Collection<User>>,
) -> impl Responder {
    let user: User = user.into_inner().into();
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
    user: web::Path<String>,
    collection: web::Data<Collection<User>>,
) -> impl Responder {
    HttpResponse::Ok().body(format!("Delete Id: {:?}", user))
}
async fn update_user_by_email(
    user: web::Path<String>,
    collection: web::Data<Collection<User>>,
) -> impl Responder {
    HttpResponse::Ok().body(format!("Update Email: {:?}", user))
}
async fn update_user_by_id(
    user: web::Path<String>,
    collection: web::Data<Collection<User>>,
) -> impl Responder {
    HttpResponse::Ok().body(format!("Update Id: {:?}", user))
}
