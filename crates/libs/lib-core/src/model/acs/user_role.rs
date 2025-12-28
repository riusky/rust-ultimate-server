//! User-Role relationship model for ACS (Access Control System)

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

// region:    --- UserRole Types

#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct UserRole {
	pub id: i64,

	// -- Foreign keys
	pub user_id: i64,
	pub role_id: i64,

	// -- Timestamps
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

/// Lightweight UserRole for listing
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct UserRoleLite {
	pub id: i64,
	pub user_id: i64,
	pub role_id: i64,
}

#[derive(Fields, Deserialize)]
pub struct UserRoleForCreate {
	pub user_id: i64,
	pub role_id: i64,
}

#[derive(Clone, FilterNodes, Default, Deserialize)]
pub struct UserRoleFilter {
	pub id: Option<OpValsInt64>,
	pub user_id: Option<OpValsInt64>,
	pub role_id: Option<OpValsInt64>,

	pub cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub ctime: Option<OpValsValue>,
	pub mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub mtime: Option<OpValsValue>,
}

#[derive(Iden)]
enum UserRoleIden {
	UserId,
	RoleId,
}

// endregion: --- UserRole Types

// region:    --- UserRoleBmc

pub struct UserRoleBmc;

impl DbBmc for UserRoleBmc {
	const TABLE: &'static str = "user_role";
}

impl UserRoleBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		user_role_c: UserRoleForCreate,
	) -> Result<i64> {
		base::create::<Self, _>(ctx, mm, user_role_c).await
	}

	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<UserRole> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<UserRoleFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<UserRole>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}

	pub async fn count(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<UserRoleFilter>>,
	) -> Result<i64> {
		base::count::<Self, _>(ctx, mm, filter).await
	}

	// --- Helper methods

	/// Get all role IDs for a user
	pub async fn list_role_ids_for_user(
		_ctx: &Ctx,
		mm: &ModelManager,
		user_id: i64,
	) -> Result<Vec<i64>> {
		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.column(UserRoleIden::RoleId)
			.and_where(Expr::col(UserRoleIden::UserId).eq(user_id));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
		let rows = mm.dbx().fetch_all(sqlx_query).await?;

		Ok(rows.into_iter().map(|(id,)| id).collect())
	}

	/// Get all user IDs that have a specific role
	pub async fn list_user_ids_for_role(
		_ctx: &Ctx,
		mm: &ModelManager,
		role_id: i64,
	) -> Result<Vec<i64>> {
		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.column(UserRoleIden::UserId)
			.and_where(Expr::col(UserRoleIden::RoleId).eq(role_id));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
		let rows = mm.dbx().fetch_all(sqlx_query).await?;

		Ok(rows.into_iter().map(|(id,)| id).collect())
	}

	/// Check if a user has a specific role
	pub async fn has_role(
		_ctx: &Ctx,
		mm: &ModelManager,
		user_id: i64,
		role_id: i64,
	) -> Result<bool> {
		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.column(UserRoleIden::UserId)
			.and_where(Expr::col(UserRoleIden::UserId).eq(user_id))
			.and_where(Expr::col(UserRoleIden::RoleId).eq(role_id))
			.limit(1);

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
		let result = mm.dbx().fetch_optional(sqlx_query).await?;

		Ok(result.is_some())
	}

	/// Delete all roles for a user
	pub async fn delete_by_user(
		_ctx: &Ctx,
		mm: &ModelManager,
		user_id: i64,
	) -> Result<u64> {
		let mut query = Query::delete();
		query
			.from_table(Self::table_ref())
			.and_where(Expr::col(UserRoleIden::UserId).eq(user_id));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_with(&sql, values);
		let count = mm.dbx().execute(sqlx_query).await?;

		Ok(count)
	}

	/// Set roles for a user (replaces all existing roles)
	pub async fn set_roles_for_user(
		ctx: &Ctx,
		mm: &ModelManager,
		user_id: i64,
		role_ids: Vec<i64>,
	) -> Result<()> {
		// Start transaction
		let mm = mm.new_with_txn()?;
		mm.dbx().begin_txn().await?;

		// Delete existing roles
		Self::delete_by_user(ctx, &mm, user_id).await?;

		// Create new roles
		for role_id in role_ids {
			Self::create(ctx, &mm, UserRoleForCreate { user_id, role_id }).await?;
		}

		// Commit transaction
		mm.dbx().commit_txn().await?;

		Ok(())
	}

	/// Get all permission keys for a user (through roles)
	/// This is a complex query that joins user_role -> role_permission -> permission
	pub async fn list_permission_keys_for_user(
		_ctx: &Ctx,
		mm: &ModelManager,
		user_id: i64,
	) -> Result<Vec<String>> {
		// SQL: SELECT DISTINCT p.key FROM user_role ur
		//      JOIN role_permission rp ON ur.role_id = rp.role_id
		//      JOIN permission p ON rp.permission_id = p.id
		//      WHERE ur.user_id = $1
		let sql = r#"
			SELECT DISTINCT p.key 
			FROM user_role ur
			JOIN role_permission rp ON ur.role_id = rp.role_id
			JOIN permission p ON rp.permission_id = p.id
			WHERE ur.user_id = $1
		"#;

		let sqlx_query = sqlx::query_as::<_, (String,)>(sql).bind(user_id);
		let rows = mm.dbx().fetch_all(sqlx_query).await?;

		Ok(rows.into_iter().map(|(key,)| key).collect())
	}

	/// Get all role names for a user
	pub async fn list_role_names_for_user(
		_ctx: &Ctx,
		mm: &ModelManager,
		user_id: i64,
	) -> Result<Vec<String>> {
		let sql = r#"
			SELECT r.name 
			FROM user_role ur
			JOIN role r ON ur.role_id = r.id
			WHERE ur.user_id = $1
		"#;

		let sqlx_query = sqlx::query_as::<_, (String,)>(sql).bind(user_id);
		let rows = mm.dbx().fetch_all(sqlx_query).await?;

		Ok(rows.into_iter().map(|(name,)| name).collect())
	}
}

// endregion: --- UserRoleBmc

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>;

	use super::*;
	use crate::_dev_utils;
	use crate::model::acs::{
		PermissionBmc, PermissionForCreate, RoleBmc, RoleForCreate,
		RolePermissionBmc, RolePermissionForCreate,
	};
	use crate::model::user::{UserBmc, UserForCreate};
	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_user_role_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();

		// Create user and role
		let user_id = UserBmc::create(
			&ctx,
			&mm,
			UserForCreate {
				username: "test_ur_user".to_string(),
				pwd_clear: "test_password".to_string(),
			},
		)
		.await?;

		let role_id = RoleBmc::create(
			&ctx,
			&mm,
			RoleForCreate {
				name: "test_ur_role".to_string(),
				display_name: None,
				description: None,
			},
		)
		.await?;

		// -- Exec
		let ur_id = UserRoleBmc::create(
			&ctx,
			&mm,
			UserRoleForCreate { user_id, role_id },
		)
		.await?;

		// -- Check
		let ur = UserRoleBmc::get(&ctx, &mm, ur_id).await?;
		assert_eq!(ur.user_id, user_id);
		assert_eq!(ur.role_id, role_id);

		// Check has_role
		let has = UserRoleBmc::has_role(&ctx, &mm, user_id, role_id).await?;
		assert!(has);

		// -- Clean
		UserRoleBmc::delete(&ctx, &mm, ur_id).await?;
		RoleBmc::delete(&ctx, &mm, role_id).await?;
		UserBmc::delete(&ctx, &mm, user_id).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_list_permission_keys_for_user_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();

		// Create user
		let user_id = UserBmc::create(
			&ctx,
			&mm,
			UserForCreate {
				username: "test_perm_user".to_string(),
				pwd_clear: "test_password".to_string(),
			},
		)
		.await?;

		// Create role
		let role_id = RoleBmc::create(
			&ctx,
			&mm,
			RoleForCreate {
				name: "test_perm_role".to_string(),
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
				key: "test:user:perm1".to_string(),
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
				key: "test:user:perm2".to_string(),
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

		// Assign role to user
		UserRoleBmc::create(&ctx, &mm, UserRoleForCreate { user_id, role_id })
			.await?;

		// -- Exec
		let perm_keys =
			UserRoleBmc::list_permission_keys_for_user(&ctx, &mm, user_id).await?;

		// -- Check
		assert_eq!(perm_keys.len(), 2);
		assert!(perm_keys.contains(&"test:user:perm1".to_string()));
		assert!(perm_keys.contains(&"test:user:perm2".to_string()));

		// -- Clean
		UserRoleBmc::delete_by_user(&ctx, &mm, user_id).await?;
		RolePermissionBmc::delete_by_role(&ctx, &mm, role_id).await?;
		RoleBmc::delete(&ctx, &mm, role_id).await?;
		PermissionBmc::delete(&ctx, &mm, perm1_id).await?;
		PermissionBmc::delete(&ctx, &mm, perm2_id).await?;
		UserBmc::delete(&ctx, &mm, user_id).await?;

		Ok(())
	}
}

// endregion: --- Tests
