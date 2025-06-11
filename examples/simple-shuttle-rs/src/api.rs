use actix_web::Error;
use actix_web::web::{Json, Path};
use apistos::actix::CreatedJson;
use apistos::{ApiComponent, api_component, api_operation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
#[api_component]
pub struct NewTodo {
  pub title: String,
  pub description: Option<String>,
}

#[derive(Serialize)]
#[api_component]
pub struct Todo {
  pub id: Uuid,
  pub title: String,
  pub description: Option<String>,
}

#[api_operation(summary = "Get an element from the todo list")]
pub(crate) async fn get_todo(todo_id: Path<Uuid>) -> Result<Json<Todo>, Error> {
  // because it is a sample app, we do not currently implement any logic to store todos
  Ok(Json(Todo {
    id: todo_id.into_inner(),
    title: "some title".to_string(),
    description: None,
  }))
}

#[api_operation(summary = "Add a new element to the todo list")]
pub(crate) async fn add_todo(body: Json<NewTodo>) -> Result<CreatedJson<Todo>, Error> {
  let new_todo = body.into_inner();
  Ok(CreatedJson(Todo {
    id: Uuid::new_v4(),
    title: new_todo.title,
    description: new_todo.description,
  }))
}
