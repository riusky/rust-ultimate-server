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
//!
//! Also registers CRUD permissions for the resource.
//! The resource name is derived from the `Suffix` parameter.

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
///     Suffix: agent,
///     ResourceDisplay: "Agent",
///     ResourceGroup: "Agent Management",
///     ResourceDescription: "agent entity"
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
/// And registers these permissions:
/// - `agent:create`, `agent:read`, `agent:update`, `agent:delete`, `agent:list`
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
        Suffix: $suffix:ident,
        ResourceDisplay: $display:literal,
        ResourceGroup: $group:literal,
        ResourceDescription: $desc:literal,
        Cache: true $(,)?
    ) => {
		$crate::generate_common_rest_fns!(
			@impl
			Bmc: $bmc,
			Entity: $entity,
			ForCreate: $for_create,
			ForUpdate: $for_update,
			Filter: $filter,
			Suffix: $suffix,
			ResourceDisplay: $display,
			ResourceGroup: $group,
			ResourceDescription: $desc,
			Cache: true
		);
	};
	(
        Bmc: $bmc:ident,
        Entity: $entity:ty,
        ForCreate: $for_create:ty,
        ForUpdate: $for_update:ty,
        Filter: $filter:ty,
        Suffix: $suffix:ident,
        ResourceDisplay: $display:literal,
        ResourceGroup: $group:literal,
        ResourceDescription: $desc:literal,
        Cache: false $(,)?
    ) => {
		$crate::generate_common_rest_fns!(
			@impl
			Bmc: $bmc,
			Entity: $entity,
			ForCreate: $for_create,
			ForUpdate: $for_update,
			Filter: $filter,
			Suffix: $suffix,
			ResourceDisplay: $display,
			ResourceGroup: $group,
			ResourceDescription: $desc,
			Cache: false
		);
	};
	(
        Bmc: $bmc:ident,
        Entity: $entity:ty,
        ForCreate: $for_create:ty,
        ForUpdate: $for_update:ty,
        Filter: $filter:ty,
        Suffix: $suffix:ident,
        ResourceDisplay: $display:literal,
        ResourceGroup: $group:literal,
        ResourceDescription: $desc:literal $(,)?
    ) => {
		$crate::generate_common_rest_fns!(
			Bmc: $bmc,
			Entity: $entity,
			ForCreate: $for_create,
			ForUpdate: $for_update,
			Filter: $filter,
			Suffix: $suffix,
			ResourceDisplay: $display,
			ResourceGroup: $group,
			ResourceDescription: $desc,
			Cache: false
		);
	};
	(
		@impl
        Bmc: $bmc:ident,
        Entity: $entity:ty,
        ForCreate: $for_create:ty,
        ForUpdate: $for_update:ty,
        Filter: $filter:ty,
        Suffix: $suffix:ident,
        ResourceDisplay: $display:literal,
        ResourceGroup: $group:literal,
        ResourceDescription: $desc:literal,
		Cache: $cache:tt
    ) => {
		::lib_core::register_crud_permissions!(stringify!($suffix), $display, $group, $desc);
		::lib_core::register_crud_handlers!(stringify!($suffix));
		$crate::generate_common_rest_fns!(@register_cache $cache, $bmc, $suffix);

		// Register paged list handler
		::inventory::submit! {
			::lib_core::model::acs::RegisteredRouteHandler {
				name: concat!("list_", stringify!($suffix), "s_paged"),
				kind: ::lib_core::model::acs::RouteHandlerKind::Protected,
				has_check: true, // Permission check is in the function body
				source: module_path!(),
			}
		}

		paste! {
			/// POST /
			/// Create a new entity
			pub async fn [<create_ $suffix>](
				ctx: CtxW,
				State(mm): State<ModelManager>,
				Json(data): Json<$for_create>,
			) -> Result<RestCreated<$entity>> {
				let ctx = ctx.0;
				ctx.require_permission(concat!(stringify!($suffix), ":create"))?;
				let id = $crate::generate_common_rest_fns!(@create $cache, $bmc, &ctx, &mm, data).await?;
				let entity = $crate::generate_common_rest_fns!(
					@get $cache,
					$bmc,
					&ctx,
					&mm,
					id,
					::lib_core::model::cache::CachePolicy::Refresh
				).await?;
				Ok(entity.into())
			}

			/// GET /:id
			/// Get an entity by ID
			pub async fn [<get_ $suffix>](
				ctx: CtxW,
				State(mm): State<ModelManager>,
				Path(PathId { id }): Path<PathId>,
				Query(cache_query): Query<CacheQuery>,
			) -> Result<RestResponse<$entity>> {
				let ctx = ctx.0;
				ctx.require_permission(concat!(stringify!($suffix), ":read"))?;
				let entity = $crate::generate_common_rest_fns!(
					@get $cache,
					$bmc,
					&ctx,
					&mm,
					id,
					cache_query.cache_policy()
				).await?;
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
				ctx.require_permission(concat!(stringify!($suffix), ":list"))?;
				let list_options = params.into_list_options();
				let cache_policy = params.cache_policy();
				let entities = $crate::generate_common_rest_fns!(
					@list $cache,
					$bmc,
					&ctx,
					&mm,
					params.filters,
					list_options,
					cache_policy
				).await?;
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
				ctx.require_permission(concat!(stringify!($suffix), ":list"))?;
				let page_size = params.get_page_size();
				let page_number = params.get_page_number();
				let list_options = params.into_list_options();
				let cache_policy = params.cache_policy();

				// Get total count
				let total = $crate::generate_common_rest_fns!(
					@count $cache,
					$bmc,
					&ctx,
					&mm,
					params.filters.clone(),
					cache_policy
				).await?;

				// Get paginated data
				let entities = $crate::generate_common_rest_fns!(
					@list $cache,
					$bmc,
					&ctx,
					&mm,
					params.filters,
					list_options,
					cache_policy
				).await?;

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
				ctx.require_permission(concat!(stringify!($suffix), ":update"))?;
				$crate::generate_common_rest_fns!(@update $cache, $bmc, &ctx, &mm, id, data).await?;
				let entity = $crate::generate_common_rest_fns!(
					@get $cache,
					$bmc,
					&ctx,
					&mm,
					id,
					::lib_core::model::cache::CachePolicy::Refresh
				).await?;
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
				ctx.require_permission(concat!(stringify!($suffix), ":delete"))?;
				let entity = $crate::generate_common_rest_fns!(
					@get $cache,
					$bmc,
					&ctx,
					&mm,
					id,
					::lib_core::model::cache::CachePolicy::Bypass
				).await?;
				$crate::generate_common_rest_fns!(@delete $cache, $bmc, &ctx, &mm, id).await?;
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
	(@register_cache true, $bmc:ident, $suffix:ident) => {
		::inventory::submit! {
			::lib_core::model::cache::RegisteredModelCache {
				resource: stringify!($suffix),
				table: <$bmc as ::lib_core::model::DbBmc>::TABLE,
				source: module_path!(),
			}
		}
	};
	(@register_cache false, $bmc:ident, $suffix:ident) => {};
	(@create true, $bmc:ident, $ctx:expr, $mm:expr, $data:expr) => {
		$bmc::create_cached($ctx, $mm, $data)
	};
	(@create false, $bmc:ident, $ctx:expr, $mm:expr, $data:expr) => {
		$bmc::create($ctx, $mm, $data)
	};
	(@get true, $bmc:ident, $ctx:expr, $mm:expr, $id:expr, $cache_policy:expr) => {
		$bmc::get_cached_with_policy($ctx, $mm, $id, $cache_policy)
	};
	(@get false, $bmc:ident, $ctx:expr, $mm:expr, $id:expr, $cache_policy:expr) => {{
		let _ = $cache_policy;
		$bmc::get($ctx, $mm, $id)
	}};
	(@list true, $bmc:ident, $ctx:expr, $mm:expr, $filter:expr, $list_options:expr, $cache_policy:expr) => {
		$bmc::list_cached_with_policy($ctx, $mm, $filter, $list_options, $cache_policy)
	};
	(@list false, $bmc:ident, $ctx:expr, $mm:expr, $filter:expr, $list_options:expr, $cache_policy:expr) => {{
		let _ = $cache_policy;
		$bmc::list($ctx, $mm, $filter, $list_options)
	}};
	(@count true, $bmc:ident, $ctx:expr, $mm:expr, $filter:expr, $cache_policy:expr) => {
		$bmc::count_cached_with_policy($ctx, $mm, $filter, $cache_policy)
	};
	(@count false, $bmc:ident, $ctx:expr, $mm:expr, $filter:expr, $cache_policy:expr) => {{
		let _ = $cache_policy;
		$bmc::count($ctx, $mm, $filter)
	}};
	(@update true, $bmc:ident, $ctx:expr, $mm:expr, $id:expr, $data:expr) => {
		$bmc::update_cached($ctx, $mm, $id, $data)
	};
	(@update false, $bmc:ident, $ctx:expr, $mm:expr, $id:expr, $data:expr) => {
		$bmc::update($ctx, $mm, $id, $data)
	};
	(@delete true, $bmc:ident, $ctx:expr, $mm:expr, $id:expr) => {
		$bmc::delete_cached($ctx, $mm, $id)
	};
	(@delete false, $bmc:ident, $ctx:expr, $mm:expr, $id:expr) => {
		$bmc::delete($ctx, $mm, $id)
	};
}
