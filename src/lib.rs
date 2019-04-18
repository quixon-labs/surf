#![feature(futures_api, async_await, await_macro)]
#![forbid(rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]

#[macro_use]
mod macros;

pub mod middlewares;

use tide::App;

pub fn minimal<AppData>(data: AppData) -> App<AppData>
where
    AppData: Sync + Send + 'static,
{
    let mut app = App::new(data);
    app.middleware(middlewares::log::RequestLogger::new().timed());
    app
}
