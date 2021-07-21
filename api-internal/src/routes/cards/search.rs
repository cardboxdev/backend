use crate::generated::{
    components::{
        request_bodies::CardsSearchRequestBody, responses::CardsSearchSuccess, schemas::Card,
    },
    paths::cards_search::{Error, Response},
};
use actix_web::web::{self, Data};
use cardbox_core::app::{CardSearchError, Cards};

pub async fn route(
    app: Data<cardbox_app::App>,
    search: web::Json<CardsSearchRequestBody>,
) -> Result<Response, Error> {
    let body = search.into_inner();

    let search_results = app
        .card_search(&body.search)
        .await
        .map_err(map_card_search_error)?;

    let total = search_results.len();

    Ok(Response::Ok(CardsSearchSuccess {
        cards: search_results
            .into_iter()
            .map(|c| Card {
                id: c.id,
                title: c.title,
                content: c.contents,
                created_at: c.created_at,
                updated_at: c.updated_at,
                author_id: c.author_id,
                tags: c.tags,
            })
            .collect(),
        total,
    }))
}

fn map_card_search_error(error: CardSearchError) -> Error {
    use CardSearchError::*;

    match error {
        Unexpected(e) => Error::InternalServerError(e),
    }
}
