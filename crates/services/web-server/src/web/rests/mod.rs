// region:    --- Modules

pub mod agent_rest;
pub mod conv_rest;

use axum::Router;
use lib_core::model::ModelManager;

// endregion: --- Modules

/// Build the combined REST router for all entities
pub fn all_rest_router() -> Router<ModelManager> {
	Router::new()
		.nest("/agents", agent_rest::rest_router_agent())
		.nest("/convs", conv_rest::rest_router_conv())
}
