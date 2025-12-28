
use axum::{
    extract::{Form},
    response::Html,
    routing::{get, post},
    Router, Extension,
};
use tera::{Tera, Context};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
struct GreetForm {
    name: String,
}

async fn greet_form() -> Html<&'static str> {
    Html(r#"
        <form action="/greet" method="post">
            Name: <input name="name">
            <input type="submit">
        </form>
    "#)
}

async fn greet_post(
    Extension(tera): Extension<Arc<Tera>>,
    Form(form): Form<GreetForm>,
) -> Html<String> {
    let mut ctx = Context::new();
    ctx.insert("name", &form.name);
    let rendered = tera.render("hello.html", &ctx).unwrap();
    Html(rendered)
}

#[tokio::main]
async fn main() {
    // initialize tracing (optional, but recommended for debugging)
    // tracing_subscriber::fmt::init();

    let tera = Arc::new(Tera::new("templates/**/*").unwrap());
    let app = Router::new()
        .route("/", get(greet_form))
        .route("/greet", post(greet_post))
        .layer(Extension(tera));

    // run our app with axum::serve, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}