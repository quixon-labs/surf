#![feature(futures_api, async_await, await_macro)]

mod cors;

pub use cors::CorsBlanketMiddleware;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
