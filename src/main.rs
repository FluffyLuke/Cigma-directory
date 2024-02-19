use actix_web::{get, post, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder};
use app_config::get_config_env;

// Here I can add shared state wrapped in mutex.
// AppState struct is created for each thread created.
struct AppState {
    app_name: String,
    version: String,
}

mod app_config;
mod handlers;
mod databases;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let conf = get_config_env();
    let conf = match conf {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error happened during parsing arguments: {}", e);
            return Ok(())
        }
    };

    println!("{:?}", conf);
    let database = databases::get_database(&conf).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Web"),
                version: String::from("0.0.1")
            }))
            .app_data(web::Data::new(database.clone()))
    })
    .keep_alive(None)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
