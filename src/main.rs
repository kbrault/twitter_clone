use actix_files as fs;
use actix_web::{
    get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder, Result,
};
use askama::Template;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use dotenvy;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

//use uuid::Uuid;

type Tweets = Response<Tweet>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    id: String,
    date: DateTime<Utc>,
    message: String,
    likes: Vec<Like>,
}

// TODO for POST :
// impl Tweet {
//     fn new(message: String) -> Self {
//         Self {
//             id: Uuid::new_v4().to_string(),
//             date: Utc::now(),
//             message,
//             likes: vec![],
//         }
//     }
// }

#[derive(Debug, Deserialize, Serialize)]
struct Like {
    id: String,
    date: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response<T> {
    results: Vec<T>,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    app_name: String,
}

async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().body("404 - Not Found")
}

#[get("/")]
async fn index() -> impl Responder {
    // Askama templating just for 1 variable : overkill, but just to try it :)
    let template = IndexTemplate {
        app_name: "Twitter Clone".to_string(),
    };

    HttpResponse::Ok().body(template.render().unwrap_or_else(|e| {
        eprintln!("Template rendering error: {}", e);
        "Error rendering template".to_string()
    }))
}

#[get("/tweets")]
pub async fn get_tweet(db: web::Data<SqlitePool>) -> Result<HttpResponse> {
    let mut tweets: Response<Tweet> = Tweets { results: vec![] };

    let result = sqlx::query("SELECT * FROM tweets")
        .fetch_all(db.as_ref())
        .await;

    match result {
        Ok(rows) => {
            for row in &rows {
                // Parsing from SQLite(TEXT) to Tweet<date: DateTime<Utc>>
                let date_db = row.get("date");
                let parsed_naive = NaiveDateTime::parse_from_str(date_db, "%Y-%m-%dT%H:%M:%S%.fZ")
                    .expect("Failed to parse date/time string");
                let datetime_utc = Utc.from_utc_datetime(&parsed_naive);
                let tweet = Tweet {
                    id: row.get("id"),
                    date: datetime_utc,
                    message: row.get("message"),
                    likes: vec![],
                };
                tweets.results.push(tweet)
            }
            Ok(HttpResponse::Ok().json(tweets))
        }
        Err(_) => Ok(HttpResponse::InternalServerError().body("500 - Internal Server Error")),
    }
}

//curl -d '{"id":"value1", "date":"2023-11-22T15:30:00Z", "message":"test", "likes":[]}' -H "Content-Type: application/json" -X POST http://127.0.0.1:8888/tweet
#[post("/tweet")]
pub async fn add_tweet(
    _tweet: web::Json<Tweet>,
    db: web::Data<SqlitePool>,
) -> Result<HttpResponse> {
    let result = sqlx::query("SELECT 1").fetch_all(db.as_ref()).await; // TODO : To be implemented
    match result {
        Ok(_) => {
            eprintln!("POST NotImplemented");
            Ok(HttpResponse::NotImplemented().body("501 - Not Implemented")) // TODO : To be implemented
        }

        Err(e) => {
            eprintln!("Failed to fetch tweets: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // DATABASE_URL .env variable
    let database_url = dotenvy::var("DATABASE_URL").unwrap();
    // Get the SQLite asynchronous pool
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // SQLite Connection Pool
            .service(index) // GET /
            .service(get_tweet) // GET get_tweet
            .service(add_tweet) // POST add_tweet
            .service(fs::Files::new("/static", "static").index_file("index.html")) // Static file handler (such as styles.css)
            .default_service(web::route().to(not_found)) // 404
            .wrap(Logger::new("%a %r %s %b %{Referer}i %{User-Agent}i %Ts")) // Logger
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}
