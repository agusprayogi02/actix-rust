use actix_web::{ App, HttpResponse, HttpServer, Responder};

#[actix_web::get("/")]
async fn hello() -> impl Responder{
    HttpResponse::Ok().body("Halo Dek!")
    }

#[actix_web::get("/hello")]
async fn say_hello() -> impl Responder {
    HttpResponse::Ok().body("Halo Semua 1234")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new().service(hello).service(say_hello)
    }).bind(("127.0.0.1", 8080))?
    .run()
    .await
}
