use crate::users::User;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

/**
 * A simple HTTP endpoint that returns a "Hello world!" message.
 *
 * This function is a simple HTTP GET endpoint that responds with a "Hello world!" message.
 *
 */
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/**
 * This function is a simple HTTP POST endpoint that echoes the request body back to the client.
 *
 * # Parameters
 *
 * - `req_body`: A `String` representing the request body to be echoed back.
 *
 * # Returns
 *
 * An `impl Responder` that returns an HTTP response with the same request body.
 */
#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

/**
 * This function is a simple HTTP GET endpoint that responds with a personalized "Hey there!" message.
 *
 * # Parameters
 *
 * - `id`: A `String` representing the unique identifier for the user.
 *
 * # Returns
 *
 * An `impl Responder` that returns an HTTP response with the personalized message.
 */
pub async fn manual_hello(id: web::Path<String>) -> impl Responder {
    println!("id: {}", id);
    HttpResponse::Ok().body(format!("Hey there! {}", id))
}

/**
 * This function is a simple HTTP GET endpoint that responds with a personalized "Hey there!" message.
 *
 * # Parameters
 *
 * - `id`: A `String` representing the unique identifier for the user.
 *
 * # Returns
 *
 * An `impl Responder` that returns an HTTP response with the personalized message.
 */
pub async fn get_user(
    db: web::Data<Arc<Mutex<HashMap<String, User>>>>,
    user_id: web::Path<String>,
) -> impl Responder {
    let db_lock: std::sync::MutexGuard<HashMap<String, User>> = db.lock().unwrap();
    if let Some(user) = db_lock.get(&user_id.to_string()) {
        HttpResponse::Ok().json(user.clone()) // Clone user for response
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

/**
 * This function is a simple HTTP POST endpoint that adds a new user to the database.
 *
 * # Parameters
 *
 * - `db`: A `web::Data<Arc<Mutex<HashMap<String, User>>>>` representing the database connection.
 * - `user`: A `web::Json<User>` representing the new user to be added to the database.
 *
 * # Returns
 *
 * An `impl Responder` that returns an HTTP response with the newly added user.
 */
#[post("/users/")]
pub async fn add_user(
    db: web::Data<Arc<Mutex<HashMap<String, User>>>>,
    user: web::Json<User>,
) -> impl Responder {
    let mut db_lock: std::sync::MutexGuard<HashMap<String, User>> = db.lock().unwrap();
    let n = db_lock.len();
    println!("user: {:?}", user);
    let new_user = User::new(user.name().to_string().clone());
    db_lock.insert((n + 1).to_string(), new_user.clone());

    HttpResponse::Ok().json(new_user.clone())
}

/**
 * This function is a simple HTTP GET endpoint that returns a list of all users in the database.
 *
 * # Parameters
 *
 * - `db`: A `web::Data<Arc<Mutex<HashMap<String, User>>>>` representing the database connection.
 *
 * # Returns
 *
 * An `impl Responder` that returns an HTTP response with the list of all users in the database.
 */
#[get("/users/")]
pub async fn get_users(db: web::Data<Arc<Mutex<HashMap<String, User>>>>) -> impl Responder {
    let db_lock: std::sync::MutexGuard<HashMap<String, User>> = db.lock().unwrap();
    HttpResponse::Ok().json(db_lock.clone())
}

/**
 * Updates a user in the database.
 *
 * # Parameters
 *
 * - `db`: A `web::Data<Arc<Mutex<HashMap<String, User>>>>` representing the database connection.
 * - `user_id`: A `web::Path<String>` representing the unique identifier for the user to be updated.
 * - `user_from_json`: A `web::Json<User>` representing the new user data to be updated in the database.
 *
 * # Returns
 *
 * An `impl Responder` that returns an HTTP response with the updated user. If the user is not found, it returns a 404 Not Found response.
 */
#[put("/users/{id}")]
pub async fn update_users(
    db: web::Data<Arc<Mutex<HashMap<String, User>>>>,
    user_id: web::Path<String>,
    user_from_json: web::Json<User>,
) -> impl Responder {
    let mut db_lock: std::sync::MutexGuard<HashMap<String, User>> = db.lock().unwrap();
    if let Some(user) = db_lock.get_mut(&user_id.to_string()) {
        user.set_name(user_from_json.name().to_string());
        HttpResponse::Ok().json(user.clone())
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

/**
 * Removes a user from the database.
 *
 * # Parameters
 *
 * - `db`: A `web::Data<Arc<Mutex<HashMap<String, User>>>>` representing the database connection.
 * - `user_id`: A `web::Path<String>` representing the unique identifier for the user to be removed.
 *
 * # Returns
 *
 * An `impl Responder` that returns an HTTP response with the updated database after the user has been removed. If the user is not found, it returns a 404 Not Found response.
 */
#[delete("/users/{user_id}")]
pub async fn remove_user(
    db: web::Data<Arc<Mutex<HashMap<String, User>>>>,
    user_id: web::Path<String>,
) -> impl Responder {
    let mut db_lock: std::sync::MutexGuard<HashMap<String, User>> = db.lock().unwrap();
    db_lock.remove(&user_id.to_string());
    HttpResponse::Ok().json(db_lock.clone())
}
