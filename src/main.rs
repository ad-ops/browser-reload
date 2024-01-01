use std::{convert::Infallible, time::Duration};
use axum::{
    routing::get,
    Router, response::{sse::{Event, KeepAlive}, Sse, Html},
};
use axum_extra::{TypedHeader, headers};
use tokio_stream::StreamExt as _;
use futures::{Stream, stream};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/hello", get(hello))
        .route("/reload", get(reload));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on http://localhost:{}", listener.local_addr().unwrap().port());
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> Html<&'static str> {
    Html(include_str!("../static/hello.html"))
}

async fn reload(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());
    let stream = stream::repeat_with(|| Event::default().retry(Duration::from_millis(500)))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream)
        .keep_alive(KeepAlive::default())
}