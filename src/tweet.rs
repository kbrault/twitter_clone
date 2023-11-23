use actix_web::{delete, get, post, web, HttpResponse, Result};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;
use v_htmlescape::escape;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    pub id: String,
    pub date: DateTime<Utc>,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetPost {
    pub message: String,
}

#[get("/tweets")]
pub async fn get_tweet(db: web::Data<SqlitePool>) -> Result<HttpResponse> {
    let mut tweets: Vec<Tweet> = vec![];

    let result = sqlx::query("SELECT * FROM tweets ORDER BY date DESC")
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
                };
                tweets.push(tweet)
            }
            Ok(HttpResponse::Ok().json(tweets))
        }
        Err(_) => Ok(HttpResponse::InternalServerError().body("500 - Internal Server Error")),
    }
}

#[post("/tweet")]
pub async fn add_tweet(
    tweet_data: web::Json<TweetPost>,
    db: web::Data<SqlitePool>,
) -> Result<HttpResponse> {
    let tweet = tweet_data.into_inner(); // Extract tweet data from Json
    let current_datetime_utc = Utc::now().format("%Y-%m-%dT%H:%M:%S%.9fZ");

    let result = sqlx::query("INSERT INTO tweets (id, date, message) VALUES (?, ?, ?)")
        .bind(Uuid::new_v4().to_string())
        .bind(current_datetime_utc.to_string())
        .bind(escape(&tweet.message).to_string()) // TODO, check if there is no SQL Injection possible despite escape()
        .execute(db.as_ref())
        .await;

    match result {
        Ok(_) => Ok(HttpResponse::Created().body("Tweet added successfully")),
        Err(e) => {
            eprintln!("Failed to add tweet: {:?}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to add tweet"))
        }
    }
}

#[delete("/tweet/{id}")]
pub async fn delete_tweet(
    id: web::Path<String>,
    db: web::Data<SqlitePool>,
) -> Result<HttpResponse> {
    let result = sqlx::query("DELETE FROM tweets WHERE id=?")
        .bind(id.to_string()) // TODO, check if there is no SQL Injection possible
        .execute(db.as_ref())
        .await;

    match result {
        Ok(_) => Ok(HttpResponse::Created().body("Tweet deleted successfully")),
        Err(e) => {
            eprintln!("Failed to add tweet: {:?}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to delete tweet"))
        }
    }
}
