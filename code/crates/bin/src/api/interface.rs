use axum;
use http::header;
use tower_http::{
    compression::CompressionLayer,
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    trace,
};

#[derive(Clone, Debug)]
pub struct Interface {
    pub address: std::net::SocketAddr,
    pub context: crate::api::context::Context,
}

impl Interface {
    pub async fn new(settings: crate::api::settings::Settings) -> Self {
        crate::api::logger::Logger::setup(&settings);

        let host = [0, 0, 0, 0];
        let port = settings.server.port;

        let address: std::net::SocketAddr = std::net::SocketAddr::from((host, port));
        let context = crate::api::context::Context::new(settings.clone());

        let client = axum::Router::new()
            .merge(crate::api::endpoints::base::create_route())
            .layer(
                trace::TraceLayer::new_for_http()
                    .make_span_with(
                        trace::DefaultMakeSpan::new().include_headers(true)
                    )
                    .on_request(
                        trace::DefaultOnRequest::new().level(tracing::Level::INFO)
                    )
                    .on_response(
                        trace::DefaultOnResponse::new().level(tracing::Level::INFO)
                    ),
            )
            .layer(
                SetSensitiveHeadersLayer::new(
                    std::iter::once(
                        header::AUTHORIZATION
                    )
                )
            )
            .layer(
                CompressionLayer::new()
            )
            .layer(
                PropagateHeaderLayer::new(
                    header::HeaderName::from_static(
                        "x-request-id"
                    )
                )
            )
            .layer(axum::Extension(context.clone()));

        println!("{}", settings.server);

        axum::Server::bind(&address)
            .serve(client.into_make_service())
            .await
            .expect("Failed to start server");

        Self { address, context }
    }
}