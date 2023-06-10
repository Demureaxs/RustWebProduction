pub mod create;
pub mod get;
pub mod edit;

use actix_web::web::{get, post, put, scope, ServiceConfig};

pub fn to_do_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/item")
            .route("create/{title}", post().to(create::create))
            .route("get", get().to(get::get))
            .route("edit", put().to(edit::edit)) // define view and url
    );
}
