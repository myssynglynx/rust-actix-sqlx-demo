use actix_web::{
    get, post, delete,
    web::{Data, Json, Path},
    Responder, HttpResponse
};
use chrono;
use sqlx;
use uuid::Uuid;

use crate::AppState;
use super::models::{InputUser, User, InputPost, Post};

// Users

#[get("/users")]
pub async fn get_users(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&state.db)
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::NotFound().json("No users found"),
    }
}

#[get("/users/{id}")]
pub async fn get_user(state: Data<AppState>, path: Path<Uuid>) -> impl Responder {
    let id: Uuid = path.into_inner();

    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().json("No user found")
    }
}

#[post("/users")]
pub async fn create_user(state: Data<AppState>, body: Json<InputUser>) -> impl Responder {
    let joined_at = chrono::Utc::now();

    match sqlx::query_as::<_, User>(
        "INSERT INTO users (username, address, joined_at)
        VALUES ($1, $2, $3)
        RETURNING id, username, address, joined_at"
    )
        .bind(body.username.to_string())
        .bind(body.address.to_string())
        .bind(joined_at)
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create user"),
    }
}

#[delete("/users/{id}")]
pub async fn delete_user(state: Data<AppState>, path: Path<Uuid>) -> impl Responder {
    let id: &Uuid = &path.into_inner();

    let mut conn = match state.db.begin().await {
        Ok(x) => x,
        Err(_) => return HttpResponse::InternalServerError().json("Failed to read pool"),
    };

    match sqlx::query("DELETE FROM posts WHERE author_id = $1")
        .bind(id)
        .execute(&mut conn)
        .await
    {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().json("Failed to delete posts"),
    };

    match sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&mut conn)
        .await
    {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().json("Failed to delete user"),
    };

    match conn.commit().await {
        Ok(_) => HttpResponse::Ok().json(format!("Successfully deleted posts & user {id}")),
        Err(_) => HttpResponse::InternalServerError().json(format!("Failed to delete posts & user {id}")),
    }
}

// Posts

#[get("/posts")]
pub async fn get_posts(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Post>("SELECT * FROM posts")
        .fetch_all(&state.db)
        .await
    {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(_) => HttpResponse::NotFound().json("No posts found"),
    }
}

#[get("/posts/{id}")]
pub async fn get_post(state: Data<AppState>, path: Path<Uuid>) -> impl Responder {
    let id: Uuid = path.into_inner();

    match sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
        .bind(id)
        .fetch_all(&state.db)
        .await
    {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(_) => HttpResponse::NotFound().json("No posts found"),
    }
}

#[get("/users/{id}/posts")]
pub async fn get_user_posts(state: Data<AppState>, path: Path<Uuid>) -> impl Responder {
    let id: Uuid = path.into_inner();

    match sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE author_id = $1")
        .bind(id)
        .fetch_all(&state.db)
        .await
    {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(_) => HttpResponse::NotFound().json("No posts found"),
    }
}

#[post("/users/{id}/posts")]
pub async fn create_user_post(state: Data<AppState>, path: Path<Uuid>, body: Json<InputPost>) -> impl Responder {
    let id: Uuid = path.into_inner();
    let created_at = chrono::Utc::now();

    match sqlx::query_as::<_, Post>(
        "INSERT INTO posts (author_id, title, content, created_at, edited_at)
        VALUES ($1, $2, $3, $4, $4)
        RETURNING id, author_id, title, content, created_at, edited_at"
    )
        .bind(id)
        .bind(body.title.to_string())
        .bind(body.content.to_string())
        .bind(created_at)
        .bind(created_at)
        .fetch_one(&state.db)
        .await
    {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create user article"),
    }
}

#[delete("/posts/{id}")]
pub async fn delete_post(state: Data<AppState>, path: Path<Uuid>) -> impl Responder {
    let id: Uuid = path.into_inner();

    match sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .fetch_all(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(format!("Successfully deleted post {id}")),
        Err(_) => HttpResponse::InternalServerError().json("Failed to delete post"),
    }
}

async fn _delete_user(state: Data<&AppState>, id: &Uuid) -> HttpResponse {
    match sqlx::query("DELETE * FROM users WHERE id = $1")
        .bind(id)
        .fetch_all(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(format!("Successfully deleted user {id}")),
        Err(_) => HttpResponse::InternalServerError().json("Failed to delete user"),
    }
}
