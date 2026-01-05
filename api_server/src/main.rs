use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, post, put, web};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Tweet {
    title: String,
    content: String,
    date: i32,
}

#[get("/tweets")]
async fn get_all_tweets(tweets: web::Data<Vec<Tweet>>) -> impl Responder {
    HttpResponse::Ok().json(tweets.clone())
}

#[post("/tweets")]
async fn add_tweet() -> impl Responder {
    format!("Tweet Added Successfully!")
}

#[get("/tweets/{id}")]
async fn get_tweet(path: web::Path<i32>, tweets: web::Data<Vec<Tweet>>) -> HttpResponse {
    let id = path.into_inner();
    let num: usize = id.try_into().expect("value must be number");

    HttpResponse::Ok().json(tweets.get(num))
}

#[put("/tweets/{id}")]
async fn edit_tweet(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let num: usize = id.try_into().expect("value must be number");

    format!("Tweet with id {} edited!", num)
}

#[delete("/tweets/{id}")]
async fn delete_tweet(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let num: usize = id.try_into().expect("value must be number");

    format!("Tweet with id {} deleted!", num)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let tweets: Vec<Tweet> = vec![
        Tweet {
            title: String::from("Title 1"),
            content: String::from("Title 1"),
            date: 1231231,
        },
        Tweet {
            title: String::from("Title 2"),
            content: String::from("Title 2"),
            date: 1231231,
        },
        Tweet {
            title: String::from("Title 3"),
            content: String::from("Title 4"),
            date: 1231231,
        },
    ];

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tweets.clone()))
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
