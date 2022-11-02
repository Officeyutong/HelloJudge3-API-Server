pub type RouteResult<T> = std::result::Result<T, MyError>;

pub struct MyError(anyhow::Error);

impl actix_web::error::ResponseError for MyError {}
impl<E> From<E> for MyError
where
    E: std::error::Error + Sync + Send + 'static,
{
    fn from(e: E) -> Self {
        Self(e.into())
    }
}

impl std::fmt::Debug for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
