use std::error::Error;

pub type Result<T> = std::result::Result<T, ArticleSearchServiceError>;

#[derive(Debug, thiserror::Error)]
pub enum ArticleSearchServiceError {
    #[error("Internal error: {0}")]
    Internal(Box<dyn Error + 'static>),
    #[error("Keyword is invalid. Keyword can only contain alphabets and numbers.\nExample:\n  search word2vec")]
    InvalidArgument,
}

impl From<std::io::Error> for ArticleSearchServiceError {
    fn from(err: std::io::Error) -> Self {
        ArticleSearchServiceError::Internal(err.into())
    }
}