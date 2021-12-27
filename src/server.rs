use actix_web::{web,App,HttpServer};
use actix_web::{middleware,get,HttpResponse,Responder};
use actix_web::{post};
use std::sync::Mutex;
use actix_files::Files;
// use actix_files as fs;

#[post("/echo")]

async fn echo(req_body:String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// struct AppState{
//     app_name : String,
// }

#[get("/user")]
async fn show_users() -> impl Responder {
    HttpResponse::Ok().body("Hello Dominic")
}


struct AppStateWithCounter{
    counter: Mutex<i32>,
}

#[get("/")]
async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // get counter's MutexGuard
    *counter += 1; // access counter inside MutexGuard
    format!("Request number: {}", counter) // with count
}

#[get("/")]
async fn greeting() -> impl Responder {
    HttpResponse::Ok().body("Hello Dominic")
}

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok().body("test")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| HttpResponse::Ok().body("app")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

#[actix_web::main]
pub async fn server() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .route("/", web::get().to(|| HttpResponse::Ok().body("/")))
            .service(Files::new("/", "./static/root").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .workers(4)
    .run()
    .await
}