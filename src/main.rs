use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_scalar::{Scalar, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::{api::ApiDoc, state::AppState};

mod api;
mod auth;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let state = AppState::from_env();

    // Run migrations
    sqlx::migrate!()
        .run(&state.pool)
        .await
        .expect("Failed to run migrations");

    tracing::info!("✅ Database migrations completed successfully");

    // Mount routes with OpenAPI documentation
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(routes::health))
        .routes(routes!(routes::list_posts))
        .routes(routes!(routes::create_post))
        .routes(routes!(routes::list_user_posts))
        .with_state(state)
        .split_for_parts();

    // Mount Swagger UI and Scalar endpoints
    let router = router
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", api.clone()))
        .merge(Scalar::with_url("/scalar", api));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tracing::info!("🚀 Running Axum REST API on http://localhost:{port}");
    tracing::info!("📖 Swagger UI available at http://localhost:{port}/swagger");
    tracing::info!("🔍 Scalar API available at http://localhost:{port}/scalar");

    axum::serve(listener, router).await.unwrap();
}
