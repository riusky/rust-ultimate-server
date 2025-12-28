//! Permission cache utilities
//!
//! This module provides functions for managing user permissions cache.
//! Use these functions when user roles or role permissions change.

use lib_valkey_core::cache_keys::CacheKey;
use lib_valkey_core::{commands, ValkeyPool};
use tracing::debug;

/// Invalidate permissions cache for a single user.
///
/// Call this when:
/// - User's roles are changed via `set_roles_for_user`
pub async fn invalidate_user_permissions_cache(
	valkey_pool: &ValkeyPool,
	user_id: i64,
) -> lib_valkey_core::Result<()> {
	let cache_key = CacheKey::UserPermissions(user_id);
	let mut conn = valkey_pool.get().await?;
	commands::del_one(&mut *conn, cache_key.as_str()).await?;
	debug!(
		"{:<12} - Invalidated permissions cache for user {}",
		"CACHE", user_id
	);
	Ok(())
}

/// Invalidate permissions cache for multiple users.
///
/// Call this when:
/// - A role's permissions are changed via `set_permissions_for_role`
///   (affects all users who have that role)
pub async fn invalidate_users_permissions_cache(
	valkey_pool: &ValkeyPool,
	user_ids: &[i64],
) -> lib_valkey_core::Result<()> {
	if user_ids.is_empty() {
		return Ok(());
	}

	let mut conn = valkey_pool.get().await?;

	for user_id in user_ids {
		let cache_key = CacheKey::UserPermissions(*user_id);
		commands::del_one(&mut *conn, cache_key.as_str()).await?;
	}

	debug!(
		"{:<12} - Invalidated permissions cache for {} users",
		"CACHE",
		user_ids.len()
	);
	Ok(())
}
