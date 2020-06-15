use actix_web::{web, App, HttpResponse, HttpServer, Responder};


async fn index() -> impl Responder {
    HttpResponse::Ok().body(format!("Welcome {}", "test"))
}

async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello world1")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(root))
            .service(
                web::scope(
                    "/app").route("/index.html", web::get().to(index)
                )
            )
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}


// use actix_web::{web, HttpResponse};
// use serde::Deserialize;
//
// #[derive(Deserialize)]
// struct FormData {
//     username: String,
// }
//
// async fn index(form: web::Form<FormData>) -> HttpResponse {
//     HttpResponse::Ok().body(format!("username: {}", form.username))
// }
