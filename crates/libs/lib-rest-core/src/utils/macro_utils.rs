//! Macro utilities for generating common REST CRUD handler functions.
//!
//! The `generate_common_rest_fns!` macro creates the following REST endpoints:
//! - `create_...` (POST /)
//! - `get_...`    (GET /:id)
//! - `list_...s`  (GET /)
//! - `update_...` (PUT /:id)
//! - `delete_...` (DELETE /:id)
//! - `rest_router_...()` - Returns an Axum Router for the entity

/// Create the base CRUD REST handler functions following the common pattern.
///
/// # Example
/// ```ignore
/// generate_common_rest_fns!(
///     Bmc: AgentBmc,
///     Entity: Agent,
///     ForCreate: AgentForCreate,
///     ForUpdate: AgentForUpdate,
///     Filter: AgentFilter,
///     Suffix: agent
/// );
/// ```
///
/// This will generate:
/// - `create_agent` - POST handler
/// - `get_agent` - GET by ID handler
/// - `list_agents` - GET list handler
/// - `update_agent` - PUT handler
/// - `delete_agent` - DELETE handler
/// - `rest_router_agent()` - Returns Router with all routes configured
///
/// NOTE: Make sure to import the necessary types in the module that uses this macro.
#[macro_export]
macro_rules! generate_common_rest_fns {
	(
        Bmc: $bmc:ident,
        Entity: $entity:ty,
        ForCreate: $for_create:ty,
        ForUpdate: $for_update:ty,
        Filter: $filter:ty,
        Suffix: $suffix:ident
    ) => {
		paste! {
			/// POST /
			/// Create a new entity
			pub async fn [<create_ $suffix>](
				ctx: CtxW,
				State(mm): State<ModelManager>,
				Json(data): Json<$for_create>,
			) -> Result<RestCreated<$entity>> {
				let ctx = ctx.0;
				let id = $bmc::create(&ctx, &mm, data).await?;
				let entity = $bmc::get(&ctx, &mm, id).await?;
				Ok(entity.into())
			}

			/// GET /:id
			/// Get an entity by ID
			pub async fn [<get_ $suffix>](
				ctx: CtxW,
				State(mm): State<ModelManager>,
				Path(PathId { id }): Path<PathId>,
			) -> Result<RestResponse<$entity>> {
				let ctx = ctx.0;
				let entity = $bmc::get(&ctx, &mm, id).await?;
				Ok(entity.into())
			}

			/// GET /
			/// List entities with optional filters
			pub async fn [<list_ $suffix s>](
				ctx: CtxW,
				State(mm): State<ModelManager>,
				Query(params): Query<QueryList<$filter>>,
			) -> Result<RestResponse<Vec<$entity>>> {
				let ctx = ctx.0;
				let entities = $bmc::list(&ctx, &mm, params.filters, params.list_options).await?;
				Ok(entities.into())
			}

			/// PUT /:id
			/// Update an entity
			pub async fn [<update_ $suffix>](
				ctx: CtxW,
				State(mm): State<ModelManager>,
				Path(PathId { id }): Path<PathId>,
				Json(data): Json<$for_update>,
			) -> Result<RestResponse<$entity>> {
				let ctx = ctx.0;
				$bmc::update(&ctx, &mm, id, data).await?;
				let entity = $bmc::get(&ctx, &mm, id).await?;
				Ok(entity.into())
			}

			/// DELETE /:id
			/// Delete an entity
			pub async fn [<delete_ $suffix>](
				ctx: CtxW,
				State(mm): State<ModelManager>,
				Path(PathId { id }): Path<PathId>,
			) -> Result<RestDeleted<$entity>> {
				let ctx = ctx.0;
				let entity = $bmc::get(&ctx, &mm, id).await?;
				$bmc::delete(&ctx, &mm, id).await?;
				Ok(entity.into())
			}

			/// Build the REST router for this entity
			/// Returns an Axum Router with all CRUD routes configured
			pub fn [<rest_router_ $suffix>]() -> axum::Router<ModelManager> {
				use axum::routing::{get, post, put, delete};
				axum::Router::new()
					.route("/", post([<create_ $suffix>]).get([<list_ $suffix s>]))
					.route("/{id}", get([<get_ $suffix>]).put([<update_ $suffix>]).delete([<delete_ $suffix>]))
			}
		}
	};
}
