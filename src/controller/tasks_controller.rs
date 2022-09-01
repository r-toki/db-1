use crate::{app_state::AppState, model::task::NewTask, result::Result};
use actix_web::{
    get, post,
    web::{Data, Json, ServiceConfig},
    Responder,
};
use serde::Deserialize;
use validator::Validate;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(create);
}

#[get("/tasks")]
async fn index(app_state: Data<AppState>) -> Result<impl Responder> {
    let tasks = app_state.db.tasks.find_all().await?;
    Ok(Json(tasks))
}

#[derive(Deserialize, Validate)]
pub struct CreateForm {
    #[validate(length(min = 1))]
    description: String,
}

#[post("/tasks")]
async fn create(app_state: Data<AppState>, form: Json<CreateForm>) -> Result<impl Responder> {
    form.validate()?;
    let new_task = NewTask {
        description: form.description.clone(),
    };
    let task = app_state.db.tasks.insert(new_task).await?;
    Ok(Json(task))
}
