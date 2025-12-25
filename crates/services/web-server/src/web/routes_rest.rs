use crate::web::rests::all_rest_router;
use axum::Router;
use lib_core::model::ModelManager;

/// Build the Axum router for '/api/rest'
pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.nest("/rest", all_rest_router())
		.with_state(mm)
}
