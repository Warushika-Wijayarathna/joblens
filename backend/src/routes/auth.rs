use actix_web::{web, HttpResponse, Result};

// Authentication routes
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login))
            .route("/register", web::post().to(register))
            .route("/logout", web::post().to(logout))
    );
}

async fn login() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("Login endpoint"))
}

async fn register() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("Register endpoint"))
}

async fn logout() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("Logout endpoint"))
}
