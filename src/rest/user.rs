use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    HttpResponse, Responder,
};
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

#[derive(Deserialize)]
struct UpdateUserParam {
    name: String,
}

#[put("/user/{id}")]
pub async fn update_user(
    id: web::Path<Info>,
    name: Json<UpdateUserParam>,
    data: web::Data<AppState>,
) -> impl Responder {
    let response = usecase::user::update_user(
        UserGateway {
            db_driver: data.db_driver,
        },
        id.id,
        name.name.clone(),
    )
    .await;
    match response {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize)]
struct DeleteUserPathParam {
    id: i32,
}

#[delete("/user/{id}")]
pub async fn delete_user(
    param: web::Path<DeleteUserPathParam>,
    data: web::Data<AppState>,
) -> impl Responder {
    let response = usecase::user::delete_user(
        UserGateway {
            db_driver: data.db_driver,
        },
        param.id,
    )
    .await;

    match response {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
