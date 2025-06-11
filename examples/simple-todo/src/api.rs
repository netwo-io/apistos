use actix_web::Error;
use actix_web::web::{Json, Path};
use apistos::actix::CreatedJson;
use apistos::{ApiComponent, api_operation};
use apistos_models::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct NewTodo {
  pub title: String,
  pub description: Option<String>,
}

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct Todo {
  pub id: Uuid,
  pub title: String,
  pub description: Option<String>,
  pub kind: TodoKind,
}

#[derive(Serialize, JsonSchema, ApiComponent)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum TodoKind {
  Simple(SimpleTodo),
  Complex(ComplexTodo),
}

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct ComplexTodo {
  pub visible: bool,
  pub labels: Vec<String>,
  pub permissions: Vec<String>,
}

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct SimpleTodo {
  pub visible: bool,
}

#[api_operation(summary = "Get an element from the todo list")]
pub(crate) async fn get_todo(todo_id: Path<Uuid>) -> Result<Json<Todo>, Error> {
  // because it is a sample app, we do not currently implement any logic to store todos
  Ok(Json(Todo {
    id: todo_id.into_inner(),
    title: "some title".to_string(),
    description: None,
    kind: TodoKind::Simple(SimpleTodo { visible: true }),
  }))
}

#[api_operation(summary = "Add a new element to the todo list")]
pub(crate) async fn add_todo(body: Json<NewTodo>) -> Result<CreatedJson<Todo>, Error> {
  let new_todo = body.into_inner();
  Ok(CreatedJson(Todo {
    id: Uuid::new_v4(),
    title: new_todo.title,
    description: new_todo.description,
    kind: TodoKind::Simple(SimpleTodo { visible: true }),
  }))
}
