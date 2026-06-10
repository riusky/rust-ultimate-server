//! Base constructs for the typed RPC Params that will be used in their respective
//! rpc handler functions (e.g., `project_rpc::create_project` and `project_rpc::list_projects`).
//!
//! Most of these base constructs use generics for their respective data elements, allowing
//! each rpc handler function to receive the exact desired type.
//!
//! `IntoParams` or `IntoDefaultRpcParams` are implemented to ensure these Params conform to the
//! `RpcRouter` (i.e., `rpc::router`) model.

use modql::filter::ListOptions;
use rpc_router::{IntoDefaultRpcParams, IntoParams};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_with::{serde_as, OneOrMany};

use lib_core::model::cache::CachePolicy;

/// Params structure for any RPC Create call.
#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
	pub data: D,
}

impl<D> IntoParams for ParamsForCreate<D> where D: DeserializeOwned + Send {}

/// Params structure for any RPC Update call.
#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
	pub id: i64,
	pub data: D,
}

impl<D> IntoParams for ParamsForUpdate<D> where D: DeserializeOwned + Send {}

/// Params structure for any RPC Update call.
#[derive(Deserialize)]
pub struct ParamsIded {
	pub id: i64,
	#[serde(default)]
	pub cache_policy: Option<CachePolicy>,
}

impl ParamsIded {
	pub fn cache_policy(&self) -> CachePolicy {
		self.cache_policy.unwrap_or_default()
	}
}
impl IntoParams for ParamsIded {}

/// Params structure for any RPC List call.
#[serde_as]
#[derive(Deserialize, Default)]
pub struct ParamsList<F>
where
	F: DeserializeOwned,
{
	#[serde_as(deserialize_as = "Option<OneOrMany<_>>")]
	pub filters: Option<Vec<F>>,
	pub list_options: Option<ListOptions>,
	#[serde(default)]
	pub cache_policy: Option<CachePolicy>,
}

impl<F> ParamsList<F>
where
	F: DeserializeOwned,
{
	pub fn cache_policy(&self) -> CachePolicy {
		self.cache_policy.unwrap_or_default()
	}
}

impl<D> IntoDefaultRpcParams for ParamsList<D> where D: DeserializeOwned + Send + Default {}
