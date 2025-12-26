//! Axum extractor for Valkey connections

use crate::{Error, ValkeyPool};
use axum::{
	extract::{FromRef, FromRequestParts},
	http::{request::Parts, StatusCode},
};
use bb8_redis::RedisConnectionManager;

/// Axum extractor that provides a Valkey connection from the pool
pub struct ValkeyConn(pub bb8::PooledConnection<'static, RedisConnectionManager>);

impl<S> FromRequestParts<S> for ValkeyConn
where
	ValkeyPool: axum::extract::FromRef<S>,
	S: Send + Sync,
{
	type Rejection = (StatusCode, String);

	async fn from_request_parts(
		_parts: &mut Parts,
		state: &S,
	) -> Result<Self, Self::Rejection> {
		let pool = ValkeyPool::from_ref(state);

		let conn = pool.get_owned().await.map_err(|e| {
			let err = Error::from(e);
			(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
		})?;

		Ok(Self(conn))
	}
}

impl std::ops::Deref for ValkeyConn {
	type Target = redis::aio::MultiplexedConnection;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl std::ops::DerefMut for ValkeyConn {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
