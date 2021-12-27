use actix_web::{get,HttpResponse,Responder};

#[get("/")]
pub async fn greeting() -> impl Responder {
    HttpResponse::Ok().body("Hello Dominic")
}



// pub async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }


