use actix_web::{ web, App, HttpResponse, HttpServer, Responder};
use presentation::user_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/user/get", web::get().to(user_controller::get_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}