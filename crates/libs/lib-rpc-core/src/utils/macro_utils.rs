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
///     ResourceGroup: "Agent Management"
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
        ResourceGroup: $group:literal
    ) => {
        // Register CRUD permissions (resource name = stringify!($suffix))
        ::lib_core::register_crud_permissions!(stringify!($suffix), $display, $group);

        // Register route handlers for startup validation
        ::lib_core::register_crud_handlers!(stringify!($suffix));

        paste! {
            pub async fn [<create_ $suffix>](
                ctx: Ctx,
                mm: ModelManager,
                params: ParamsForCreate<$for_create>,
            ) -> Result<DataRpcResult<$entity>> {
                ctx.require_permission(concat!(stringify!($suffix), ":create"))?;
                let ParamsForCreate { data } = params;
                let id = $bmc::create(&ctx, &mm, data).await?;
                let entity = $bmc::get(&ctx, &mm, id).await?;
                Ok(entity.into())
            }

            pub async fn [<get_ $suffix>](
                ctx: Ctx,
                mm: ModelManager,
                params: ParamsIded,
            ) -> Result<DataRpcResult<$entity>> {
                ctx.require_permission(concat!(stringify!($suffix), ":read"))?;
                let entity = $bmc::get(&ctx, &mm, params.id).await?;
                Ok(entity.into())
            }

            // Note: for now just add `s` after the suffix.
            pub async fn [<list_ $suffix s>](
                ctx: Ctx,
                mm: ModelManager,
                params: ParamsList<$filter>,
            ) -> Result<DataRpcResult<Vec<$entity>>> {
                ctx.require_permission(concat!(stringify!($suffix), ":list"))?;
                let entities = $bmc::list(&ctx, &mm, params.filters, params.list_options).await?;
                Ok(entities.into())
            }

            pub async fn [<update_ $suffix>](
                ctx: Ctx,
                mm: ModelManager,
                params: ParamsForUpdate<$for_update>,
            ) -> Result<DataRpcResult<$entity>> {
                ctx.require_permission(concat!(stringify!($suffix), ":update"))?;
                let ParamsForUpdate { id, data } = params;
                $bmc::update(&ctx, &mm, id, data).await?;
                let entity = $bmc::get(&ctx, &mm, id).await?;
                Ok(entity.into())
            }

            pub async fn [<delete_ $suffix>](
                ctx: Ctx,
                mm: ModelManager,
                params: ParamsIded,
            ) -> Result<DataRpcResult<$entity>> {
                ctx.require_permission(concat!(stringify!($suffix), ":delete"))?;
                let ParamsIded { id } = params;
                let entity = $bmc::get(&ctx, &mm, id).await?;
                $bmc::delete(&ctx, &mm, id).await?;
                Ok(entity.into())
            }
        }
    };
}
