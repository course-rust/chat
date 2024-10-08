mod config;
mod error;
mod handlers;
mod modules;
mod utils;

use handlers::*;
pub use modules::*;
use std::{ops::Deref, sync::Arc};

use axum::{
    routing::{get, patch, post},
    Router,
};
pub use config::AppConfig;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    pub(crate) inner: Arc<AppStateInner>,
}

#[allow(unused)]
#[derive(Debug)]
pub(crate) struct AppStateInner {
    pub(crate) config: AppConfig,
}

pub fn get_router(config: AppConfig) -> Router {
    let state = AppState::new(config);

    let api = Router::new()
        .route("/sigin", post(signin_handler))
        .route("/signup", post(signup_handler))
        .route("/chat", get(list_chat_handler).post(create_chat_handler))
        .route(
            "/chat/:id",
            patch(update_chat_handler)
                .delete(delete_chat_handler)
                .post(send_chat_handler),
        )
        .route("/chat/:id/messages", get(list_messages_handler));

    Router::new()
        // route
        .route("/", get(index_handler))
        // nest route
        .nest("/api", api)
        // state
        .with_state(state)
}

impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    fn new(config: AppConfig) -> Self {
        AppState {
            inner: Arc::new(AppStateInner { config }),
        }
    }
}
