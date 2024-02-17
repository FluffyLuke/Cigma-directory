use actix_web::{get, web, Responder, Result};

#[get("/create")]
async fn create_user() -> Result<impl Responder> {
    Ok(web::Json(""))
}