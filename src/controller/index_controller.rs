use actix_web::{get, web, Responder, Result};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}

#[get("/")]
async fn index() -> Result<impl Responder> {
    Ok(web::Json(()))
}
