use futures::future::FutureObj;
use log::{info, trace};
use tide::{
    middleware::{Middleware, Next},
    Context,
};

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

    async fn log_basic<'a, Data: Send + Sync + 'static>(
        &'a self,
        ctx: Context<Data>,
        next: Next<'a, Data>,
    ) -> tide::Response {
        // TODO: Think of how to abstrct this cleanly, so it
        // can be reused for different config like timer, and
        // custom formatter
        let path = ctx.uri().path().to_owned();
        let method = ctx.method().as_str().to_owned();
        trace!("IN => {} {}", method, path);
        let start = std::time::Instant::now();
        let res = await!(next.run(ctx));
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

impl<Data: Send + Sync + 'static> Middleware<Data> for RequestLogger {
    fn handle<'a>(
        &'a self,
        ctx: Context<Data>,
        next: Next<'a, Data>,
    ) -> FutureObj<'a, tide::Response> {
        box_async! { await!(self.log_basic(ctx, next)) }
    }
}
