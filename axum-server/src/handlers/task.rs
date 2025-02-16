use axum::{
    extract::{Path, Query, State},
    Json
};
use sqlx::{PgPool, QueryBuilder};
use crate::{
    models::task::{CreateTask, Task, TaskFilter, UpdateTask},
    error::AppError,
    auth::AuthUser,
};
use std::sync::Arc;
use serde_json::json;
use anyhow::Error;

pub async fn get_tasks(
    State(pool): State<Arc<PgPool>>,
    // 移除 AuthUser 暂时
    Query(filter): Query<TaskFilter>,
) -> Result<Json<Vec<Task>>, AppError> {
    let mut query = sqlx::QueryBuilder::new("SELECT * FROM tasks WHERE user_id = $1");

    if let Some(category) = filter.category {
        query.push(" AND category = ");
        query.push_bind(category);
    }

    if let Some(priority) = filter.priority {
        query.push(" AND priority = ");
        query.push_bind(priority);
    }

    if let Some(status) = filter.status {
        query.push(" AND status = ");
        query.push_bind(status);
    }

    let tasks = query
        .build_query_as::<Task>()
        .bind(1)
        // 临时使用固定用户ID 1
        .fetch_all(pool.as_ref())
        .await
        .map_err(|e| AppError::from(e))?;

    Ok(Json(tasks))
}


pub async fn create_task(
    auth_user: AuthUser,
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateTask>,
) -> Result<Json<Task>, AppError> {
    let task = sqlx::query_as::<_, Task>(
        "INSERT INTO tasks (title, description, category, priority, due_date, user_id, status, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, 'pending', NOW(), NOW())
         RETURNING *"
    )
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(&payload.category)
    .bind(payload.priority)
    .bind(payload.due_date)
    .bind(auth_user.user_id)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(Json(task))
}

pub async fn get_task(
    auth_user: AuthUser,
    State(pool): State<Arc<PgPool>>,
    Path(task_id): Path<i32>,
) -> Result<Json<Task>, AppError> {
    let task = sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks WHERE id = $1 AND user_id = $2"
    )
    .bind(task_id)
    .bind(auth_user.user_id)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(Json(task))
}

pub async fn update_task(
    auth_user: AuthUser,
    State(pool): State<Arc<PgPool>>,
    Path(task_id): Path<i32>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, AppError> {
    let mut query = QueryBuilder::new("UPDATE tasks SET updated_at = NOW()");

    if let Some(title) = payload.title {
        query.push(", title = ");
        query.push_bind(title);
    }

    if let Some(description) = payload.description {
        query.push(", description = ");
        query.push_bind(description);
    }

    if let Some(category) = payload.category {
        query.push(", category = ");
        query.push_bind(category);
    }

    if let Some(priority) = payload.priority {
        query.push(", priority = ");
        query.push_bind(priority);
    }

    if let Some(status) = payload.status {
        query.push(", status = ");
        query.push_bind(status);
    }

    if let Some(due_date) = payload.due_date {
        query.push(", due_date = ");
        query.push_bind(due_date);
    }

    query.push(" WHERE id = ");
    query.push_bind(task_id);
    query.push(" AND user_id = ");
    query.push_bind(auth_user.user_id);
    query.push(" RETURNING *");

    let task = query
        .build_query_as::<Task>()
        .fetch_one(pool.as_ref())
        // ✅ 使用 pool.as_ref()
        .await?;

    Ok(Json(task))
}


pub async fn delete_task(
    auth_user: AuthUser,
    State(pool): State<Arc<PgPool>>,
    Path(task_id): Path<i32>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query("DELETE FROM tasks WHERE id = $1 AND user_id = $2")
        .bind(task_id)
        .bind(auth_user.user_id)
        .execute(pool.as_ref())
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError(Error::msg("Task not found")));
    }

    Ok(Json(json!({
        "message": "Task deleted successfully"
    })))
}