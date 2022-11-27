use actix_web::{ HttpServer, App};
use actix_web::middleware::Logger;
use std::env;
use env_logger::Env;
use actix_files::Files;

#[actix_web::main]
pub async fn server() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    // Get the port number to listen on.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .expect("PORT must be a number");

    // Start a server, configuring the resources to serve.
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(Files::new("/", "./static/root").index_file("index.html"))
            .wrap(Logger::new("%a %t %r %s %b %P %s %D %{Referer}i %{User-Agent}i %T"))
    })
    .bind(("0.0.0.0", port))
    .expect("Can not bind to port 8000")
    .run()
    .await
}
 