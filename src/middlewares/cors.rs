use futures::future::FutureObj;
use http::{header::HeaderValue, HeaderMap, Method, Response, StatusCode};
use http_service::Body;
use tide::{
    middleware::{Middleware, Next},
    Context,
};

/// A blanket CORS middleware. It's customizable, but currently,
/// it's a simple blanket impl for the route tree than dynamic.
///
/// # Examples
///
/// ```rust
/// use surf::middlewares;
/// use http::header::HeaderValue;
/// 
/// let mut app = tide::App::new(()); 
/// app.middleware(middlewares::cors::CorsBlanket::new()
///      .origin(HeaderValue::from_str("https://surf-with-the-tide").unwrap())
///      .max_age(HeaderValue::from_str("600").unwrap()));
/// ```
///
pub struct CorsBlanket {
    max_age: HeaderValue,
    methods: HeaderValue,
    origin: HeaderValue,
    headers: HeaderValue,
}

impl Default for CorsBlanket {
    fn default() -> Self {
        Self {
            max_age: HeaderValue::from_static(DEFAULT_MAX_AGE),
            methods: HeaderValue::from_static(DEFAULT_METHODS),
            origin: HeaderValue::from_static(STAR),
            headers: HeaderValue::from_static(STAR),
        }
    }
}

pub const DEFAULT_MAX_AGE: &str = "86400";
pub const DEFAULT_METHODS: &str = "GET, POST, OPTIONS";
pub const STAR: &str = "*";

impl CorsBlanket {
    pub fn new() -> Self {
        CorsBlanket::default()
    }

    pub fn max_age<S: Into<HeaderValue>>(mut self, max_age: S) -> Self {
        self.max_age = max_age.into();
        self
    }

    pub fn methods<S: Into<HeaderValue>>(mut self, methods: S) -> Self {
        self.methods = methods.into();
        self
    }

    pub fn origin<S: Into<HeaderValue>>(mut self, origin: S) -> Self {
        self.origin = origin.into();
        self
    }

    pub fn headers<S: Into<HeaderValue>>(mut self, headers: S) -> Self {
        self.headers = headers.into();
        self
    }
}

impl<Data: Send + Sync + 'static> Middleware<Data> for CorsBlanket {
    fn handle<'a>(
        &'a self,
        ctx: Context<Data>,
        next: Next<'a, Data>,
    ) -> FutureObj<'a, tide::Response> {
        use http::header;
        box_async! {
            if ctx.method() == Method::OPTIONS {
                return Response::builder()
                    .status(StatusCode::OK)
                    .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, self.origin.clone())
                    .header(header::ACCESS_CONTROL_ALLOW_METHODS, self.methods.clone())
                    .header(header::ACCESS_CONTROL_ALLOW_HEADERS, self.headers.clone())
                    .header(header::ACCESS_CONTROL_MAX_AGE, self.max_age.clone())
                    .body(Body::empty())
                    .unwrap();
            }
            let mut res = await!(next.run(ctx));
            let headers: &mut HeaderMap = res.headers_mut();
            headers
                .entry(header::ACCESS_CONTROL_ALLOW_ORIGIN)
                .unwrap()
                .or_insert(self.origin.clone());
            res
        }
    }
}
