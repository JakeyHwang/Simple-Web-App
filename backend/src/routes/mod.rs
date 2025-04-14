use actix_web::web;
use crate::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(handlers::register))
                    .route("/login", web::post().to(handlers::login)),
            )
            .service(
                web::scope("/products")
                    .route("", web::get().to(handlers::get_products_handler))
                    .route("", web::post().to(handlers::create_product_handler))
                    .route("/{id}", web::get().to(handlers::get_product_handler))
                    .route("/{id}", web::put().to(handlers::update_product_handler))
                    .route("/{id}", web::delete().to(handlers::delete_product_handler)),
            ),
    );
} 