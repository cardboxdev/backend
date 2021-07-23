use crate::contracts::UnexpectedDatabaseError;
use crate::models;
use futures::stream::BoxStream;

#[async_trait]
pub trait Cards {
    async fn card_create(
        &self,
        card: CardCreateForm,
        token: String,
    ) -> Result<models::Card, CardCreateError>;

    fn cards_search<'a>(
        &'a self,
        query: &str,
        limit: Option<i64>,
    ) -> Result<BoxStream<'a, Result<(models::Card, models::User), CardSearchError>>, eyre::Report>;
}

#[derive(Debug, Validate)]
pub struct CardCreateForm {
    #[validate(length(min = 1))]
    pub title: String,
    pub contents: serde_json::Value,
    pub tags: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum CardCreateError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    // BL domain where we can later add more errors
    // Maybe user exceeded his limit on cards
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

#[derive(Debug, thiserror::Error)]
pub enum CardSearchError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

impl From<UnexpectedDatabaseError> for CardCreateError {
    #[inline]
    fn from(e: UnexpectedDatabaseError) -> Self {
        Self::Unexpected(e.into())
    }
}

impl From<UnexpectedDatabaseError> for CardSearchError {
    #[inline]
    fn from(e: UnexpectedDatabaseError) -> Self {
        Self::Unexpected(e.into())
    }
}