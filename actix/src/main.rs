use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs::File;
use std::io::prelude::*;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[warn(unused_must_use)]
async fn test() -> impl Responder {
    let mut file = File::open("example_849K.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    HttpResponse::Ok().body(contents)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/test", web::get().to(test))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
