use actix_service::Service;
use actix_web::{App, HttpServer};
mod json_serialization;
mod jwt;
mod processes;
mod state;
mod to_do;
mod views;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        println!("Http server factory is firing");
        let app = App::new()
            .wrap_fn(|req, srv| {
                println!("Request: {:?}", req);
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory);
        return app;
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// ------ standard actix -------
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         println!("Http server factor is firing");
//         let app = App::new();
//         return app;
//     })
//     .bind("127.0.0.1:8080")?
//     // if this command is omitted it will fire a core for each of your cpus
//     .workers(4)
//     .run()
//     .await
// }

// ---- use of futures ----

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let server_one = HttpServer::new(move || {
//         App::new().service(web::scope("/utils").route("/one", web::get().to(utils_one)))
//     })
//     .bind("0.0.0.0:3006")?
//     .run();

//     let server_two = HttpServer::new(move || {
//         App::new().service(web::resource("/health").route(web::get().to(health)))
//     })
//     .bind("0.0.0.0:8080")?
//     .run();

//     future::try_join(server_one, server_two).await?;

//     Ok(())
// }
