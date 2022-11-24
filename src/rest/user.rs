use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{
    domain::user::User,
    driver::{self, db_driver::DbDriver},
    gateway::user::UserGateway,
    usecase,
};

pub struct AppState {
    pub db_driver: DbDriver,
}

#[derive(Deserialize)]
struct Info {
    id: i32,
}

#[get("/user/{id}")]
pub async fn get_user(path_params: web::Path<Info>, data: web::Data<AppState>) -> impl Responder {
    let response = usecase::user::get_user(
        UserGateway {
            db_driver: data.db_driver,
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
pub async fn create_user(user: web::Json<User>, data: web::Data<AppState>) -> impl Responder {
    let response = usecase::user::create_user(
        UserGateway {
            db_driver: data.db_driver,
        },
        user.0,
    )
    .await;

    match response {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
