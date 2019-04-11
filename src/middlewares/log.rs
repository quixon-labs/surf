use futures::future::FutureObj;
use log::{info, trace};
use tide::{middleware::RequestContext, Middleware};

/// A simple requests logger
#[derive(Debug, Clone)]
pub struct RequestLogger {
    timer_on: bool,
    // TODO: formatter
    // formatter: Box<?>
}

impl Default for RequestLogger {
    fn default() -> Self {
        Self { timer_on: false }
    }
}

impl RequestLogger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn timed(mut self) -> Self {
        self.timer_on = true;
        self
    }

    async fn log_basic<'a, Data: Clone + Send>(
        &'a self,
        ctx: RequestContext<'a, Data>,
    ) -> tide::Response {
        let path = ctx.req.uri().path().to_owned();
        let method = ctx.req.method().as_str().to_owned();
        trace!("IN => {} {}", method, path);
        let start = std::time::Instant::now();
        let res = await!(ctx.next());
        let status = res.status();
        if self.timer_on {
            info!(
                "{} {} {} {}ms",
                method,
                path,
                status.as_str(),
                start.elapsed().as_millis()
            );
        } else {
            info!("{} {} {}", method, path, status.as_str());
        }
        res
    }
}

impl<Data: Clone + Send> Middleware<Data> for RequestLogger {
    fn handle<'a>(&'a self, ctx: RequestContext<'a, Data>) -> FutureObj<'a, tide::Response> {
        FutureObj::new(Box::new(self.log_basic(ctx)))
    }
}
