use crate::ctx::Ctx;
use crate::model::base::{self, prep_fields_for_update, DbBmc};
use crate::model::modql_utils::time_to_sea_value;
use crate::model::user_info::{UserGender, UserStatus};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use lib_auth::pwd::{self, ContentToHash};
use lib_utils::time::Rfc3339;
use modql::field::{Fields, HasSeaFields, SeaField, SeaFields};
use modql::filter::{
	FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue,
};
use sea_query::{Alias, Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::postgres::PgRow;
use sqlx::types::time::OffsetDateTime;
use sqlx::{FromRow, Row};
use time::Date;
use uuid::Uuid;

#[cfg(feature = "with-ts")]
use ts_rs::TS;

// region:    --- User Types
#[derive(Clone, Debug, PartialEq, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[cfg_attr(feature = "with-ts", derive(TS))]
#[cfg_attr(feature = "with-ts", ts(export, export_to = "../../../../cmx-vue-ultimate-starter/src/services/types/user/"))]
#[sqlx(type_name = "user_typ")]
pub enum UserTyp {
	Sys,
	User,
}
impl From<UserTyp> for sea_query::Value {
	fn from(val: UserTyp) -> Self {
		val.to_string().into()
	}
}

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
#[cfg_attr(feature = "with-ts", derive(TS))]
#[cfg_attr(feature = "with-ts", ts(export, export_to = "../../../../cmx-vue-ultimate-starter/src/services/types/user/"))]
pub struct User {
	pub id: i64,
	pub username: String,
	pub typ: UserTyp,
}

#[derive(Deserialize)]
pub struct UserForCreate {
	pub username: String,
	pub pwd_clear: String,
}

#[derive(Fields)]
pub struct UserForInsert {
	pub username: String,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForLogin {
	pub id: i64,
	pub username: String,

	// -- pwd and token info
	pub pwd: Option<String>, // encrypted, #_scheme_id_#....
	pub pwd_salt: Uuid,
	pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForAuth {
	pub id: i64,
	pub username: String,

	// -- token info
	pub token_salt: Uuid,
}

/// Marker trait
pub trait UserBy: HasSeaFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserForLogin {}
impl UserBy for UserForAuth {}

// Note: Since the entity properties Iden will be given by modql
//       UserIden does not have to be exhaustive, but just have the columns
//       we use in our specific code.
#[derive(Iden)]
enum UserIden {
	Id,
	Username,
	Pwd,
}

#[derive(FilterNodes, Deserialize, Default, Debug, Clone)]
pub struct UserFilter {
	pub id: Option<OpValsInt64>,

	pub username: Option<OpValsString>,

	pub cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub ctime: Option<OpValsValue>,
	pub mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub mtime: Option<OpValsValue>,
}

/// User with info - combined view of user and user_info
#[serde_as]
#[derive(Debug, Clone, FromRow, Serialize)]
#[cfg_attr(feature = "with-ts", derive(TS))]
#[cfg_attr(feature = "with-ts", ts(export, export_to = "../../../../cmx-vue-ultimate-starter/src/services/types/user/"))]
pub struct UserWithInfo {
	// -- User fields
	pub id: i64,
	pub username: String,
	pub typ: UserTyp,
	// -- User timestamps
	#[serde_as(as = "Rfc3339")]
	#[cfg_attr(feature = "with-ts", ts(type = "string"))]
	pub ctime: OffsetDateTime,
	#[serde_as(as = "Rfc3339")]
	#[cfg_attr(feature = "with-ts", ts(type = "string"))]
	pub mtime: OffsetDateTime,

	// -- UserInfo fields (optional, may be null if user_info doesn't exist)
	pub user_info_id: Option<i64>,
	pub nickname: Option<String>,
	pub avatar: Option<String>,
	pub bio: Option<String>,
	pub email: Option<String>,
	pub email_verified: Option<bool>,
	pub phone: Option<String>,
	pub phone_verified: Option<bool>,
	pub gender: Option<UserGender>,
	#[cfg_attr(feature = "with-ts", ts(type = "string | null"))]
	pub birthday: Option<Date>,
	pub country: Option<String>,
	pub province: Option<String>,
	pub city: Option<String>,
	pub address: Option<String>,
	pub postal_code: Option<String>,
	pub timezone: Option<String>,
	pub locale: Option<String>,
	pub theme: Option<String>,
	pub status: Option<UserStatus>,
	#[serde_as(as = "Option<Rfc3339>")]
	#[cfg_attr(feature = "with-ts", ts(type = "string | null"))]
	pub last_login_at: Option<OffsetDateTime>,
	pub last_login_ip: Option<String>,
	pub login_count: Option<i32>,
}

// endregion: --- User Types

// region:    --- UserBmc

pub struct UserBmc;

impl DbBmc for UserBmc {
	const TABLE: &'static str = "user";
}

impl UserBmc {
	/// Count users excluding system users
	pub async fn count(
		_ctx: &Ctx,
		mm: &ModelManager,
		_filter: Option<Vec<UserFilter>>,
	) -> Result<i64> {
		let db = mm.dbx().db();

		// Build count query excluding system users (cast to user_typ enum)
		let query = Query::select()
			.from(Self::table_ref())
			.expr(sea_query::Expr::col(sea_query::Asterisk).count())
			.and_where(Expr::cust("typ != 'Sys'::user_typ"))
			.to_owned();

		let query_str = query.to_string(PostgresQueryBuilder);

		let result = sqlx::query(&query_str)
			.fetch_one(db)
			.await
			.map_err(|_| Error::CountFail)?;

		let count: i64 = result.try_get("count").map_err(|_| Error::CountFail)?;

		Ok(count)
	}

	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		user_c: UserForCreate,
	) -> Result<i64> {
		let UserForCreate {
			username,
			pwd_clear,
		} = user_c;

		// -- Create the user row
		let user_fi = UserForInsert {
			username: username.to_string(),
		};

		// Start the transaction
		let mm = mm.new_with_txn()?;

		mm.dbx().begin_txn().await?;

		let user_id = base::create::<Self, _>(ctx, &mm, user_fi).await.map_err(
			|model_error| {
				Error::resolve_unique_violation(
					model_error,
					Some(|table: &str, constraint: &str| {
						if table == "user" && constraint.contains("username") {
							Some(Error::UserAlreadyExists { username })
						} else {
							None // Error::UniqueViolation will be created by resolve_unique_violation
						}
					}),
				)
			},
		)?;

		// -- Update the database
		Self::update_pwd(ctx, &mm, user_id, &pwd_clear).await?;

		// Commit the transaction
		mm.dbx().commit_txn().await?;

		Ok(user_id)
	}

	pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
	where
		E: UserBy,
	{
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn first_by_username<E>(
		_ctx: &Ctx,
		mm: &ModelManager,
		username: &str,
	) -> Result<Option<E>>
	where
		E: UserBy,
	{
		// -- Build query
		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.columns(E::sea_idens())
			.and_where(Expr::col(UserIden::Username).eq(username));

		// -- Execute query
		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

		let sqlx_query = sqlx::query_as_with::<_, E, _>(&sql, values);
		let entity = mm.dbx().fetch_optional(sqlx_query).await?;

		Ok(entity)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<UserFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<User>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}

	pub async fn update_pwd(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		pwd_clear: &str,
	) -> Result<()> {
		// -- Prep password
		let user: UserForLogin = Self::get(ctx, mm, id).await?;
		let pwd = pwd::hash_pwd(ContentToHash {
			content: pwd_clear.to_string(),
			salt: user.pwd_salt,
		})
		.await?;

		// -- Prep the data
		let mut fields = SeaFields::new(vec![SeaField::new(UserIden::Pwd, pwd)]);
		prep_fields_for_update::<Self>(&mut fields, ctx.user_id());

		// -- Build query
		let fields = fields.for_sea_update();
		let mut query = Query::update();
		query
			.table(Self::table_ref())
			.values(fields)
			.and_where(Expr::col(UserIden::Id).eq(id));

		// -- Exec query
		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_with(&sql, values);
		let _count = mm.dbx().execute(sqlx_query).await?;

		Ok(())
	}

	/// TODO: For User, deletion will require a soft-delete approach:
	///       - Set `deleted: true`.
	///       - Change `username` to "DELETED-_user_id_".
	///       - Clear any other UUIDs or PII (Personally Identifiable Information).
	///       - The automatically set `mid`/`mtime` will record who performed the deletion.
	///       - It's likely necessary to record this action in a `um_change_log` (a user management change audit table).
	///       - Remove or clean up any user-specific assets (messages, etc.).
	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}

	/// List users with their user_info (LEFT JOIN)
	pub async fn list_with_info(
		_ctx: &Ctx,
		mm: &ModelManager,
		_filter: Option<Vec<UserFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<UserWithInfo>> {
		// Helper to add multiple columns from a table
		fn add_columns(
			query: &mut sea_query::SelectStatement,
			table: &Alias,
			fields: &[(&str, &str)], // (column_name, alias)
		) {
			for (col, alias) in fields {
				query.expr_as(
					Expr::col((table.clone(), Alias::new(*col))),
					Alias::new(*alias),
				);
			}
		}

		let user_table = Alias::new("user");
		let user_info_table = Alias::new("user_info");

		let mut query = Query::select();
		query.from(user_table.clone());

		// User fields
		add_columns(&mut query, &user_table, &[
			("id", "id"),
			("username", "username"),
			("typ", "typ"),
			("ctime", "ctime"),
			("mtime", "mtime"),
		]);

		// UserInfo fields (with alias for id -> user_info_id)
		add_columns(&mut query, &user_info_table, &[
			("id", "user_info_id"),
			("nickname", "nickname"),
			("avatar", "avatar"),
			("bio", "bio"),
			("email", "email"),
			("email_verified", "email_verified"),
			("phone", "phone"),
			("phone_verified", "phone_verified"),
			("gender", "gender"),
			("birthday", "birthday"),
			("country", "country"),
			("province", "province"),
			("city", "city"),
			("address", "address"),
			("postal_code", "postal_code"),
			("timezone", "timezone"),
			("locale", "locale"),
			("theme", "theme"),
			("status", "status"),
			("last_login_at", "last_login_at"),
			("last_login_ip", "last_login_ip"),
			("login_count", "login_count"),
		]);

		// LEFT JOIN user_info ON user.id = user_info.user_id
		query.left_join(
			user_info_table.clone(),
			Expr::col((user_table.clone(), Alias::new("id")))
				.equals((user_info_table.clone(), Alias::new("user_id"))),
		);

		// Exclude system users (typ = 'Sys')
		query.and_where(Expr::cust("\"user\".typ != 'Sys'::user_typ"));

		// Apply list options (limit, offset, order_by)
		if let Some(opts) = list_options {
			if let Some(limit) = opts.limit {
				query.limit(limit as u64);
			}
			if let Some(offset) = opts.offset {
				query.offset(offset as u64);
			}
			// Default order by user.id desc
			if opts.order_bys.is_none() {
				query.order_by(
					(user_table.clone(), Alias::new("id")),
					sea_query::Order::Desc,
				);
			}
		} else {
			// Default order by user.id desc
			query.order_by(
				(user_table.clone(), Alias::new("id")),
				sea_query::Order::Desc,
			);
		}

		// Execute query
		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_as_with::<_, UserWithInfo, _>(&sql, values);
		let entities = mm.dbx().fetch_all(sqlx_query).await?;

		Ok(entities)
	}
}

// endregion: --- UserBmc

// region:    --- Tests

#[cfg(test)]
mod tests {
	pub type Result<T> = core::result::Result<T, Error>;
	pub type Error = Box<dyn std::error::Error>; // For tests.

	use super::*;
	use crate::_dev_utils;
	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_username = "test_create_ok-user-01";
		let fx_pwd_clear = "test_create_ok pwd 01";

		// -- Exec
		let user_id = UserBmc::create(
			&ctx,
			&mm,
			UserForCreate {
				username: fx_username.to_string(),
				pwd_clear: fx_pwd_clear.to_string(),
			},
		)
		.await?;

		// -- Check
		let user: UserForLogin = UserBmc::get(&ctx, &mm, user_id).await?;
		assert_eq!(user.username, fx_username);

		// -- Clean
		UserBmc::delete(&ctx, &mm, user_id).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_first_ok_demo1() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_username = "demo1";

		// -- Exec
		let user: User = UserBmc::first_by_username(&ctx, &mm, fx_username)
			.await?
			.ok_or("Should have user 'demo1'")?;

		// -- Check
		assert_eq!(user.username, fx_username);

		Ok(())
	}
}

// endregion: --- Tests
