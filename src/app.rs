use crate::routes::user_routes::get_users;
use actix_web::web;

// ミドルウェアの設定
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1").service(get_users));
}
