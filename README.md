# Surf

Trying to keep up with the tide.

Helpers and middleware collection for [tide](https://github.com/rustasync/tide)

### Current features

- Logging middleware
- CORS Blanket middleware

### Notes

- `middleware::log::RequestLogger`

Simple logger that doesn't take any dependencies other than the `log` crate.

```rust
use surf::middlewares;
app.middleware(middlewares::log::RequestLogger::new().timed());
```

- `middleware::cors::CorsBlanket`

A blanket middleware for Cors.

```rust
use surf::middlewares;
app.middleware(middlewares::cors::CorsBlanket::new()
     .origin("https://surf-with-the-tide")
     .max_age("600"));
```
