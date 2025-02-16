//! [Firebase](https://firebase.google.com) authentication layer for popular frameworks.
//!
//! Support:
//!
//! - [Axum](https://github.com/tokio-rs/axum)
//!
//! ## Example:
//!
//!
//! ### Axum
//!
//! ```rust
//! use axum::{routing::get, Router};
//! use firebase_auth::{FirebaseAuth, FirebaseAuthState, FirebaseUser};
//!
//! async fn greet(user: FirebaseUser) -> String {
//!     let email = user.email.unwrap_or("empty email".to_string());
//!     format!("hello {}", email)
//! }
//!
//! async fn public() -> &'static str {
//!     "ok"
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let firebase_auth = FirebaseAuth::new("my-project-id").await;
//!
//!     let app = Router::new()
//!         .route("/hello", get(greet))
//!         .route("/", get(public))
//!         .with_state(FirebaseAuthState { firebase_auth });
//!
//!
//!     let addr = "127.0.0.1:8080";
//!     let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
//!
//!     axum::serve(listener, app).await.unwrap();
//! }
//! ```
//!
//!Visit [README.md](https://github.com/trchopan/firebase-auth/) for more details.

mod firebase_auth;
pub use firebase_auth::FirebaseAuth;

mod structs;
pub use structs::{FirebaseUser, PublicKeysError, FirebaseProvider};


#[cfg(feature = "axum")]
mod axum_feature;

#[cfg(feature = "axum")]
pub use axum_feature::FirebaseAuthState;
