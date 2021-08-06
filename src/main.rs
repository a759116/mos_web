//pub mod services;

use actix_web::{App, HttpServer, web};

use mos_web::services::{home, memory::best_fit_allocate};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //println!("Hello, world! {:?}", mb);
    HttpServer::new(|| 
        App::new().service(
            web::scope("/os")
                .service(web::resource("/memory/{alloc_method}").route(web::post().to(best_fit_allocate))),
        )
        .service(web::resource("/").route(web::get().to(home::index))),
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
