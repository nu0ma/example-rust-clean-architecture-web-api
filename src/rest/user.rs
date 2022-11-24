use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{
    domain::user::User,
    driver::{self},
    gateway::user::UserGateway,
    usecase,
};

#[derive(Deserialize)]
struct Info {
    id: i32,
}

#[get("/user/{id}")]
pub async fn get_user(path_params: web::Path<Info>) -> impl Responder {
    let response = usecase::user::get_user(
        UserGateway {
            db_driver: driver::db_driver::DbDriver {},
        },
        path_params.id,
    )
    .await;
    match response {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/user")]
pub async fn create_user(user: web::Json<User>) -> impl Responder {
    let response = usecase::user::create_user(
        UserGateway {
            db_driver: driver::db_driver::DbDriver {},
        },
        user.0,
    )
    .await;

    match response {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
