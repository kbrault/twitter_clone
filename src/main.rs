use actix_web::{get, web, App, HttpServer, middleware::Logger, Responder, Result};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::{sync::Mutex, vec};
use uuid::Uuid;

type Tweets = Response<Tweet>;

struct AppState {
    app_name : String,
    counter: Mutex<i32>
}

#[derive(Debug, Deserialize, Serialize)]
struct Tweet {
    id: String,
    date: DateTime<Utc>,
    message: String,
    likes: Vec<Like>,
}

impl Tweet{
    fn new(message: String) -> Self{
        Self { 
            id: Uuid::new_v4().to_string(), 
            date: Utc::now(), 
            message, 
            likes: vec![] }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Like{
    id: String,
    date: DateTime<Utc>
}

#[derive(Debug, Deserialize, Serialize)]
struct Response<T> {
    results: Vec<T>,
}



#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap(); 
    *counter += 1;
    format!("{app_name} was visited {counter} times this run.")
}

#[get("/tweets")]
pub async fn list() -> Result<impl Responder> {
    let mut tweets: Response<Tweet> = Tweets {results: vec![]};
    
    let tweet1 = Tweet::new("tweet_1".to_string());
    let tweet2 = Tweet::new("tweet_2".to_string());
    let tweet3 = Tweet::new("tweet_3".to_string());

    tweets.results.push(tweet1);
    tweets.results.push(tweet2);
    tweets.results.push(tweet3);

    Ok(web::Json(tweets))
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
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}
