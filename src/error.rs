#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("This is a test error: {0}")]
    TestError(String),
}
