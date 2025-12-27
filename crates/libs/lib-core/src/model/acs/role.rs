//! Role model for ACS (Access Control System)

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

#[cfg(feature = "with-ts")]
use ts_rs::TS;

// region:    --- Role Types

#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
#[cfg_attr(feature = "with-ts", derive(TS))]
#[cfg_attr(feature = "with-ts", ts(export, export_to = "../../../../cmx-vue-ultimate-starter/src/types/generated/acs/"))]
pub struct Role {
	pub id: i64,

	// -- Properties
	pub name: String,
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

/// Lightweight role for listing
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
#[cfg_attr(feature = "with-ts", derive(TS))]
#[cfg_attr(feature = "with-ts", ts(export, export_to = "../../../../cmx-vue-ultimate-starter/src/types/generated/acs/"))]
pub struct RoleLite {
	pub id: i64,
	pub name: String,
	pub display_name: Option<String>,
}

#[derive(Fields, Deserialize)]
pub struct RoleForCreate {
	pub name: String,
	pub display_name: Option<String>,
	pub description: Option<String>,
}

#[derive(Fields, Deserialize)]
pub struct RoleForUpdate {
	pub name: Option<String>,
	pub display_name: Option<String>,
	pub description: Option<String>,
}

#[derive(Clone, FilterNodes, Default, Deserialize)]
pub struct RoleFilter {
	pub id: Option<OpValsInt64>,
	pub name: Option<OpValsString>,
	pub display_name: Option<OpValsString>,

	pub cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub ctime: Option<OpValsValue>,
	pub mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub mtime: Option<OpValsValue>,
}

#[derive(Iden)]
enum RoleIden {
	Name,
}

// endregion: --- Role Types

// region:    --- RoleBmc

pub struct RoleBmc;

impl DbBmc for RoleBmc {
	const TABLE: &'static str = "role";
}

impl RoleBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		role_c: RoleForCreate,
	) -> Result<i64> {
		base::create::<Self, _>(ctx, mm, role_c).await
	}

	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Role> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn first_by_name(
		_ctx: &Ctx,
		mm: &ModelManager,
		name: &str,
	) -> Result<Option<Role>> {
		// -- Build query
		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.columns(Role::sea_column_refs())
			.and_where(Expr::col(RoleIden::Name).eq(name));

		// -- Execute query
		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_as_with::<_, Role, _>(&sql, values);
		let entity = mm.dbx().fetch_optional(sqlx_query).await?;

		Ok(entity)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<RoleFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Role>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}

	pub async fn list_lite(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<RoleFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<RoleLite>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		role_u: RoleForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, role_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}

	pub async fn count(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<RoleFilter>>,
	) -> Result<i64> {
		base::count::<Self, _>(ctx, mm, filter).await
	}
}

// endregion: --- RoleBmc

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
	async fn test_role_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_name = "test_role_create";

		// -- Exec
		let role_id = RoleBmc::create(
			&ctx,
			&mm,
			RoleForCreate {
				name: fx_name.to_string(),
				display_name: Some("Test Role".to_string()),
				description: None,
			},
		)
		.await?;

		// -- Check
		let role = RoleBmc::get(&ctx, &mm, role_id).await?;
		assert_eq!(role.name, fx_name);
		assert_eq!(role.display_name, Some("Test Role".to_string()));

		// -- Clean
		RoleBmc::delete(&ctx, &mm, role_id).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_role_first_by_name_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_name = "test_role_first_by_name";

		let role_id = RoleBmc::create(
			&ctx,
			&mm,
			RoleForCreate {
				name: fx_name.to_string(),
				display_name: None,
				description: None,
			},
		)
		.await?;

		// -- Exec
		let role = RoleBmc::first_by_name(&ctx, &mm, fx_name).await?;

		// -- Check
		assert!(role.is_some());
		assert_eq!(role.unwrap().name, fx_name);

		// -- Clean
		RoleBmc::delete(&ctx, &mm, role_id).await?;

		Ok(())
	}
}

// endregion: --- Tests
