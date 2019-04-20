#![feature(futures_api, async_await)]

use http_service::Body;
use http_service_mock::make_server;

#[allow(unused_imports)]
use surf::middlewares;
#[allow(unused_imports)]
use tide::App;

#[test]
fn create_app() {
    let mut app = surf::minimal(());
    app.at("/").get(async move |_| "Hello world!");

    let mut server = make_server(app.into_http_service()).unwrap();

    let req = http::Request::get("/").body(Body::empty()).unwrap();

    let res = server.simulate(req).unwrap();
    assert_eq!(res.status(), 200);
}
