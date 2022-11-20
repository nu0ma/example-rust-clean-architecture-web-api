use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{
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
