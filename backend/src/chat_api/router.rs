use std::sync::Arc;

use axum::{body::Body, extract::Request, handler::Handler, http::Method, response::IntoResponse, routing::MethodRouter, Router};

use crate::{server_state::ServerState, structs::chatty_response::ChattyResponse};

#[derive(Clone)]
struct ProtectedHandle<H, P> {
    handle: H,
    protection: P,
}

struct ChatRouter {
    router: Router<Arc<ServerState>>
}

impl ChatRouter {
    fn new() -> ChatRouter {
        ChatRouter {router: Router::new()}
    }

    fn route<H, T, S, P>(mut self, path: &str, method: Method, protected_handle: ProtectedHandle<H, P>, state: S) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
        S: Clone + Send + Sync + 'static,
        P: Fn(&Request<Body>) -> Result<(), ChattyResponse> + Clone + Send + Sync + 'static,
    {
        
        let mut l: MethodRouter<Arc<ServerState>> = axum::routing::MethodRouter::new();
        let func = move |req: Request<Body>| async {
            match (protected_handle.clone().protection)(&req) {
                Ok(_) => protected_handle.handle.call(req, state).await,
                Err(err) => err.into_response()
            }
        };

        match method {
            Method::GET =>      { l = l.get(func); },
            Method::POST =>     { l = l.post(func); },
            Method::DELETE =>   { l = l.delete(func); },
            Method::PATCH =>    { l = l.patch(func); },
            Method::PUT =>      { l = l.put(func); },
            Method::HEAD =>     { l = l.head(func); },
            Method::TRACE =>    { l = l.trace(func); },
            Method::OPTIONS =>  { l = l.options(func); },

            _ => { unreachable!() },
        };

        self.router = self.router.route(path, l);
        self
    }

    fn build (self) -> Router<Arc<ServerState>> {
        self.router 
    }
}

pub fn create_chat_api_router(state: Arc<ServerState>) -> Router<Arc<ServerState>> {
    ChatRouter::new()
        .route("/", Method::POST, ProtectedHandle {
            handle: super::handles::send_message,
            protection: super::protections::send_message,
        }, state.clone())
        .build()
}
