//! Permission model for ACS (Access Control System)

use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::modql_utils::time_to_sea_value;
use crate::model::ModelManager;
use crate::model::Result;
use lib_utils::time::Rfc3339;
use modql::field::{Fields, HasSeaFields};
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use std::collections::HashSet;
use tracing::info;

#[cfg(feature = "with-ts")]
use ts_rs::TS;

// region:    --- Permission Types

#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
#[cfg_attr(feature = "with-ts", derive(TS))]
#[cfg_attr(feature = "with-ts", ts(export, export_to = "../../../../cmx-vue-ultimate-starter/src/types/generated/acs/"))]
pub struct Permission {
	pub id: i64,

	// -- Properties
	pub key: String,
	pub group_name: Option<String>,
	pub display_name: Option<String>,
	pub description: Option<String>,

	// -- Timestamps
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	#[cfg_attr(feature = "with-ts", ts(type = "string"))]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	#[cfg_attr(feature = "with-ts", ts(type = "string"))]
	pub mtime: OffsetDateTime,
}

/// Lightweight permission for listing
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
#[cfg_attr(feature = "with-ts", derive(TS))]
#[cfg_attr(feature = "with-ts", ts(export, export_to = "../../../../cmx-vue-ultimate-starter/src/types/generated/acs/"))]
pub struct PermissionLite {
	pub id: i64,
	pub key: String,
	pub group_name: Option<String>,
	pub display_name: Option<String>,
}

#[derive(Fields, Deserialize)]
pub struct PermissionForCreate {
	pub key: String,
	pub group_name: Option<String>,
	pub display_name: Option<String>,
	pub description: Option<String>,
}

#[derive(Fields, Deserialize)]
pub struct PermissionForUpdate {
	pub group_name: Option<String>,
	pub display_name: Option<String>,
	pub description: Option<String>,
}

#[derive(Clone, FilterNodes, Default, Deserialize)]
pub struct PermissionFilter {
	pub id: Option<OpValsInt64>,
	pub key: Option<OpValsString>,
	pub group_name: Option<OpValsString>,
	pub display_name: Option<OpValsString>,

	pub cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub ctime: Option<OpValsValue>,
	pub mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub mtime: Option<OpValsValue>,
}

#[derive(Iden)]
enum PermissionIden {
	Key,
}

// endregion: --- Permission Types

// region:    --- PermissionBmc

pub struct PermissionBmc;

impl DbBmc for PermissionBmc {
	const TABLE: &'static str = "permission";
}

impl PermissionBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		permission_c: PermissionForCreate,
	) -> Result<i64> {
		base::create::<Self, _>(ctx, mm, permission_c).await
	}

	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Permission> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn first_by_key(
		_ctx: &Ctx,
		mm: &ModelManager,
		key: &str,
	) -> Result<Option<Permission>> {
		// -- Build query
		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.columns(Permission::sea_column_refs())
			.and_where(Expr::col(PermissionIden::Key).eq(key));

		// -- Execute query
		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_as_with::<_, Permission, _>(&sql, values);
		let entity = mm.dbx().fetch_optional(sqlx_query).await?;

		Ok(entity)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<PermissionFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Permission>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}

	pub async fn list_lite(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<PermissionFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<PermissionLite>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		permission_u: PermissionForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, permission_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}

	pub async fn count(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<PermissionFilter>>,
	) -> Result<i64> {
		base::count::<Self, _>(ctx, mm, filter).await
	}

	// --- Helper methods for permission registry

	/// Get all permission keys
	pub async fn list_all_keys(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<String>> {
		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.column(PermissionIden::Key);

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_as_with::<_, (String,), _>(&sql, values);
		let rows = mm.dbx().fetch_all(sqlx_query).await?;

		Ok(rows.into_iter().map(|(key,)| key).collect())
	}

	/// Create or update permission from registered permission
	pub async fn upsert_from_registered(
		ctx: &Ctx,
		mm: &ModelManager,
		key: &str,
		group_name: &str,
		display_name: &str,
	) -> Result<i64> {
		// Check if permission already exists
		if let Some(existing) = Self::first_by_key(ctx, mm, key).await? {
			// Update if display info changed
			if existing.group_name.as_deref() != Some(group_name)
				|| existing.display_name.as_deref() != Some(display_name)
			{
				Self::update(
					ctx,
					mm,
					existing.id,
					PermissionForUpdate {
						group_name: Some(group_name.to_string()),
						display_name: Some(display_name.to_string()),
						description: None,
					},
				)
				.await?;
			}
			Ok(existing.id)
		} else {
			// Create new permission
			Self::create(
				ctx,
				mm,
				PermissionForCreate {
					key: key.to_string(),
					group_name: Some(group_name.to_string()),
					display_name: Some(display_name.to_string()),
					description: None,
				},
			)
			.await
		}
	}

	/// Sync permissions from code registry to database.
	/// - Creates new permissions that exist in code but not in database
	/// - Updates existing permissions if metadata changed
	/// - Deletes permissions that exist in database but not in code
	///   (also deletes related role_permission records)
	///
	/// Returns (created_count, updated_count, deleted_count)
	pub async fn sync_from_registry(
		ctx: &Ctx,
		mm: &ModelManager,
	) -> Result<(usize, usize, usize)> {
		use super::{RegisteredPermission, RolePermissionBmc};
		use std::collections::HashMap;

		// Get all registered permissions from code, deduplicated by key
		// (same key may be registered multiple times from different sources)
		let mut registered_map: HashMap<&str, &RegisteredPermission> = HashMap::new();
		for perm in inventory::iter::<RegisteredPermission> {
			// Keep the last registration for each key (or first, doesn't matter as they should be same)
			registered_map.insert(perm.key, perm);
		}
		let registered_keys: HashSet<&str> = registered_map.keys().copied().collect();

		// Get all permissions from database
		let db_permissions = Self::list(ctx, mm, None, None).await?;

		let mut created = 0;
		let mut updated = 0;
		let mut deleted = 0;

		// --- Insert or update permissions from code
		for (key, reg_perm) in &registered_map {
			if let Some(existing) = db_permissions.iter().find(|p| &p.key.as_str() == key) {
				// Check if update needed
				let desc_opt = if reg_perm.description.is_empty() { None } else { Some(reg_perm.description) };
				if existing.group_name.as_deref() != Some(reg_perm.group)
					|| existing.display_name.as_deref() != Some(reg_perm.display)
					|| existing.description.as_deref() != desc_opt
				{
					Self::update(
						ctx,
						mm,
						existing.id,
						PermissionForUpdate {
							group_name: Some(reg_perm.group.to_string()),
							display_name: Some(reg_perm.display.to_string()),
							description: if reg_perm.description.is_empty() { None } else { Some(reg_perm.description.to_string()) },
						},
					)
					.await?;
					updated += 1;
					info!("Permission updated: {} (group={}, display={})", reg_perm.key, reg_perm.group, reg_perm.display);
				}
			} else {
				// Create new permission
				Self::create(
					ctx,
					mm,
					PermissionForCreate {
						key: reg_perm.key.to_string(),
						group_name: Some(reg_perm.group.to_string()),
						display_name: Some(reg_perm.display.to_string()),
						description: if reg_perm.description.is_empty() { None } else { Some(reg_perm.description.to_string()) },
					},
				)
				.await?;
				created += 1;
				info!("Permission created: {} (group={}, display={})", reg_perm.key, reg_perm.group, reg_perm.display);
			}
		}

		// --- Delete permissions that exist in database but not in code
		for db_perm in &db_permissions {
			if !registered_keys.contains(db_perm.key.as_str()) {
				// First delete related role_permission records
				let rp_deleted = RolePermissionBmc::delete_by_permission(ctx, mm, db_perm.id).await?;
				if rp_deleted > 0 {
					info!("Deleted {} role_permission records for permission: {}", rp_deleted, db_perm.key);
				}

				// Then delete the permission
				Self::delete(ctx, mm, db_perm.id).await?;
				deleted += 1;
				info!("Permission deleted (not in code): {}", db_perm.key);
			}
		}

		info!(
			"Permission sync complete: {} created, {} updated, {} deleted",
			created, updated, deleted
		);

		Ok((created, updated, deleted))
	}
}

// endregion: --- PermissionBmc

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>;

	use super::*;
	use crate::_dev_utils;
	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_permission_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_key = "test:permission:create";

		// -- Exec
		let permission_id = PermissionBmc::create(
			&ctx,
			&mm,
			PermissionForCreate {
				key: fx_key.to_string(),
				group_name: Some("Test Group".to_string()),
				display_name: Some("Test Permission".to_string()),
				description: None,
			},
		)
		.await?;

		// -- Check
		let permission = PermissionBmc::get(&ctx, &mm, permission_id).await?;
		assert_eq!(permission.key, fx_key);
		assert_eq!(permission.group_name, Some("Test Group".to_string()));

		// -- Clean
		PermissionBmc::delete(&ctx, &mm, permission_id).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_permission_first_by_key_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_key = "test:permission:first_by_key";

		let permission_id = PermissionBmc::create(
			&ctx,
			&mm,
			PermissionForCreate {
				key: fx_key.to_string(),
				group_name: None,
				display_name: None,
				description: None,
			},
		)
		.await?;

		// -- Exec
		let permission = PermissionBmc::first_by_key(&ctx, &mm, fx_key).await?;

		// -- Check
		assert!(permission.is_some());
		assert_eq!(permission.unwrap().key, fx_key);

		// -- Clean
		PermissionBmc::delete(&ctx, &mm, permission_id).await?;

		Ok(())
	}
}

// endregion: --- Tests
