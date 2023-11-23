use actix_files as fs;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder, Result};
use askama::Template;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use dotenvy;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

//use uuid::Uuid;

type Tweets = Response<Tweet>;

#[derive(Debug, Deserialize, Serialize)]
struct Tweet {
    id: String,
    date: DateTime<Utc>,
    message: String,
    likes: Vec<Like>,
}

// TODO :
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

#[get("/")]
async fn index() -> impl Responder {
    let template = IndexTemplate {
        app_name: "Twitter Clone".to_string(),
    };

    HttpResponse::Ok().body(template.render().unwrap_or_else(|e| {
        eprintln!("Template rendering error: {}", e);
        "Error rendering template".to_string()
    }))
}

#[get("/tweets")]
pub async fn list(db: web::Data<SqlitePool>) -> Result<HttpResponse, actix_web::Error> {
    let mut tweets: Response<Tweet> = Tweets { results: vec![] };

    let result = sqlx::query("SELECT * FROM tweets")
        .fetch_all(db.as_ref())
        .await;

    match result {
        Ok(rows) => {
            for row in &rows {
                let date_db = row.get("date");
                let parsed_naive = NaiveDateTime::parse_from_str(date_db, "%Y-%m-%dT%H:%M:%S%.fZ")
                    .expect("Failed to parse date/time string");
                let datetime_utc = Utc.from_utc_datetime(&parsed_naive);
                let to_add = Tweet {
                    id: row.get("id"),
                    date: datetime_utc,
                    message: row.get("message"),
                    likes: vec![],
                };
                tweets.results.push(to_add)
            }
            Ok(HttpResponse::Ok().json(tweets))
        }
        Err(e) => {
            eprintln!("Failed to fetch users: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

// TODO : #[post("/tweets")]
// API POST/tweets --> add a tweet to database

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = dotenvy::var("DATABASE_URL").unwrap();
    let pool = SqlitePool::connect(&database_url).await.unwrap();

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(index)
            .service(list)
            .service(fs::Files::new("/static", "static").index_file("index.html"))
            .wrap(Logger::new("%a %r %s %b %{Referer}i %{User-Agent}i %Ts"))
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}
