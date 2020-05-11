use actix_web::{get,web,App,HttpResponse,HttpServer,Responder};
use std::thread;
use std::time::Duration;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World !!!")
}

async fn hello() -> impl Responder {
    thread::sleep(Duration::from_secs(3));
    HttpResponse::Ok().body("Hello Rust")
}

#[get("/hello")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .route("/",web::get().to(index))
            .route("/hello",web::get().to(hello))
            .service(ping)
    })
        .bind("0.0.0.0:8077")?
        .run()
        .await
}
