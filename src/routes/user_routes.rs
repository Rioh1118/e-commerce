use crate::models::user::User;
use actix_web::{get, HttpResponse, Responder};

#[get("/users")]
pub async fn get_users() -> impl Responder {
    let user = User {
        id: 1235890,
        name: String::from("Rio Hatta"),
        email: String::from("tapiokapann598@gmail.com"),
    };

    HttpResponse::Ok().json(user)
}
