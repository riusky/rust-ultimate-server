use crate::ctx::Ctx;
use crate::generate_common_bmc_fns;
use crate::model::base::{self, DbBmc};
use crate::model::modql_utils::time_to_sea_value;
use crate::model::ModelManager;
use crate::model::Result;
use lib_utils::time::Rfc3339;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue};
use sea_query::Nullable;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use time::Date;

#[cfg(feature = "with-ts")]
use ts_rs::TS;

// region:    --- UserInfo Types

/// User gender
#[derive(Clone, Debug, PartialEq, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[cfg_attr(feature = "with-ts", derive(TS))]
#[cfg_attr(feature = "with-ts", ts(export, export_to = "../../../../cmx-vue-ultimate-starter/src/services/types/user/"))]
#[sqlx(type_name = "user_gender")]
pub enum UserGender {
	Unknown,
	Male,
	Female,
}

impl Default for UserGender {
	fn default() -> Self {
		Self::Unknown
	}
}

impl From<UserGender> for sea_query::Value {
	fn from(val: UserGender) -> Self {
		val.to_string().into()
	}
}

impl Nullable for UserGender {
	fn null() -> sea_query::Value {
		UserGender::Unknown.into()
	}
}

/// User status
#[derive(Clone, Debug, PartialEq, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[cfg_attr(feature = "with-ts", derive(TS))]
#[cfg_attr(feature = "with-ts", ts(export, export_to = "../../../../cmx-vue-ultimate-starter/src/services/types/user/"))]
#[sqlx(type_name = "user_status")]
pub enum UserStatus {
	Active,
	Inactive,
	Suspended,
	Deleted,
}

impl Default for UserStatus {
	fn default() -> Self {
		Self::Active
	}
}

impl From<UserStatus> for sea_query::Value {
	fn from(val: UserStatus) -> Self {
		val.to_string().into()
	}
}

impl Nullable for UserStatus {
	fn null() -> sea_query::Value {
		UserStatus::Active.into()
	}
}

/// User info entity
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
#[cfg_attr(feature = "with-ts", derive(TS))]
#[cfg_attr(feature = "with-ts", ts(export, export_to = "../../../../cmx-vue-ultimate-starter/src/services/types/user/"))]
pub struct UserInfo {
	pub id: i64,
	pub user_id: i64,

	// -- Basic Info
	pub nickname: Option<String>,
	pub avatar: Option<String>,
	pub bio: Option<String>,

	// -- Contact
	pub email: Option<String>,
	pub email_verified: bool,
	pub phone: Option<String>,
	pub phone_verified: bool,

	// -- Personal Attributes
	pub gender: UserGender,
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

	// -- Account Status
	pub status: UserStatus,
	#[serde_as(as = "Option<Rfc3339>")]
	#[cfg_attr(feature = "with-ts", ts(type = "string | null"))]
	pub last_login_at: Option<OffsetDateTime>,
	pub last_login_ip: Option<String>,
	pub login_count: i32,

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

/// For creating a new user info record
#[derive(Fields, Deserialize)]
pub struct UserInfoForCreate {
	pub user_id: i64,
	pub nickname: Option<String>,
	pub avatar: Option<String>,
	pub bio: Option<String>,
	pub email: Option<String>,
	pub phone: Option<String>,
	pub gender: Option<UserGender>,
	pub birthday: Option<Date>,
	pub country: Option<String>,
	pub province: Option<String>,
	pub city: Option<String>,
	pub address: Option<String>,
	pub postal_code: Option<String>,
	pub timezone: Option<String>,
	pub locale: Option<String>,
	pub theme: Option<String>,
}

/// For updating user info
#[derive(Fields, Deserialize, Default)]
pub struct UserInfoForUpdate {
	pub nickname: Option<String>,
	pub avatar: Option<String>,
	pub bio: Option<String>,
	pub email: Option<String>,
	pub email_verified: Option<bool>,
	pub phone: Option<String>,
	pub phone_verified: Option<bool>,
	pub gender: Option<UserGender>,
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
	pub last_login_at: Option<OffsetDateTime>,
	pub last_login_ip: Option<String>,
	pub login_count: Option<i32>,
}

/// Filter for querying user info
#[derive(Clone, Debug, FilterNodes, Default, Deserialize)]
#[cfg_attr(feature = "with-ts", derive(TS))]
#[cfg_attr(feature = "with-ts", ts(export, export_to = "../../../../cmx-vue-ultimate-starter/src/services/types/user/"))]
pub struct UserInfoFilter {
	pub id: Option<OpValsInt64>,
	pub user_id: Option<OpValsInt64>,
	pub nickname: Option<OpValsString>,
	pub email: Option<OpValsString>,
	pub phone: Option<OpValsString>,
	pub gender: Option<OpValsString>,
	pub country: Option<OpValsString>,
	pub province: Option<OpValsString>,
	pub city: Option<OpValsString>,
	pub status: Option<OpValsString>,

	pub cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub ctime: Option<OpValsValue>,
	pub mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub mtime: Option<OpValsValue>,
}

// endregion: --- UserInfo Types

// region:    --- UserInfoBmc

pub struct UserInfoBmc;

impl DbBmc for UserInfoBmc {
	const TABLE: &'static str = "user_info";
}

generate_common_bmc_fns!(
	Bmc: UserInfoBmc,
	Entity: UserInfo,
	ForCreate: UserInfoForCreate,
	ForUpdate: UserInfoForUpdate,
	Filter: UserInfoFilter,
);

// endregion: --- UserInfoBmc

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>;

	use super::*;
	use crate::_dev_utils;
	use crate::model::user::{UserBmc, UserForCreate};
	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_create_user_info_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_username = "test_user_info_01";

		// Create a user first
		let user_id = UserBmc::create(
			&ctx,
			&mm,
			UserForCreate {
				username: fx_username.to_string(),
				pwd_clear: "test_pwd".to_string(),
			},
		)
		.await?;

		// -- Exec
		let user_info_c = UserInfoForCreate {
			user_id,
			nickname: Some("Test User".to_string()),
			avatar: None,
			bio: Some("Test bio".to_string()),
			email: Some("test@example.com".to_string()),
			phone: None,
			gender: Some(UserGender::Male),
			birthday: None,
			country: Some("China".to_string()),
			province: None,
			city: None,
			address: None,
			postal_code: None,
			timezone: Some("Asia/Shanghai".to_string()),
			locale: Some("zh-CN".to_string()),
			theme: Some("dark".to_string()),
		};
		let user_info_id = UserInfoBmc::create(&ctx, &mm, user_info_c).await?;

		// -- Check
		let user_info = UserInfoBmc::get(&ctx, &mm, user_info_id).await?;
		assert_eq!(user_info.user_id, user_id);
		assert_eq!(user_info.nickname, Some("Test User".to_string()));
		assert_eq!(user_info.gender, UserGender::Male);
		assert_eq!(user_info.status, UserStatus::Active);

		// -- Clean
		UserBmc::delete(&ctx, &mm, user_id).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_update_user_info_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_username = "test_user_info_update_01";

		// Create a user first
		let user_id = UserBmc::create(
			&ctx,
			&mm,
			UserForCreate {
				username: fx_username.to_string(),
				pwd_clear: "test_pwd".to_string(),
			},
		)
		.await?;

		// Create user info
		let user_info_c = UserInfoForCreate {
			user_id,
			nickname: Some("Original Name".to_string()),
			avatar: None,
			bio: None,
			email: None,
			phone: None,
			gender: None,
			birthday: None,
			country: None,
			province: None,
			city: None,
			address: None,
			postal_code: None,
			timezone: None,
			locale: None,
			theme: None,
		};
		let user_info_id = UserInfoBmc::create(&ctx, &mm, user_info_c).await?;

		// -- Exec
		let user_info_u = UserInfoForUpdate {
			nickname: Some("Updated Name".to_string()),
			theme: Some("light".to_string()),
			..Default::default()
		};
		UserInfoBmc::update(&ctx, &mm, user_info_id, user_info_u).await?;

		// -- Check
		let user_info = UserInfoBmc::get(&ctx, &mm, user_info_id).await?;
		assert_eq!(user_info.nickname, Some("Updated Name".to_string()));
		assert_eq!(user_info.theme, Some("light".to_string()));

		// -- Clean
		UserBmc::delete(&ctx, &mm, user_id).await?;

		Ok(())
	}
}

// endregion: --- Tests
