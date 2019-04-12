#[macro_export]
macro_rules! box_async {
    {$($t:tt)*} => {
        FutureObj::new(Box::new(async move { $($t)* }))
    };
}
