#![allow(unused_imports)]
use actix_web::{
    get, patch, post,
    web::{Data, Json, Path},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use db::database::Database;
use error::user_error::UserError;
use models::user::User;
use models::user_request::UserRequest;
use models::uuid::Uuid;
use std::result::Result;
use uuid;
use validator::Validate;

mod db;
mod error;
mod models;

#[get("/user")]
async fn get_user(data: Data<Database>) -> impl Responder {
    let users = data.get_all_users().await;
    match users {
        Some(valid_users) => Ok(Json(valid_users)),
        None => Err(UserError::NoUserFound),
    }
}

#[post("/user")]
async fn post_user(body: Json<UserRequest>, db: Data<Database>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let name = body.name.clone();
            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_user = db.add_user(User::new(String::from(new_uuid), name)).await;

            match new_user {
                Some(user) => HttpResponse::Ok().body(format!("User created is {:?}", user)),
                None => HttpResponse::Ok().body("Error creating user"),
            };
        }
        Err(e) => {
            println!("User is invalid: {:?}", e);
            return HttpResponse::Ok().body("User is invalid");
        }
    }
    HttpResponse::Ok().body("User created")
}

#[patch("/user/{uuid}")]
async fn patch_user(uuid: Path<Uuid>) -> impl Responder {
    let uuid = uuid.into_inner().id;
    HttpResponse::Ok().body(format!("User patched {uuid}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init()
        .await
        .expect("error connecting to the database");
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_user)
            .service(post_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
