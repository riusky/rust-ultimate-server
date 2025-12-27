//! Unified cache key management for Valkey/Redis.
//!
//! This module provides a centralized way to define and manage cache keys
//! across the application, ensuring consistency and preventing key collisions.

use std::fmt;

/// Cache key prefix for different domains
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CachePrefix {
	/// User-related cache keys
	User,
	/// Session-related cache keys
	Session,
	/// Permission-related cache keys
	Permission,
	/// Role-related cache keys
	Role,
	/// Token-related cache keys
	Token,
	/// Rate limiting cache keys
	RateLimit,
	/// General application cache
	App,
}

impl fmt::Display for CachePrefix {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			CachePrefix::User => write!(f, "user"),
			CachePrefix::Session => write!(f, "session"),
			CachePrefix::Permission => write!(f, "perm"),
			CachePrefix::Role => write!(f, "role"),
			CachePrefix::Token => write!(f, "token"),
			CachePrefix::RateLimit => write!(f, "rate"),
			CachePrefix::App => write!(f, "app"),
		}
	}
}

/// Pre-defined cache keys for common use cases
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CacheKey {
	/// User permissions cache: perm:user:{user_id}
	UserPermissions(i64),
	/// User roles cache: role:user:{user_id}
	UserRoles(i64),
	/// User session cache: session:user:{user_id}
	UserSession(i64),
	/// User profile cache: user:profile:{user_id}
	UserProfile(i64),
	/// Role permissions cache: perm:role:{role_id}
	RolePermissions(i64),
	/// Token blacklist: token:blacklist:{token_hash}
	TokenBlacklist(String),
	/// Rate limit counter: rate:limit:{key}
	RateLimitCounter(String),
	/// Custom key with prefix
	Custom(CachePrefix, String),
}

impl CacheKey {
	/// Get the full cache key string
	pub fn as_str(&self) -> String {
		match self {
			CacheKey::UserPermissions(user_id) => format!("perm:user:{}", user_id),
			CacheKey::UserRoles(user_id) => format!("role:user:{}", user_id),
			CacheKey::UserSession(user_id) => format!("session:user:{}", user_id),
			CacheKey::UserProfile(user_id) => format!("user:profile:{}", user_id),
			CacheKey::RolePermissions(role_id) => format!("perm:role:{}", role_id),
			CacheKey::TokenBlacklist(token_hash) => format!("token:blacklist:{}", token_hash),
			CacheKey::RateLimitCounter(key) => format!("rate:limit:{}", key),
			CacheKey::Custom(prefix, key) => format!("{}:{}", prefix, key),
		}
	}
}

impl fmt::Display for CacheKey {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

impl From<CacheKey> for String {
	fn from(key: CacheKey) -> String {
		key.as_str()
	}
}

impl redis::ToRedisArgs for CacheKey {
	fn write_redis_args<W>(&self, out: &mut W)
	where
		W: ?Sized + redis::RedisWrite,
	{
		self.as_str().write_redis_args(out)
	}
}

/// Default TTL values for different cache types (in seconds)
pub mod ttl {
	/// User permissions TTL: 5 minutes
	pub const USER_PERMISSIONS: u64 = 300;
	/// User roles TTL: 5 minutes
	pub const USER_ROLES: u64 = 300;
	/// User session TTL: 24 hours
	pub const USER_SESSION: u64 = 86400;
	/// User profile TTL: 10 minutes
	pub const USER_PROFILE: u64 = 600;
	/// Role permissions TTL: 10 minutes
	pub const ROLE_PERMISSIONS: u64 = 600;
	/// Token blacklist TTL: 7 days (should match token expiry)
	pub const TOKEN_BLACKLIST: u64 = 604800;
	/// Rate limit window: 1 minute
	pub const RATE_LIMIT_WINDOW: u64 = 60;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_cache_key_format() {
		assert_eq!(
			CacheKey::UserPermissions(123).as_str(),
			"perm:user:123"
		);
		assert_eq!(
			CacheKey::UserRoles(456).as_str(),
			"role:user:456"
		);
		assert_eq!(
			CacheKey::TokenBlacklist("abc123".to_string()).as_str(),
			"token:blacklist:abc123"
		);
		assert_eq!(
			CacheKey::Custom(CachePrefix::App, "config".to_string()).as_str(),
			"app:config"
		);
	}
}
