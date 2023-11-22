use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware::Logger, cookie::time::Date};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use uuid::Uuid;

type Tweets = Vec<Tweet>;

struct AppState {
    app_name : String,
    counter: Mutex<i32>
}

#[derive(Serialize, Deserialize, Debug)]
struct Tweet{
    id: String,
    date: DateTime<Utc>,
    message: String,
    likes: Vec<Like>,
}

struct Like{
    id: String,
    date: DateTime<Utc>
}

impl Tweet{
    fn new(message: String) -> Self{
        Self { id: Uuid::new_v4().to_string(), date: Utc::now(), message, likes: vec![] }
    }
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap(); 
    *counter += 1;
    format!("{app_name} was visited {counter} times this run.")
}

#[get("/tweets")]
pub async fn list() -> HttpResponse {
    // TODO find the last 50 tweets and return them

    let tweets: Tweets = vec![];

    HttpResponse::Ok()
        .content_type("application/json")
        .json(tweets)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {  
    let app = web::Data::new(AppState {
        app_name: String::from("Twitter Clone"),
        counter: Mutex::new(0)
    });

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .app_data(app.clone())
            .service(index)
            .service(list)
            .wrap(Logger::new("%a %r %s %b %{Referer}i %{User-Agent}i %Ts"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
