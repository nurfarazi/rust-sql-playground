use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::{Mssql, Pool};

use crate::models::{CreateUser, UpdateUser, User};

pub async fn get_users(
    State(pool): State<Pool<Mssql>>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = sqlx::query_as::<_, User>("SELECT id, name, email, CONVERT(VARCHAR, created_at, 120) as created_at FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(users))
}

pub async fn create_user(
    State(pool): State<Pool<Mssql>>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let id = sqlx::query_scalar::<_, i32>(
        "INSERT INTO users (name, email) OUTPUT INSERTED.id VALUES (@p1, @p2)",
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = sqlx::query_as::<_, User>("SELECT id, name, email, CONVERT(VARCHAR, created_at, 120) as created_at FROM users WHERE id = @p1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(user))
}

pub async fn update_user(
    Path(id): Path<i32>,
    State(pool): State<Pool<Mssql>>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let _ = sqlx::query("UPDATE users SET name = @p1, email = @p2 WHERE id = @p3")
        .bind(&payload.name)
        .bind(&payload.email)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = sqlx::query_as::<_, User>("SELECT id, name, email, CONVERT(VARCHAR, created_at, 120) as created_at FROM users WHERE id = @p1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(user))
}

pub async fn delete_user(
    Path(id): Path<i32>,
    State(pool): State<Pool<Mssql>>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query("DELETE FROM users WHERE id = @p1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn initialize_db(
    State(pool): State<Pool<Mssql>>,
) -> Result<String, (StatusCode, String)> {
    // 1. Create Database if not exists
    let db_check = sqlx::query("SELECT name FROM sys.databases WHERE name = 'RustCrudApp'")
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to check DB: {}", e)))?;

    if db_check.is_none() {
        sqlx::query("CREATE DATABASE RustCrudApp")
            .execute(&pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create DB: {}", e)))?;
    }

    // 2. Create Table
    // We explicitly refer to RustCrudApp.dbo.users to ensure it's created in the right place
    // even if we are connected to master.
    let table_query = r#"
        IF NOT EXISTS (SELECT * FROM RustCrudApp.sys.objects WHERE object_id = OBJECT_ID(N'RustCrudApp.dbo.users') AND type in (N'U'))
        BEGIN
            CREATE TABLE RustCrudApp.dbo.users (
                id INT IDENTITY(1,1) PRIMARY KEY,
                name NVARCHAR(100) NOT NULL,
                email NVARCHAR(100) NOT NULL,
                created_at DATETIME2 DEFAULT GETDATE()
            );
        END
    "#;

    sqlx::query(table_query)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create table: {}", e)))?;

    Ok("Database and table initialized successfully".to_string())
}
