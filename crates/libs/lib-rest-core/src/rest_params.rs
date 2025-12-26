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

/// Query params for list operations with pagination support
/// Used with `Query<QueryList<F>>` extractor for filtering and pagination
///
/// Example URL: `/api/rest/agents?page_size=10&page_number=1&order_bys=id`
#[serde_as]
#[derive(Debug, Deserialize, Default)]
pub struct QueryList<F>
where
	F: DeserializeOwned,
{
	/// Filters for the list query
	#[serde_as(deserialize_as = "Option<OneOrMany<_>>")]
	#[serde(default)]
	pub filters: Option<Vec<F>>,

	/// Number of items per page (default: 20, max: 1000)
	#[serde(default)]
	pub page_size: Option<i64>,

	/// Page number, starting from 1 (default: 1)
	#[serde(default)]
	pub page_number: Option<i64>,

	/// Order by fields (e.g., "id", "-created_at" for descending)
	#[serde(default)]
	pub order_bys: Option<String>,
}

impl<F> QueryList<F>
where
	F: DeserializeOwned,
{
	/// Get page size with default value
	pub fn get_page_size(&self) -> i64 {
		self.page_size.unwrap_or(20).min(1000).max(1)
	}

	/// Get page number with default value (1-based)
	pub fn get_page_number(&self) -> i64 {
		self.page_number.unwrap_or(1).max(1)
	}

	/// Convert pagination params to ListOptions
	pub fn into_list_options(&self) -> Option<ListOptions> {
		let page_size = self.get_page_size();
		let page_number = self.get_page_number();
		let offset = (page_number - 1) * page_size;

		Some(ListOptions {
			limit: Some(page_size),
			offset: Some(offset),
			order_bys: self.order_bys.as_ref().map(|s| s.as_str().into()),
		})
	}
}
