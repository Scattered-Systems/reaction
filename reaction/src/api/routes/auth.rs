/*
   Appellation: index <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use async_session::{MemoryStore, Session, SessionStore};
use axum::{
    async_trait,
    http::{header::SET_COOKIE, HeaderMap},
    routing::{get, post},
    Json, Router, headers::Cookie
};
use axum::extract::{rejection::TypedHeaderRejectionReason, Extension, FromRequest, Query, Path, RequestParts, TypedHeader};
use axum::response::{IntoResponse,  Redirect, Response};
use http::header;
use oauth2::{basic::BasicClient, reqwest::async_http_client, ClientSecret, ClientId, AuthUrl, TokenUrl, RedirectUrl, AuthorizationCode,  CsrfToken, Scope, TokenResponse, AuthType};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

static COOKIE_NAME: &str = "SESSION";

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AuthRouter(pub String);

impl AuthRouter {
    pub fn new(data: String) -> Self {
        Self(data)
    }
    pub fn router(&mut self) -> Router {
        let oauth_client = oauth_client();
        let store = MemoryStore::new();

        Router::new()
            .route("/auth", get(index))
            .route("/token/:id", post(token))
            .route("/auth/jetbrains", get(auth_jbspace))
            .route("/auth/login", get(login_authorized))
            .route("/protected", get(protected))
            .route("/logout", get(logout))
            .layer(Extension(store))
            .layer(Extension(oauth_client))
    }
}

impl Default for AuthRouter {
    fn default() -> Self {
        Self::new("/oauth".to_string())
    }
}

fn oauth_client() -> BasicClient {
    // Environment variables (* = required):
    // *"CLIENT_ID"     "REPLACE_ME";
    // *"CLIENT_SECRET" "REPLACE_ME";
    //  "REDIRECT_URL"  "http://127.0.0.1:3000/auth/authorized";
    //  "AUTH_URL"      "https://accounts.google.com/o/oauth2/v2/auth";
    //  "TOKEN_URL"     "https://www.googleapis.com/oauth2/v4/token";

    let client_id = std::env::var("CLIENT_ID").expect("Missing CLIENT_ID!");
    let client_secret = std::env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET!");
    let redirect_url = std::env::var("REDIRECT_URL")
        .unwrap_or_else(|_| "https://localhost:9000/auth/redirect".to_string());

    let auth_url = std::env::var("AUTH_URL")
        .unwrap_or_else(|_| "https://scsys.jetbrains.space/oauth/auth".to_string());

    let token_url = std::env::var("TOKEN_URL")
        .unwrap_or_else(|_| "https://scsys.jetbrains.space/oauth/token".to_string());

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url).unwrap(),
        Some(TokenUrl::new(token_url).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}


/// Implements the authorization url following the OAuth2 specification
pub async fn authorize(Path(id): Path<usize>) -> Json<Value> {
    let data = json!({ "id": id });
    Json(data)
}

/// Implements the OAuth2 token
pub async fn token(Path(id): Path<usize>) -> Json<Value> {
    let data = json!({ "id": id });
    Json(data)
}


// The user data we'll get back from Google.
#[derive(Debug, Serialize, Deserialize)]
struct User {
    sub: String,
    picture: Option<String>,
    email: String,
    name: String,
}

// Session is optional
async fn index(user: Option<User>) -> impl IntoResponse {
    match user {
        Some(u) => format!(
            "Hey {}! You're logged in!\nYou may now access `/protected`.\nLog out with `/logout`.",
            u.name
        ),
        None => "You're not logged in.\nVisit `/auth/google` to do so.".to_string(),
    }
}

async fn auth_jbspace(Extension(client): Extension<BasicClient>) -> impl IntoResponse {
    // append("response_type", "code")
    // append("redirect_uri", "https://scattered-systems.com")
    // append("client_id", "897bd650-093e-44f3-97b5-142e76ddb795")
    // append("access_type", "offline")
    // append("request_credentials", "default")
    // append("scope", "**")
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("**".to_string()))
        .add_extra_param("access_type", "offline")
        .url();

    // Redirect to Google's oauth service
    Redirect::to(&auth_url.to_string())
}

// Valid user session required. If there is none, redirect to the auth page
async fn protected(user: User) -> impl IntoResponse {
    format!(
        "Welcome to the protected area :)\nHere's your info:\n{:?}",
        user
    )
}

async fn logout(
    Extension(store): Extension<MemoryStore>,
    TypedHeader(cookies): TypedHeader<Cookie>,
) -> impl IntoResponse {
    let cookie = cookies.get(COOKIE_NAME).unwrap();
    let session = match store.load_session(cookie.to_string()).await.unwrap() {
        Some(s) => s,
        // No session active, just redirect
        None => return Redirect::to("/"),
    };

    store.destroy_session(session).await.unwrap();

    Redirect::to("/")
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AuthRequest {
    code: String,
    state: String,
}

async fn login_authorized(
    Query(query): Query<AuthRequest>,
    Extension(store): Extension<MemoryStore>,
    Extension(oauth_client): Extension<BasicClient>,
) -> impl IntoResponse {
    // Get an auth token
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .unwrap();

    // Fetch user data from Google
    let client = reqwest::Client::new();
    let user_data: User = client
        .get("https://scsys.jetbrains.space/api/http/organization")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap()
        .json::<User>()
        .await
        .unwrap();

    // Create a new session filled with user data
    let mut session = Session::new();
    session.insert("user", &user_data).unwrap();

    // Store session and get corresponding cookie
    let cookie = store.store_session(session).await.unwrap().unwrap();

    // Build the cookie
    let cookie = format!("{}={}; SameSite=Lax; Path=/", COOKIE_NAME, cookie);

    // Set cookie
    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.parse().unwrap());

    (headers, Redirect::to("/"))
}

struct AuthRedirect;

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/auth/jetbrains").into_response()
    }
}

#[async_trait]
impl<B> FromRequest<B> for User
where
    B: Send,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = AuthRedirect;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(store) = Extension::<MemoryStore>::from_request(req)
            .await
            .expect("`MemoryStore` extension is missing");

        let cookies = TypedHeader::<Cookie>::from_request(req)
            .await
            .map_err(|e| match *e.name() {
                header::COOKIE => match e.reason() {
                    TypedHeaderRejectionReason::Missing => AuthRedirect,
                    _ => panic!("unexpected error getting Cookie header(s): {}", e),
                },
                _ => panic!("unexpected error getting cookies: {}", e),
            })?;
        let session_cookie = cookies.get(COOKIE_NAME).ok_or(AuthRedirect)?;

        let session = store
            .load_session(session_cookie.to_string())
            .await
            .unwrap()
            .ok_or(AuthRedirect)?;

        let user = session.get::<User>("user").ok_or(AuthRedirect)?;

        Ok(user)
    }
}
