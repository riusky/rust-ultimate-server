/// Convenience macro rules to generate default CRUD functions for a Bmc/Entity.
/// Note: If custom functionality is required, use the code below as foundational
///       code for the custom implementations.
#[macro_export]
macro_rules! generate_common_bmc_fns {
	(
		Bmc: $struct_name:ident,
		Entity: $entity:ty,
		$(ForCreate: $for_create:ty,)?
		$(ForUpdate: $for_update:ty,)?
		$(Filter: $filter:ty,)?
	) => {
		impl $struct_name {
			$(
				pub async fn create(
					ctx: &Ctx,
					mm: &ModelManager,
					entity_c: $for_create,
				) -> Result<i64> {
					base::create::<Self, _>(ctx, mm, entity_c).await
				}

				pub async fn create_many(
					ctx: &Ctx,
					mm: &ModelManager,
					entity_c: Vec<$for_create>,
				) -> Result<Vec<i64>> {
					base::create_many::<Self, _>(ctx, mm, entity_c).await
				}
			)?

				pub async fn get(
					ctx: &Ctx,
					mm: &ModelManager,
					id: i64,
				) -> Result<$entity> {
					base::get::<Self, _>(ctx, mm, id).await
				}

			$(
				pub async fn first(
					ctx: &Ctx,
					mm: &ModelManager,
					filter: Option<Vec<$filter>>,
					list_options: Option<ListOptions>,
				) -> Result<Option<$entity>> {
					base::first::<Self, _, _>(ctx, mm, filter, list_options).await
				}

				pub async fn list(
					ctx: &Ctx,
					mm: &ModelManager,
					filter: Option<Vec<$filter>>,
					list_options: Option<ListOptions>,
				) -> Result<Vec<$entity>> {
					base::list::<Self, _, _>(ctx, mm, filter, list_options).await
				}

				pub async fn count(
					ctx: &Ctx,
					mm: &ModelManager,
					filter: Option<Vec<$filter>>,
				) -> Result<i64> {
					base::count::<Self, _>(ctx, mm, filter).await
				}
			)?

			$(
				pub async fn update(
					ctx: &Ctx,
					mm: &ModelManager,
					id: i64,
					entity_u: $for_update,
				) -> Result<()> {
					base::update::<Self, _>(ctx, mm, id, entity_u).await
				}
			)?

				pub async fn delete(
					ctx: &Ctx,
					mm: &ModelManager,
					id: i64,
				) -> Result<()> {
					base::delete::<Self>(ctx, mm, id).await
				}

				pub async fn delete_many(
					ctx: &Ctx,
					mm: &ModelManager,
					ids: Vec<i64>,
				) -> Result<u64> {
					base::delete_many::<Self>(ctx, mm, ids).await
				}
		}
	};
}

/// Generate cache-aside CRUD functions for a BMC.
///
/// These functions keep the existing CRUD methods intact and add `*_cached`
/// variants that read/write Redis when the `ModelManager` has a Valkey pool.
#[macro_export]
macro_rules! generate_cached_bmc_fns {
	(
		Bmc: $struct_name:ident,
		Entity: $entity:ty,
		$(ForCreate: $for_create:ty,)?
		$(ForUpdate: $for_update:ty,)?
		$(Filter: $filter:ty,)?
	) => {
		impl $struct_name {
			$(
				pub async fn create_cached(
					ctx: &$crate::ctx::Ctx,
					mm: &$crate::model::ModelManager,
					entity_c: $for_create,
				) -> $crate::model::Result<i64>
				where
					$entity: serde::Serialize,
				{
					let cache_pool = mm.valkey_pool();
					let id = Self::create(ctx, mm, entity_c).await?;

					match Self::get(ctx, mm, id).await {
						Ok(entity) => {
							$crate::model::cache::write_model_entity_best_effort(
								cache_pool,
								<Self as $crate::model::DbBmc>::TABLE,
								id,
								&entity,
							)
							.await;
						}
						Err(err) => {
							tracing::warn!(
								table = <Self as $crate::model::DbBmc>::TABLE,
								id,
								error = ?err,
								"model cache refresh after create failed"
							);
						}
					}

					$crate::model::cache::bump_model_query_version_best_effort(
						cache_pool,
						<Self as $crate::model::DbBmc>::TABLE,
					)
					.await;

					Ok(id)
				}

				pub async fn create_many_cached(
					ctx: &$crate::ctx::Ctx,
					mm: &$crate::model::ModelManager,
					entity_c: Vec<$for_create>,
				) -> $crate::model::Result<Vec<i64>>
				where
					$entity: serde::Serialize,
				{
					let cache_pool = mm.valkey_pool();
					let ids = Self::create_many(ctx, mm, entity_c).await?;

					for id in &ids {
						match Self::get(ctx, mm, *id).await {
							Ok(entity) => {
								$crate::model::cache::write_model_entity_best_effort(
									cache_pool,
									<Self as $crate::model::DbBmc>::TABLE,
									*id,
									&entity,
								)
								.await;
							}
							Err(err) => {
								tracing::warn!(
									table = <Self as $crate::model::DbBmc>::TABLE,
									id = *id,
									error = ?err,
									"model cache refresh after create_many failed"
								);
							}
						}
					}

					$crate::model::cache::bump_model_query_version_best_effort(
						cache_pool,
						<Self as $crate::model::DbBmc>::TABLE,
					)
					.await;

					Ok(ids)
				}
			)?

			pub async fn get_cached(
				ctx: &$crate::ctx::Ctx,
				mm: &$crate::model::ModelManager,
				id: i64,
			) -> $crate::model::Result<$entity>
			where
				$entity: serde::Serialize + serde::de::DeserializeOwned,
			{
				let cache_pool = mm.valkey_pool();
				let key = $crate::model::cache::model_entity_key(
					<Self as $crate::model::DbBmc>::TABLE,
					id,
				);

				$crate::model::cache::get_or_load_json(
					cache_pool,
					Some(&key),
					Some($crate::model::cache::MODEL_ENTITY_TTL_SECS),
					|| async { Self::get(ctx, mm, id).await },
				)
				.await
			}

			$(
				pub async fn first_cached(
					ctx: &$crate::ctx::Ctx,
					mm: &$crate::model::ModelManager,
					filter: Option<Vec<$filter>>,
					list_options: Option<modql::filter::ListOptions>,
				) -> $crate::model::Result<Option<$entity>>
				where
					$entity: serde::Serialize + serde::de::DeserializeOwned,
					$filter: serde::Serialize,
				{
					let cache_pool = mm.valkey_pool();
					let version = $crate::model::cache::model_query_version(
						cache_pool,
						<Self as $crate::model::DbBmc>::TABLE,
					)
					.await;
					let key = $crate::model::cache::model_query_key(
						<Self as $crate::model::DbBmc>::TABLE,
						"first",
						version,
						&(&filter, &list_options),
					);

					$crate::model::cache::get_or_load_json(
						cache_pool,
						key.as_deref(),
						Some($crate::model::cache::MODEL_QUERY_TTL_SECS),
						|| async { Self::first(ctx, mm, filter, list_options).await },
					)
					.await
				}

				pub async fn list_cached(
					ctx: &$crate::ctx::Ctx,
					mm: &$crate::model::ModelManager,
					filter: Option<Vec<$filter>>,
					list_options: Option<modql::filter::ListOptions>,
				) -> $crate::model::Result<Vec<$entity>>
				where
					$entity: serde::Serialize + serde::de::DeserializeOwned,
					$filter: serde::Serialize,
				{
					let cache_pool = mm.valkey_pool();
					let version = $crate::model::cache::model_query_version(
						cache_pool,
						<Self as $crate::model::DbBmc>::TABLE,
					)
					.await;
					let key = $crate::model::cache::model_query_key(
						<Self as $crate::model::DbBmc>::TABLE,
						"list",
						version,
						&(&filter, &list_options),
					);

					$crate::model::cache::get_or_load_json(
						cache_pool,
						key.as_deref(),
						Some($crate::model::cache::MODEL_QUERY_TTL_SECS),
						|| async { Self::list(ctx, mm, filter, list_options).await },
					)
					.await
				}

				pub async fn count_cached(
					ctx: &$crate::ctx::Ctx,
					mm: &$crate::model::ModelManager,
					filter: Option<Vec<$filter>>,
				) -> $crate::model::Result<i64>
				where
					$filter: serde::Serialize,
				{
					let cache_pool = mm.valkey_pool();
					let version = $crate::model::cache::model_query_version(
						cache_pool,
						<Self as $crate::model::DbBmc>::TABLE,
					)
					.await;
					let key = $crate::model::cache::model_query_key(
						<Self as $crate::model::DbBmc>::TABLE,
						"count",
						version,
						&filter,
					);

					$crate::model::cache::get_or_load_json(
						cache_pool,
						key.as_deref(),
						Some($crate::model::cache::MODEL_QUERY_TTL_SECS),
						|| async { Self::count(ctx, mm, filter).await },
					)
					.await
				}
			)?

			$(
				pub async fn update_cached(
					ctx: &$crate::ctx::Ctx,
					mm: &$crate::model::ModelManager,
					id: i64,
					entity_u: $for_update,
				) -> $crate::model::Result<()>
				where
					$entity: serde::Serialize,
				{
					let cache_pool = mm.valkey_pool();
					Self::update(ctx, mm, id, entity_u).await?;

					match Self::get(ctx, mm, id).await {
						Ok(entity) => {
							$crate::model::cache::write_model_entity_best_effort(
								cache_pool,
								<Self as $crate::model::DbBmc>::TABLE,
								id,
								&entity,
							)
							.await;
						}
						Err(err) => {
							tracing::warn!(
								table = <Self as $crate::model::DbBmc>::TABLE,
								id,
								error = ?err,
								"model cache refresh after update failed"
							);
							$crate::model::cache::delete_model_entity_best_effort(
								cache_pool,
								<Self as $crate::model::DbBmc>::TABLE,
								id,
							)
							.await;
						}
					}

					$crate::model::cache::bump_model_query_version_best_effort(
						cache_pool,
						<Self as $crate::model::DbBmc>::TABLE,
					)
					.await;

					Ok(())
				}
			)?

			pub async fn delete_cached(
				ctx: &$crate::ctx::Ctx,
				mm: &$crate::model::ModelManager,
				id: i64,
			) -> $crate::model::Result<()> {
				let cache_pool = mm.valkey_pool();
				Self::delete(ctx, mm, id).await?;

				$crate::model::cache::delete_model_entity_best_effort(
					cache_pool,
					<Self as $crate::model::DbBmc>::TABLE,
					id,
				)
				.await;
				$crate::model::cache::bump_model_query_version_best_effort(
					cache_pool,
					<Self as $crate::model::DbBmc>::TABLE,
				)
				.await;

				Ok(())
			}

			pub async fn delete_many_cached(
				ctx: &$crate::ctx::Ctx,
				mm: &$crate::model::ModelManager,
				ids: Vec<i64>,
			) -> $crate::model::Result<u64> {
				let cache_pool = mm.valkey_pool();
				let deleted = Self::delete_many(ctx, mm, ids.clone()).await?;

				$crate::model::cache::delete_model_entities_best_effort(
					cache_pool,
					<Self as $crate::model::DbBmc>::TABLE,
					&ids,
				)
				.await;
				$crate::model::cache::bump_model_query_version_best_effort(
					cache_pool,
					<Self as $crate::model::DbBmc>::TABLE,
				)
				.await;

				Ok(deleted)
			}
		}
	};
}
