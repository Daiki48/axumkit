use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::sync::Arc;
use tera::Tera;

#[derive(Clone)]
struct AppState {
    tera: Arc<Tera>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let tera_path = concat!(env!("CARGO_MANIFEST_DIR"), "/client/**/*");
    let tera = Arc::new(Tera::new(tera_path)?);

    let app_state = AppState { tera };

    let app: Router = Router::new()
        .route("/", get(root))
        .route("/about", get(about_page))
        .fallback(get(not_found))
        .with_state(app_state);
    println!("Listening on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

fn common_context() -> tera::Context {
    let mut context = tera::Context::new();
    context.insert("title", "axumkit");
    context
}

#[axum_macros::debug_handler]
async fn root( State(state): State<AppState> ) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut context = common_context();
    context.insert("page_title", "Index");
    context.insert("message", "This is Index page.");

    Ok(state.tera
        .render("index.html", &context)
        .map(Html)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())))
}

#[axum_macros::debug_handler]
async fn about_page(State(state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut context = common_context();
    context.insert("page_title", "About");
    context.insert("message", "This is About page.");

    Ok(state.tera
        .render("pages/about.html", &context)
        .map(Html)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())))
}

async fn not_found(State(state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut context = common_context();
    context.insert("page_title", "Not Found");

    Ok((StatusCode::NOT_FOUND, state.tera
        .render("pages/not_found.html", &context)
        .map(Html)
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))))
}
