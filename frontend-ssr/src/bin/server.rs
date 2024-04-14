use std::collections::HashMap;
use std::convert::Infallible;

use axum::body::Body;
use axum::extract::{Query, State};
use axum::handler::HandlerWithoutStateExt;
use axum::http::Uri;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use exeter_cycling_club::{ServerApp, ServerAppProps};
use futures::stream::{self, StreamExt};
use lambda_http::{run, tracing, Error};
use tower_http::{compression::CompressionLayer, services::ServeDir};
use yew::ServerRenderer;

const DIST_DIR: &str = "dist";

#[cfg(unix)]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

async fn render(
    url: Uri,
    Query(queries): Query<HashMap<String, String>>,
    State((index_html_before, index_html_after)): State<(String, String)>,
) -> Html<Body> {
    let url = url.path().to_owned();

    let renderer = ServerRenderer::<ServerApp>::with_props(move || ServerAppProps {
        url: url.into(),
        queries,
    });

    Html(Body::from_stream(
        stream::once(async move { index_html_before })
            .chain(renderer.render_stream())
            .chain(stream::once(async move { index_html_after }))
            .map(Result::<_, Infallible>::Ok),
    ))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let index_html_s = tokio::fs::read_to_string(DIST_DIR.to_owned() + "/index.html")
        .await
        .expect("failed to read index.html");

    let (index_html_before, index_html_after) = index_html_s.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");

    let index_html_after = index_html_after.to_owned();

    let compression_layer = CompressionLayer::new().gzip(true);

    let serve_dir = ServeDir::new(DIST_DIR)
        .append_index_html_on_directories(false)
        .fallback(
            get(render)
                .with_state((index_html_before.clone(), index_html_after.clone()))
                .into_service(),
        );

    let app = Router::new()
        .fallback_service(serve_dir)
        .layer(compression_layer);

    #[cfg(not(debug_assertions))]
    {
        // Run the server on AWS Lambda
        tracing::init_default_subscriber();
        run(app).await
    }

    #[cfg(debug_assertions)]
    {
        // Run the server locally for development
        println!("Listening on: http://127.0.0.1:3000");
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .unwrap();
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
        )
        .await?;
        Ok(())
    }
}
