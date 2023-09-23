use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    model::{QueryOptions, Todo, UpdateTodoSchema, DB},
    response::{SingleTodoResponse, TodoData, TodoListResponse},
};

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "CRUD API Running";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn todos_list_handler(opts: Option<Query<QueryOptions>>, State(db): State<DB>) -> impl IntoResponse {
    let todos = db.lock().await;
    
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let todos: Vec<Todo> = todos
        .clone()
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect();

    let json_response = TodoListResponse {
        status: "success".to_string(),
        results: todos.len(),
        todos,
    };

    Json(json_response)
}

pub async fn create_todo_handler(State(db): State<DB>, Json(mut body): Json<Todo>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let mut todos = db.lock().await;

    if let Some(todo) = todos.iter().find(|todo| todo.title == body.title ) {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item '{}' already exists", todo.title)
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    let uuid_id = Uuid::new_v4();
    let datetime = chrono::Utc::now();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.date_created = Some(datetime);
    body.date_updated = Some(datetime);

    let todo = body.to_owned();

    todos.push(body);

    let json_response = SingleTodoResponse {
        status: "success".to_string(),
        data: TodoData { todo },
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn get_todo_handler(Path(id): Path<Uuid>, State(db): State<DB>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let todos = db.lock().await;

    if let Some(todo) = todos.iter().find(|todo| todo.id == Some(id.to_owned())) {
        let json_response = SingleTodoResponse {
            status: "success".to_string(),
            data: TodoData { todo: todo.clone() },
        };
        return Ok((StatusCode::OK, Json(json_response)));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Todo with ID: {} not found", id)
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

pub async fn edit_todo_handler(Path(id): Path<Uuid>, State(db): State<DB>, Json(body): Json<UpdateTodoSchema>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut todos = db.lock().await;

    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == Some(id.clone())) {
        let datetime = chrono::Utc::now();
        let title = body.title.to_owned().unwrap_or_else(|| todo.title.to_owned());
        let content = body.content.to_owned().unwrap_or_else(|| todo.content.to_owned());
        let completed = body.completed.unwrap();
        let payload = Todo {
            id: todo.id.to_owned(),
            title: if !title.is_empty() {
                title
            } else {
                todo.title.to_owned()
            },
            content: if !content.is_empty() {
                content
            } else {
                todo.content.to_owned()
            },
            completed: Some(completed),
            date_created: todo.date_created,
            date_updated: Some(datetime)
        };
        *todo = payload;

        let json_response = SingleTodoResponse {
            status: "success".to_string(),
            data: TodoData { todo: todo.clone() }
        };
        Ok((StatusCode::OK, Json(json_response)))

    } else {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Todo {} not found", id)
        });

        Err((StatusCode::NOT_FOUND, Json(error_response)))
    }

}

pub async fn delete_todo_handler(Path(id): Path<Uuid>, State(db): State<DB>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut todos = db.lock().await;

    if let Some(todo_idx) = todos.iter().position(|todo| todo.id == Some(id.clone())) {
        todos.remove(todo_idx);
        return Ok((StatusCode::NO_CONTENT, Json("")));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Todo {} not found", id)
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}