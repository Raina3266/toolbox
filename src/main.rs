use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/youtube", post(youtube));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

// { "title": "You can do it", "view_count": 202, "is_published": true, "category": null }
// {  "title": "I can do it", "view_count": 222, "is_published": false, "category": "daily" }
#[derive(Serialize, Deserialize)]
struct YoutubeVideo {
    title: String,
    view_count: i32,
    is_published: bool,
    category: Option<String>,
}

async fn youtube(Json(mut info): Json<YoutubeVideo>) -> Json<YoutubeVideo> {
    info.title = info.title.replace("You", "I");
    info.view_count = info.view_count + 20;
    info.is_published = !info.is_published;
    info.category = match info.category {
        None => Some(String::from("daily")),
        Some(_) => None,
    };

    Json(info)
}
