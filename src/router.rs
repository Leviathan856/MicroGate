use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use wireframe::{HttpMethod, HttpRequest};

use crate::response::Response;

pub struct RequestContext {
    pub req: HttpRequest,
}

pub type HandlerFuture = Pin<Box<dyn Future<Output = Response> + Send + 'static>>;

pub trait Handler: Send + Sync + 'static {
    fn handle(&self, ctx: RequestContext) -> HandlerFuture;
}

impl<F, Fut> Handler for F
where
    F: Fn(RequestContext) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    fn handle(&self, ctx: RequestContext) -> HandlerFuture {
        Box::pin(self(ctx))
    }
}

#[derive(Default)]
pub struct Router {
    routes: HashMap<(HttpMethod, String), Box<dyn Handler>>,
}

impl Router {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Handler,
    {
        self.routes
            .insert((HttpMethod::GET, path.to_string()), Box::new(handler));
        self
    }

    pub fn post<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Handler,
    {
        self.routes
            .insert((HttpMethod::POST, path.to_string()), Box::new(handler));
        self
    }

    pub fn put<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Handler,
    {
        self.routes
            .insert((HttpMethod::PUT, path.to_string()), Box::new(handler));
        self
    }

    pub fn delete<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Handler,
    {
        self.routes
            .insert((HttpMethod::DELETE, path.to_string()), Box::new(handler));
        self
    }

    pub fn patch<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Handler,
    {
        self.routes
            .insert((HttpMethod::PATCH, path.to_string()), Box::new(handler));
        self
    }

    pub fn route(&self, method: &HttpMethod, path: &str) -> Option<&dyn Handler> {
        self.routes
            .get(&(*method, path.to_string()))
            .map(|b| b.as_ref())
    }
}
