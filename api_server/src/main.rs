use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
struct Tweet {
    title: String,
    content: String,
    date: i32,
}

struct AppState {
    tweets: Mutex<Vec<Tweet>>,
}

#[get("/tweets")]
async fn get_all_tweets(data: web::Data<AppState>) -> impl Responder {
    let tweets = data.tweets.lock().unwrap();
    HttpResponse::Ok().json(&*tweets)
}

#[post("/tweets")]
async fn add_tweet(req_tweet: web::Json<Tweet>, data: web::Data<AppState>) -> impl Responder {
    let mut tweets = data.tweets.lock().unwrap();
    tweets.push(req_tweet.into_inner());
    HttpResponse::Ok().body("Tweet Added Successfully!")
}

#[get("/tweets/{id}")]
async fn get_tweet(path: web::Path<usize>, data: web::Data<AppState>) -> HttpResponse {
    let id = path.into_inner();
    let tweets = data.tweets.lock().unwrap();

    if let Some(tweet) = tweets.get(id) {
        HttpResponse::Ok().json(tweet)
    } else {
        HttpResponse::NotFound().body("Tweet not found")
    }
}

#[put("/tweets/{id}")]
async fn edit_tweet(
    path: web::Path<usize>,
    req_tweet: web::Json<Tweet>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner();
    let mut tweets = data.tweets.lock().unwrap();

    if id < tweets.len() {
        tweets[id] = req_tweet.into_inner();
        HttpResponse::Ok().body(format!("Tweet with id {} edited!", id))
    } else {
        HttpResponse::NotFound().body("Tweet not found")
    }
}

#[delete("/tweets/{id}")]
async fn delete_tweet(path: web::Path<usize>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let mut tweets = data.tweets.lock().unwrap();

    if id < tweets.len() {
        tweets.remove(id);
        HttpResponse::Ok().body(format!("Tweet with id {} deleted!", id))
    } else {
        HttpResponse::NotFound().body("Tweet not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tweets = vec![
        Tweet {
            title: String::from("Title 1"),
            content: String::from("Content 1"),
            date: 1231231,
        },
        Tweet {
            title: String::from("Title 2"),
            content: String::from("Content 2"),
            date: 1231231,
        },
        Tweet {
            title: String::from("Title 3"),
            content: String::from("Content 3"),
            date: 1231231,
        },
    ];

    let app_state = web::Data::new(AppState {
        tweets: Mutex::new(tweets),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(get_all_tweets)
            .service(add_tweet)
            .service(get_tweet)
            .service(edit_tweet)
            .service(delete_tweet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
