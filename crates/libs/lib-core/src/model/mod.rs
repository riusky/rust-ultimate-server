//! Model Layer
//!
//! Design:
//!
//! - The Model layer normalizes the application's data type
//!   structures and access.
//! - All application code data access must go through the Model layer.
//! - The `ModelManager` holds the internal states/resources
//!   needed by ModelControllers to access data.
//!   (e.g., db_pool, S3 client, redis client).
//! - Model Controllers (e.g., `ConvBmc`, `AgentBmc`) implement
//!   CRUD and other data access methods on a given "entity"
//!   (e.g., `Conv`, `Agent`).
//!   (`Bmc` is short for Backend Model Controller).
//! - In frameworks like Axum, Tauri, `ModelManager` are typically used as App State.
//! - ModelManager are designed to be passed as an argument
//!   to all Model Controllers functions.
//!

// region:    --- Modules

mod base;
pub mod cache;
mod error;
mod store;

pub mod acs;
pub mod agent;
pub mod conv;
pub mod conv_msg;
pub mod conv_user;
pub mod modql_utils;
#[cfg(feature = "with-ts")]
pub mod ts_export;
pub mod user;
pub mod user_info;

pub use self::base::DbBmc;
pub use self::error::{Error, Result};
pub use self::user::{User, UserTyp};
pub use self::user_info::{UserGender, UserInfo, UserStatus};

use crate::model::store::dbx::Dbx;
use crate::model::store::{new_db_pool, run_db_migrations};
use lib_valkey_core::ValkeyPool;

// endregion: --- Modules

// region:    --- ModelManager

#[cfg_attr(feature = "with-rpc", derive(rpc_router::RpcResource))]
#[derive(Clone)]
pub struct ModelManager {
	dbx: Dbx,
	valkey_pool: Option<ValkeyPool>,
}

impl ModelManager {
	/// Constructor
	pub async fn new() -> Result<Self> {
		Self::new_with_valkey_pool(None).await
	}

	pub async fn new_with_valkey_pool(valkey_pool: Option<ValkeyPool>) -> Result<Self> {
		// 1. Run database migrations first
		run_db_migrations()
			.await
			.map_err(|ex| Error::CantCreateModelManagerProvider(ex.to_string()))?;

		// 2. Create sqlx connection pool
		let db_pool = new_db_pool()
			.await
			.map_err(|ex| Error::CantCreateModelManagerProvider(ex.to_string()))?;
		let dbx = Dbx::new(db_pool, false)?;
		Ok(ModelManager { dbx, valkey_pool })
	}

	pub fn new_with_txn(&self) -> Result<ModelManager> {
		let dbx = Dbx::new(self.dbx.db().clone(), true)?;
		Ok(ModelManager {
			dbx,
			valkey_pool: self.valkey_pool.clone(),
		})
	}

	pub fn dbx(&self) -> &Dbx {
		&self.dbx
	}

	pub fn valkey_pool(&self) -> Option<&ValkeyPool> {
		self.valkey_pool.as_ref()
	}
}

// endregion: --- ModelManager
