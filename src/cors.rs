use futures::future::FutureObj;
use http::{header::HeaderValue, HeaderMap, Method, Response, StatusCode};
use http_service::Body;
use tide::{middleware::RequestContext, Middleware};

/// A blanket CORS middleware. It's customizable, but currently, 
/// it's a simple blanket impl for the route tree than dynamic.
pub struct CorsBlanketMiddleware {
    // TODO: Switch to HeaderValue
    max_age: String,
    methods: String,
    origin: String,
    headers: String,
}

impl Default for CorsBlanketMiddleware {
    fn default() -> Self {
        Self {
            max_age: Self::DEFAULT_MAX_AGE.to_owned(),
            methods: Self::DEFAULT_METHODS.to_owned(),
            origin: Self::STAR.to_owned(),
            headers: Self::STAR.to_owned(),
        }
    }
}

impl CorsBlanketMiddleware {

    pub const DEFAULT_MAX_AGE: &'static str = "86400"; // 24h => 24 * 60 * 60
    pub const DEFAULT_METHODS: &'static str = "GET, POST, OPTIONS";
    pub const STAR: &'static str = "*";

    // Headers
    pub const HEADER_ALLOW_ORIGIN: &'static str = "Access-Control-Allow-Origin";
    pub const HEADER_ALLOW_METHODS: &'static str = "Access-Control-Allow-Methods";
    pub const HEADER_ALLOW_HEADERS: &'static str = "Access-Control-Allow-Headers";
    pub const HEADER_MAX_AGE: &'static str = "Access-Control-Max-Age";

    pub fn new() -> Self {
        CorsBlanketMiddleware::default()
    }
    
    pub fn max_age<S: Into<String>>(mut self, max_age: S) -> Self {
        self.max_age = max_age.into();
        self
    }
    
    pub fn methods<S: Into<String>>(mut self, methods: S) -> Self {
        self.methods = methods.into();
        self
    }
    
    pub fn origin<S: Into<String>>(mut self, origin: S) -> Self {
        self.origin = origin.into();
        self
    }
    
    pub fn headers<S: Into<String>>(mut self, headers: S) -> Self {
        self.headers = headers.into();
        self
    }
}

impl<Data: Clone + Send> Middleware<Data> for CorsBlanketMiddleware {
    fn handle<'a>(&'a self, ctx: RequestContext<'a, Data>) -> FutureObj<'a, tide::Response> {
        FutureObj::new(Box::new(
            async move {
                if ctx.req.method() == Method::OPTIONS {
                    // TODO: Switch to HeaderValue and avoid the unwrap.
                    return Response::builder()
                        .status(StatusCode::OK)
                        .header(
                            Self::HEADER_ALLOW_ORIGIN,
                            HeaderValue::from_str(&self.origin).unwrap(),
                        )
                        .header(
                            Self::HEADER_ALLOW_METHODS,
                            HeaderValue::from_str(&self.methods).unwrap(),
                        )
                        .header(
                            Self::HEADER_ALLOW_HEADERS,
                            HeaderValue::from_str(&self.headers).unwrap(),
                        )
                        .header(
                            Self::HEADER_MAX_AGE,
                            HeaderValue::from_str(&self.max_age).unwrap(),
                        )
                        .body(Body::empty())
                        .unwrap();
                }
                let mut res = await!(ctx.next());
                let headers: &mut HeaderMap = res.headers_mut();
                headers
                    .entry(Self::HEADER_ALLOW_ORIGIN)
                    .unwrap()
                    .or_insert(HeaderValue::from_str(&self.origin).unwrap());
                res
            },
        ))
    }
}
