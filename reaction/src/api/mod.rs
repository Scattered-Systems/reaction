/*
    Appellation: api <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::interface::*;

pub mod routes;

pub fn new() -> Api {
    Api::default()
}

pub fn from_context(ctx: crate::Context) -> Api {
    Api::new(ctx)
}

pub(crate) mod interface {
    use crate::{api::routes, Context};
    use axum::{Router, Server};
    use http::header::{HeaderName, AUTHORIZATION};
    use scsys::BoxResult;
    use serde::{Deserialize, Serialize};
    use tower_http::compression::CompressionLayer;
    use tower_http::propagate_header::PropagateHeaderLayer;
    use tower_http::sensitive_headers::SetSensitiveHeadersLayer;
    use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
    use utoipa::OpenApi;
    use utoipa_swagger_ui::SwaggerUi;

    #[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
    pub struct Api {
        pub ctx: Context,
    }

    impl Api {
        pub fn new(ctx: Context) -> Self {
            Self { ctx }
        }
        pub async fn client(&self) -> Router {
            let mut router = Router::new();
            // Merge other routers into the base router
            router = router.merge(routes::index::router());
            router = router.merge(SwaggerUi::new("/docs").url(
                "/api-doc/openapi.json",
                super::routes::docs::ApiDoc::openapi(),
            ));
            router = router
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().include_headers(true))
                        .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                        .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
                )
                .layer(SetSensitiveHeadersLayer::new(std::iter::once(
                    AUTHORIZATION,
                )))
                .layer(CompressionLayer::new())
                .layer(PropagateHeaderLayer::new(HeaderName::from_static(
                    "x-request-id",
                )))
                .layer(axum::Extension(self.ctx.clone()));
            router
        }
        /// Implements a graceful shutdown when users press CTRL + C
        pub async fn shutdown(&self) {
            tokio::signal::ctrl_c()
                .await
                .expect("Expect shutdown signal handler");
            tracing::info!("Terminating the application...");
        }
        /// Quickly run the api
        pub async fn serve(&self) -> BoxResult {
            let address = self.ctx.clone().settings.server.address();
            let client = self.client().await;
            let server = Server::bind(&address)
                .serve(client.into_make_service())
                .with_graceful_shutdown(self.shutdown())
                .await?;
            Ok(server)
        }
    }

    impl std::fmt::Display for Api {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", serde_json::to_string(&self).unwrap())
        }
    }
}
