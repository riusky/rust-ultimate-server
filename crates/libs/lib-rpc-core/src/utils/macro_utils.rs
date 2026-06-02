/// Create the base crud rpc functions following the common pattern.
/// - `create_...`
/// - `get_...`
/// - `list_...s`
/// - `update_...`
/// - `delete_...`
///
/// Also registers CRUD permissions for the resource.
/// The resource name is derived from the `Suffix` parameter.
///
/// # Example
/// ```ignore
/// generate_common_rpc_fns!(
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
/// NOTE: Make sure to import the Ctx, ModelManager, ... in the model that uses this macro.
#[macro_export]
macro_rules! generate_common_rpc_fns {
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
        $crate::generate_common_rpc_fns!(
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
        $crate::generate_common_rpc_fns!(
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
        $crate::generate_common_rpc_fns!(
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
        $crate::generate_common_rpc_fns!(@register_cache $cache, $bmc, $suffix);

        paste! {
            pub async fn [<create_ $suffix>](
                ctx: Ctx,
                mm: ModelManager,
                params: ParamsForCreate<$for_create>,
            ) -> Result<DataRpcResult<$entity>> {
                ctx.require_permission(concat!(stringify!($suffix), ":create"))?;
                let ParamsForCreate { data } = params;
                let id = $crate::generate_common_rpc_fns!(@create $cache, $bmc, &ctx, &mm, data).await?;
                let entity = $crate::generate_common_rpc_fns!(@get $cache, $bmc, &ctx, &mm, id).await?;
                Ok(entity.into())
            }

            pub async fn [<get_ $suffix>](
                ctx: Ctx,
                mm: ModelManager,
                params: ParamsIded,
            ) -> Result<DataRpcResult<$entity>> {
                ctx.require_permission(concat!(stringify!($suffix), ":read"))?;
                let entity = $crate::generate_common_rpc_fns!(@get $cache, $bmc, &ctx, &mm, params.id).await?;
                Ok(entity.into())
            }

            // Note: for now just add `s` after the suffix.
            pub async fn [<list_ $suffix s>](
                ctx: Ctx,
                mm: ModelManager,
                params: ParamsList<$filter>,
            ) -> Result<DataRpcResult<Vec<$entity>>> {
                ctx.require_permission(concat!(stringify!($suffix), ":list"))?;
                let entities = $crate::generate_common_rpc_fns!(@list $cache, $bmc, &ctx, &mm, params.filters, params.list_options).await?;
                Ok(entities.into())
            }

            pub async fn [<update_ $suffix>](
                ctx: Ctx,
                mm: ModelManager,
                params: ParamsForUpdate<$for_update>,
            ) -> Result<DataRpcResult<$entity>> {
                ctx.require_permission(concat!(stringify!($suffix), ":update"))?;
                let ParamsForUpdate { id, data } = params;
                $crate::generate_common_rpc_fns!(@update $cache, $bmc, &ctx, &mm, id, data).await?;
                let entity = $crate::generate_common_rpc_fns!(@get $cache, $bmc, &ctx, &mm, id).await?;
                Ok(entity.into())
            }

            pub async fn [<delete_ $suffix>](
                ctx: Ctx,
                mm: ModelManager,
                params: ParamsIded,
            ) -> Result<DataRpcResult<$entity>> {
                ctx.require_permission(concat!(stringify!($suffix), ":delete"))?;
                let ParamsIded { id } = params;
                let entity = $crate::generate_common_rpc_fns!(@get $cache, $bmc, &ctx, &mm, id).await?;
                $crate::generate_common_rpc_fns!(@delete $cache, $bmc, &ctx, &mm, id).await?;
                Ok(entity.into())
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
    (@get true, $bmc:ident, $ctx:expr, $mm:expr, $id:expr) => {
        $bmc::get_cached($ctx, $mm, $id)
    };
    (@get false, $bmc:ident, $ctx:expr, $mm:expr, $id:expr) => {
        $bmc::get($ctx, $mm, $id)
    };
    (@list true, $bmc:ident, $ctx:expr, $mm:expr, $filter:expr, $list_options:expr) => {
        $bmc::list_cached($ctx, $mm, $filter, $list_options)
    };
    (@list false, $bmc:ident, $ctx:expr, $mm:expr, $filter:expr, $list_options:expr) => {
        $bmc::list($ctx, $mm, $filter, $list_options)
    };
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
