#![feature(futures_api, async_await)]

use http_service_mock::make_server;
use http_service::Body;
use surf::middlewares;
use tide::App;

#[test]
fn run() {
    let mut app = App::new(());
    app.middleware(middlewares::log::RequestLogger::new().timed());
    app.at("/").get(async move |_| {
        "Hello world!"
    });
    
    let mut server = make_server(app.into_http_service()).unwrap();

    let req = http::Request::get("/")
        .body(Body::empty())
        .unwrap();

    let res = server.simulate(req).unwrap();
    assert_eq!(res.status(), 200);
}