//! Role-Permission relationship model for ACS (Access Control System)

use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::modql_utils::time_to_sea_value;
use crate::model::ModelManager;
use crate::model::Result;
use lib_utils::time::Rfc3339;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsValue};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

// region:    --- RolePermission Types

#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct RolePermission {
	pub id: i64,

	// -- Foreign keys
	pub role_id: i64,
	pub permission_id: i64,

	// -- Timestamps
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

/// Lightweight RolePermission for listing
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct RolePermissionLite {
	pub id: i64,
	pub role_id: i64,
	pub permission_id: i64,
}

#[derive(Fields, Deserialize)]
pub struct RolePermissionForCreate {
	pub role_id: i64,
	pub permission_id: i64,
}

#[derive(Clone, FilterNodes, Default, Deserialize)]
pub struct RolePermissionFilter {
	pub id: Option<OpValsInt64>,
	pub role_id: Option<OpValsInt64>,
	pub permission_id: Option<OpValsInt64>,

	pub cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub ctime: Option<OpValsValue>,
	pub mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub mtime: Option<OpValsValue>,
}

#[derive(Iden)]
enum RolePermissionIden {
	RoleId,
	PermissionId,
}

// endregion: --- RolePermission Types

// region:    --- RolePermissionBmc

pub struct RolePermissionBmc;

impl DbBmc for RolePermissionBmc {
	const TABLE: &'static str = "role_permission";
}

impl RolePermissionBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		role_permission_c: RolePermissionForCreate,
	) -> Result<i64> {
		let role_id = role_permission_c.role_id;
		let id = base::create::<Self, _>(ctx, mm, role_permission_c).await?;
		Self::invalidate_role_users_permissions_best_effort(ctx, mm, role_id).await;
		Ok(id)
	}

	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<RolePermission> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<RolePermissionFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<RolePermission>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		let role_permission = Self::get(ctx, mm, id).await?;
		base::delete::<Self>(ctx, mm, id).await?;
		Self::invalidate_role_users_permissions_best_effort(ctx, mm, role_permission.role_id).await;
		Ok(())
	}

	pub async fn count(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<RolePermissionFilter>>,
	) -> Result<i64> {
		base::count::<Self, _>(ctx, mm, filter).await
	}

	// --- Helper methods

	/// Get all permission IDs for a role
	pub async fn list_permission_ids_for_role(
		_ctx: &Ctx,
		mm: &ModelManager,
		role_id: i64,
	) -> Result<Vec<i64>> {
		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.column(RolePermissionIden::PermissionId)
			.and_where(Expr::col(RolePermissionIden::RoleId).eq(role_id));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
		let rows = mm.dbx().fetch_all(sqlx_query).await?;

		Ok(rows.into_iter().map(|(id,)| id).collect())
	}

	/// Check if a role has a specific permission
	pub async fn has_permission(
		_ctx: &Ctx,
		mm: &ModelManager,
		role_id: i64,
		permission_id: i64,
	) -> Result<bool> {
		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.column(RolePermissionIden::RoleId)
			.and_where(Expr::col(RolePermissionIden::RoleId).eq(role_id))
			.and_where(Expr::col(RolePermissionIden::PermissionId).eq(permission_id))
			.limit(1);

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
		let result = mm.dbx().fetch_optional(sqlx_query).await?;

		Ok(result.is_some())
	}

	/// Delete all permissions for a role
	pub async fn delete_by_role(ctx: &Ctx, mm: &ModelManager, role_id: i64) -> Result<u64> {
		let mut query = Query::delete();
		query
			.from_table(Self::table_ref())
			.and_where(Expr::col(RolePermissionIden::RoleId).eq(role_id));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_with(&sql, values);
		let count = mm.dbx().execute(sqlx_query).await?;
		Self::invalidate_role_users_permissions_best_effort(ctx, mm, role_id).await;

		Ok(count)
	}

	/// Delete all role associations for a permission
	/// Used when syncing permissions (removing stale permissions from database)
	pub async fn delete_by_permission(
		ctx: &Ctx,
		mm: &ModelManager,
		permission_id: i64,
	) -> Result<u64> {
		let role_ids = Self::list_role_ids_for_permission(ctx, mm, permission_id).await?;
		let mut query = Query::delete();
		query
			.from_table(Self::table_ref())
			.and_where(Expr::col(RolePermissionIden::PermissionId).eq(permission_id));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_with(&sql, values);
		let count = mm.dbx().execute(sqlx_query).await?;
		for role_id in role_ids {
			Self::invalidate_role_users_permissions_best_effort(ctx, mm, role_id).await;
		}

		Ok(count)
	}

	/// Set permissions for a role (replaces all existing permissions)
	pub async fn set_permissions_for_role(
		ctx: &Ctx,
		mm: &ModelManager,
		role_id: i64,
		permission_ids: Vec<i64>,
	) -> Result<()> {
		// Start transaction
		let mm = mm.new_with_txn()?;
		mm.dbx().begin_txn().await?;

		// Delete existing permissions
		Self::delete_by_role(ctx, &mm, role_id).await?;

		// Create new permissions
		for permission_id in permission_ids {
			Self::create(
				ctx,
				&mm,
				RolePermissionForCreate {
					role_id,
					permission_id,
				},
			)
			.await?;
		}

		// Commit transaction
		mm.dbx().commit_txn().await?;
		Self::invalidate_role_users_permissions_best_effort(ctx, &mm, role_id).await;

		Ok(())
	}

	async fn list_role_ids_for_permission(
		_ctx: &Ctx,
		mm: &ModelManager,
		permission_id: i64,
	) -> Result<Vec<i64>> {
		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.column(RolePermissionIden::RoleId)
			.and_where(Expr::col(RolePermissionIden::PermissionId).eq(permission_id));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
		let rows = mm.dbx().fetch_all(sqlx_query).await?;

		Ok(rows.into_iter().map(|(id,)| id).collect())
	}

	async fn invalidate_role_users_permissions_best_effort(
		ctx: &Ctx,
		mm: &ModelManager,
		role_id: i64,
	) {
		match super::UserRoleBmc::list_user_ids_for_role(ctx, mm, role_id).await {
			Ok(user_ids) => {
				crate::model::cache::invalidate_users_permissions_cache_best_effort(
					mm.valkey_pool(),
					&user_ids,
				)
				.await;
			}
			Err(err) => {
				tracing::warn!(
					role_id,
					error = ?err,
					"permission cache invalidation skipped because role users could not be listed"
				);
			}
		}
	}
}

// endregion: --- RolePermissionBmc

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>;

	use super::*;
	use crate::_dev_utils;
	use crate::model::acs::{PermissionBmc, PermissionForCreate, RoleBmc, RoleForCreate};
	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_role_permission_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();

		// Create role and permission
		let role_id = RoleBmc::create(
			&ctx,
			&mm,
			RoleForCreate {
				name: "test_rp_role".to_string(),
				display_name: None,
				description: None,
			},
		)
		.await?;

		let permission_id = PermissionBmc::create(
			&ctx,
			&mm,
			PermissionForCreate {
				key: "test:rp:perm".to_string(),
				group_name: None,
				display_name: None,
				description: None,
			},
		)
		.await?;

		// -- Exec
		let rp_id = RolePermissionBmc::create(
			&ctx,
			&mm,
			RolePermissionForCreate {
				role_id,
				permission_id,
			},
		)
		.await?;

		// -- Check
		let rp = RolePermissionBmc::get(&ctx, &mm, rp_id).await?;
		assert_eq!(rp.role_id, role_id);
		assert_eq!(rp.permission_id, permission_id);

		// Check has_permission
		let has = RolePermissionBmc::has_permission(&ctx, &mm, role_id, permission_id).await?;
		assert!(has);

		// -- Clean
		RolePermissionBmc::delete(&ctx, &mm, rp_id).await?;
		RoleBmc::delete(&ctx, &mm, role_id).await?;
		PermissionBmc::delete(&ctx, &mm, permission_id).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_list_permission_ids_for_role_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();

		// Create role
		let role_id = RoleBmc::create(
			&ctx,
			&mm,
			RoleForCreate {
				name: "test_list_perm_role".to_string(),
				display_name: None,
				description: None,
			},
		)
		.await?;

		// Create permissions
		let perm1_id = PermissionBmc::create(
			&ctx,
			&mm,
			PermissionForCreate {
				key: "test:list:perm1".to_string(),
				group_name: None,
				display_name: None,
				description: None,
			},
		)
		.await?;

		let perm2_id = PermissionBmc::create(
			&ctx,
			&mm,
			PermissionForCreate {
				key: "test:list:perm2".to_string(),
				group_name: None,
				display_name: None,
				description: None,
			},
		)
		.await?;

		// Assign permissions to role
		RolePermissionBmc::create(
			&ctx,
			&mm,
			RolePermissionForCreate {
				role_id,
				permission_id: perm1_id,
			},
		)
		.await?;

		RolePermissionBmc::create(
			&ctx,
			&mm,
			RolePermissionForCreate {
				role_id,
				permission_id: perm2_id,
			},
		)
		.await?;

		// -- Exec
		let perm_ids = RolePermissionBmc::list_permission_ids_for_role(&ctx, &mm, role_id).await?;

		// -- Check
		assert_eq!(perm_ids.len(), 2);
		assert!(perm_ids.contains(&perm1_id));
		assert!(perm_ids.contains(&perm2_id));

		// -- Clean
		RolePermissionBmc::delete_by_role(&ctx, &mm, role_id).await?;
		RoleBmc::delete(&ctx, &mm, role_id).await?;
		PermissionBmc::delete(&ctx, &mm, perm1_id).await?;
		PermissionBmc::delete(&ctx, &mm, perm2_id).await?;

		Ok(())
	}
}

// endregion: --- Tests
