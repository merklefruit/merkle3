use actix_web::{web, HttpResponse};

pub fn tree_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(|| async { HttpResponse::Ok().body("tree") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

pub fn test_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
