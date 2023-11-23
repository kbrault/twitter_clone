use actix_files::NamedFile;
use actix_web::{middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use dotenvy;
use sqlx::SqlitePool;
use std::path::PathBuf;

mod tweet;

async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().body("404 - Not Found")
}

async fn index(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./static/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
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
            .route("/", web::get().to(index)) // GET /
            .service(tweet::get_tweet) // GET get_tweet
            .service(tweet::add_tweet) // POST add_tweet
            .service(tweet::delete_tweet) // DELETE delete_tweet
            .service(actix_files::Files::new("/static", "static").index_file("index.html")) // Static file handler (such as styles.css)
            .default_service(web::route().to(not_found)) // 404
            .wrap(Logger::new("%a %r %s %b %{Referer}i %{User-Agent}i %Ts")) // Logger
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}
