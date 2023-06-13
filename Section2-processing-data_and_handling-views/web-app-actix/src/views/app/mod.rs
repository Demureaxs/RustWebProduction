pub mod items;
pub mod content_loader;
use actix_web::web;

pub fn app_views_factor(app: &mut web::ServiceConfig) {
    app.route("/", web::get().to(items::items));
}