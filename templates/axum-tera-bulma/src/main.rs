use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tera::Tera;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app: Router = Router::new()
        .route("/", get(root))
        .route("/about", get(about_page))
        .fallback(get(not_found));
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

fn tera_include() -> Result<Tera, (StatusCode, String)> {
    let tera_path = concat!(env!("CARGO_MANIFEST_DIR"), "/client/**/*");
    Tera::new(tera_path).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

#[axum_macros::debug_handler]
async fn root() -> Result<impl IntoResponse, (StatusCode, String)> {
    let tera: Tera = tera_include()?;
    let mut context = common_context();
    context.insert("page_title", "Index");
    context.insert("message", "This is Index page.");

    Ok(tera
        .render("index.html", &context)
        .map(Html)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())))
}

#[axum_macros::debug_handler]
async fn about_page() -> Result<impl IntoResponse, (StatusCode, String)> {
    let tera: Tera = tera_include()?;
    let mut context = common_context();
    context.insert("page_title", "About");
    context.insert("message", "This is About page.");

    Ok(tera
        .render("pages/about.html", &context)
        .map(Html)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())))
}

async fn not_found() -> Result<impl IntoResponse, (StatusCode, String)> {
    let tera: Tera = tera_include()?;
    let mut context = common_context();
    context.insert("page_title", "Not Found");

    Ok(tera
        .render("pages/not_found.html", &context)
        .map(Html)
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;
    use tokio::task;

    #[tokio::test]
    async fn test_root() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let app = Router::new().route("/", get(root));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
        let server = task::spawn(async move {
            axum::serve(listener, app).await?;
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        });
        let client = Client::new();
        let res = client.get("http://127.0.0.1:3000").send().await?;
        assert_eq!(res.status(), StatusCode::OK);

        server.abort();
        Ok(())
    }
}
