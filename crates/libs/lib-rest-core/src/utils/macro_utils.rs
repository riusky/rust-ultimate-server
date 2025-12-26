//! Macro utilities for generating common REST CRUD handler functions.
//!
//! The `generate_common_rest_fns!` macro creates the following REST endpoints:
//! - `create_...` (POST /)
//! - `get_...`    (GET /:id)
//! - `list_...s`  (GET /)
//! - `list_...s_paged` (GET /paged) - with pagination info
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
/// - `list_agents` - GET list handler (simple)
/// - `list_agents_paged` - GET list handler with pagination info
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
			/// List entities with optional filters and pagination
			/// Query params: ?limit=10&offset=0&order_bys=id
			pub async fn [<list_ $suffix s>](
				ctx: CtxW,
				State(mm): State<ModelManager>,
				Query(params): Query<QueryList<$filter>>,
			) -> Result<RestResponse<Vec<$entity>>> {
				let ctx = ctx.0;
				let list_options = params.into_list_options();
				let entities = $bmc::list(&ctx, &mm, params.filters, list_options).await?;
				Ok(entities.into())
			}

			/// GET /paged
			/// List entities with pagination info (total count, has_more, etc.)
			/// Query params: ?page_size=10&page_number=1&order_bys=id
			/// Response: { data: [...], page_info: { total, page_size, page_number, total_pages, has_more } }
			pub async fn [<list_ $suffix s_paged>](
				ctx: CtxW,
				State(mm): State<ModelManager>,
				Query(params): Query<QueryList<$filter>>,
			) -> Result<RestPagedResponse<$entity>> {
				let ctx = ctx.0;
				let page_size = params.get_page_size();
				let page_number = params.get_page_number();
				let list_options = params.into_list_options();

				// Get total count
				let total = $bmc::count(&ctx, &mm, params.filters.clone()).await?;

				// Get paginated data
				let entities = $bmc::list(&ctx, &mm, params.filters, list_options).await?;

				Ok(RestPagedResponse::new(entities, total, page_size, page_number))
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
			/// Routes:
			/// - POST /          - Create
			/// - GET /           - List (simple)
			/// - GET /paged      - List with pagination info
			/// - GET /:id        - Get by ID
			/// - PUT /:id        - Update
			/// - DELETE /:id     - Delete
			pub fn [<rest_router_ $suffix>]() -> axum::Router<ModelManager> {
				use axum::routing::{get, post, put, delete};
				axum::Router::new()
					.route("/", post([<create_ $suffix>]).get([<list_ $suffix s>]))
					.route("/paged", get([<list_ $suffix s_paged>]))
					.route("/{id}", get([<get_ $suffix>]).put([<update_ $suffix>]).delete([<delete_ $suffix>]))
			}
		}
	};
}
