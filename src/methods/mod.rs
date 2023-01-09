use actix_web::{HttpResponse, Responder, web, get, post, put, patch, delete};
use serde::{Deserialize};
use core::option::Option;
use crate::{AppState};
use crate::models::Task;

#[derive(Deserialize)]
pub struct CreateTaskForm {
    name: String
}

#[derive(Deserialize)]
pub struct UpdateTaskForm {
    name: String
}

#[derive(Deserialize)]
pub struct TaskIdParam {
    task_id: usize
}

#[get("/tasks")]
pub async fn get_tasks(data: web::Data<AppState>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();
    let body = serde_json::to_string(&tasks.clone()).unwrap();
    HttpResponse::Ok().body(body)
}

#[post("/tasks")]
pub async fn create_task(data: web::Data<AppState>, json: web::Json<CreateTaskForm>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    let new_id = tasks.len().clone();

    let new_task = Task::new(
        new_id,
        json.name.clone(),
    );

    let body = serde_json::to_string(&new_task).unwrap();
    tasks.push(new_task);
    HttpResponse::Ok().body(body)
}

#[put("/tasks/{task_id}")]
pub async fn change_task_name(path: web::Path<TaskIdParam>, data: web::Data<AppState>, json: web::Json<UpdateTaskForm>) -> impl Responder {
    let tasks = &mut data.tasks.lock().unwrap();
    let maybe_index = if tasks.len() > path.task_id { Option::from(path.task_id) } else { None };

    match maybe_index {
        Some(index) => {
            let target = &mut tasks[index];
            target.update_name(json.name.clone());
            HttpResponse::Ok().body(serde_json::to_string(target).unwrap())
        },
        None => HttpResponse::NotFound().body(format!("{} is not found", path.task_id)),
    }
}

#[patch("/tasks/{task_id}")]
pub async fn toggle_task_status(path: web::Path<TaskIdParam>, data: web::Data<AppState>) -> impl Responder {
    let tasks = &mut data.tasks.lock().unwrap();
    let maybe_index = if tasks.len() > path.task_id { Option::from(path.task_id) } else { None };

    match maybe_index {
        Some(index) => {
            let target = &mut tasks[index];
            target.toggle_status();
            HttpResponse::Ok().body(serde_json::to_string(target).unwrap())
        },
        None => HttpResponse::NotFound().body(format!("{} is not found", path.task_id)),
    }
}

#[delete("/tasks/{task_id}")]
pub async fn delete_task(path: web::Path<TaskIdParam>, data: web::Data<AppState>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    let maybe_index = if tasks.len() > path.task_id { Option::from(path.task_id) } else { None };

    match maybe_index {
        Some(index) => {
            let target = &mut tasks[index];
            target.delete();
            HttpResponse::NoContent().body("")
        },
        None => HttpResponse::NotFound().body(format!("{} is not found", path.task_id)),
    }
}