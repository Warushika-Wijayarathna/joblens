// API routes for the JobLens application
pub mod auth;
pub mod jobs;
pub mod users;
pub mod applications;

use actix_web::web;

/// Configure all routes
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(auth::config)
       .configure(jobs::config)
       .configure(users::config)
       .configure(applications::config);
}
