//! Base constructs for the typed REST Params that will be used in their respective
//! REST handler functions (e.g., `agent_rest::create_agent` and `agent_rest::list_agents`).
//!
//! These structures are designed to work with Axum's extractors for RESTful APIs.

use modql::filter::ListOptions;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_with::{serde_as, OneOrMany};

/// Path parameter for entity ID
/// Used with `Path<PathId>` extractor for routes like `/:id`
#[derive(Debug, Deserialize)]
pub struct PathId {
	pub id: i64,
}

/// Query params for list operations
/// Used with `Query<QueryList<F>>` extractor for filtering and pagination
#[serde_as]
#[derive(Debug, Deserialize, Default)]
pub struct QueryList<F>
where
	F: DeserializeOwned,
{
	#[serde_as(deserialize_as = "Option<OneOrMany<_>>")]
	#[serde(default)]
	pub filters: Option<Vec<F>>,
	#[serde(flatten)]
	pub list_options: Option<ListOptions>,
}
