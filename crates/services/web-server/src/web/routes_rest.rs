use crate::web::rests::all_rest_router;
use axum::middleware;
use axum::Router;
use lib_core::model::ModelManager;
use lib_web::middleware::mw_rest_error::mw_rest_error;

/// Build the Axum router for '/api/rest'
pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.nest("/rest", all_rest_router())
		.layer(middleware::from_fn(mw_rest_error))
		.with_state(mm)
}
