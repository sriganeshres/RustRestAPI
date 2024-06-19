use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use actix_web::{App, HttpServer, web};
use handlers::{remove_user, update_users};
use crate::{
    users::User,
    handlers::{hello, echo, manual_hello, get_user, add_user, get_users},
};
 // Module for User struct
mod handlers;
mod users; // Module for handler functions

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let users_db: Arc<Mutex<HashMap<String, User>>> = Arc::new(Mutex::new(HashMap::new()));
    {
        let mut map = users_db.lock().unwrap();
        map.insert("1".to_string(), User::new("Alice".to_string()));
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(users_db.clone()))
            .service(hello)
            .service(echo)
            .service(add_user)
            .service(get_users)
            .service(update_users)
            .service(remove_user)
            .route("/hey/{id}", web::get().to(manual_hello))
            .route("/users/{id}", web::get().to(get_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

