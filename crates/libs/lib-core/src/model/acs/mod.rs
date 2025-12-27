//! ACS - Access Control System
//!
//! A permission-based access control system that provides:
//! - Permission management (individual permissions)
//! - Role management (groups of permissions)
//! - Role-Permission relationships
//! - User-Role relationships
//!
//! ## Architecture
//!
//! ```text
//! User -> UserRole -> Role -> RolePermission -> Permission
//! ```
//!
//! ## Usage
//!
//! ```rust,ignore
//! use lib_core::model::acs::{UserRoleBmc, PermissionBmc};
//!
//! // Get all permission keys for a user
//! let permissions = UserRoleBmc::list_permission_keys_for_user(&ctx, &mm, user_id).await?;
//!
//! // Check if user has specific permission
//! if permissions.contains(&"agent:create".to_string()) {
//!     // User can create agents
//! }
//! ```

mod permission;
mod role;
mod role_permission;
mod sync;
mod user_role;

// -- Re-exports
pub use permission::*;
pub use role::*;
pub use role_permission::*;
pub use sync::*;
pub use user_role::*;

use std::collections::HashSet;

// region:    --- Permission Registry

/// Registered permission definition for compile-time collection
/// Used with inventory crate for automatic permission discovery
#[derive(Debug, Clone)]
pub struct RegisteredPermission {
	pub key: &'static str,
	pub group: &'static str,
	pub display: &'static str,
	pub description: &'static str,
	pub source: &'static str,
}

// Note: The actual inventory::collect! for compile-time permission registration
inventory::collect!(RegisteredPermission);

// endregion: --- Permission Registry

// region:    --- User Permissions Cache Structure

/// Cached user permissions for runtime checking
#[derive(Debug, Clone, Default)]
pub struct UserPermissions {
	pub user_id: i64,
	pub roles: Vec<String>,
	pub permissions: HashSet<String>,
}

impl UserPermissions {
	pub fn new(user_id: i64, roles: Vec<String>, permissions: Vec<String>) -> Self {
		Self {
			user_id,
			roles,
			permissions: permissions.into_iter().collect(),
		}
	}

	/// Check if user has a specific permission
	pub fn has_permission(&self, key: &str) -> bool {
		self.permissions.contains(key)
	}

	/// Check if user has any of the specified permissions
	pub fn has_any_permission(&self, keys: &[&str]) -> bool {
		keys.iter().any(|key| self.permissions.contains(*key))
	}

	/// Check if user has all of the specified permissions
	pub fn has_all_permissions(&self, keys: &[&str]) -> bool {
		keys.iter().all(|key| self.permissions.contains(*key))
	}

	/// Check if user has a specific role
	pub fn has_role(&self, role: &str) -> bool {
		self.roles.iter().any(|r| r == role)
	}
}

// endregion: --- User Permissions Cache Structure

// region:    --- Route Handler Registry

/// Kind of route handler permission requirement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteHandlerKind {
	/// Handler requires specific permission(s)
	Protected,
	/// Handler is public (no permission check)
	Public,
}

/// Registered route handler for compile-time collection
/// Used with inventory crate for startup validation
#[derive(Debug, Clone)]
pub struct RegisteredRouteHandler {
	/// Function name (used for matching with router registration)
	pub name: &'static str,
	/// Whether the handler is protected or public
	pub kind: RouteHandlerKind,
	/// Whether the handler has permission check code injected
	pub has_check: bool,
	/// Source module path
	pub source: &'static str,
}

// Note: The actual inventory::collect! for compile-time route handler registration
inventory::collect!(RegisteredRouteHandler);

/// Get all registered route handlers
pub fn get_registered_route_handlers() -> Vec<&'static RegisteredRouteHandler> {
	inventory::iter::<RegisteredRouteHandler>.into_iter().collect()
}

/// Validate that all handlers in the list are registered with permissions
/// Returns a list of unregistered handler names
pub fn validate_route_handlers(handler_names: &[&str]) -> Vec<String> {
	let registered: std::collections::HashSet<&str> = 
		inventory::iter::<RegisteredRouteHandler>
			.into_iter()
			.map(|h| h.name)
			.collect();
	
	handler_names
		.iter()
		.filter(|name| !registered.contains(*name))
		.map(|s| s.to_string())
		.collect()
}

/// Validate that handlers are registered within a specific module path
/// The module_prefix is used to filter handlers by their source module
pub fn validate_route_handlers_in_module(
	handler_names: &[&str],
	module_prefix: &str,
) -> Vec<String> {
	// Collect handlers registered from the specified module
	let registered: std::collections::HashSet<&str> = inventory::iter::<RegisteredRouteHandler>
		.into_iter()
		.filter(|h| h.source.contains(module_prefix))
		.map(|h| h.name)
		.collect();

	handler_names
		.iter()
		.filter(|name| !registered.contains(*name))
		.map(|s| s.to_string())
		.collect()
}

/// Panic if any handlers are not registered
/// Call this at startup to ensure all routes have permission annotations
pub fn ensure_all_handlers_registered(handler_names: &[&str]) {
	let unregistered = validate_route_handlers(handler_names);
	if !unregistered.is_empty() {
		panic!(
			"Route handlers without permission annotations: {:?}\n\
			All route handlers must use #[permission], #[rest_permission], or #[public] macro.",
			unregistered
		);
	}
}

/// Panic if any handlers are not registered within the specified module
/// This allows RPC and REST handlers to be validated separately
pub fn ensure_handlers_registered_in_module(handler_names: &[&str], module_prefix: &str) {
	let unregistered = validate_route_handlers_in_module(handler_names, module_prefix);
	if !unregistered.is_empty() {
		panic!(
			"Route handlers in module '{}' without permission annotations: {:?}\n\
			All route handlers must use #[permission], #[rest_permission], or #[public] macro.",
			module_prefix,
			unregistered
		);
	}

	// Also check for handlers that have registration but no permission check
	let no_check: Vec<&str> = inventory::iter::<RegisteredRouteHandler>
		.into_iter()
		.filter(|h| h.source.contains(module_prefix))
		.filter(|h| handler_names.contains(&h.name))
		.filter(|h| h.kind == RouteHandlerKind::Protected && !h.has_check)
		.map(|h| h.name)
		.collect();

	if !no_check.is_empty() {
		panic!(
			"Route handlers in module '{}' registered but without permission check: {:?}\n\
			Use #[permission] or #[rest_permission] instead of #[register_permission] to inject check code.",
			module_prefix,
			no_check
		);
	}
}

// endregion: --- Route Handler Registry
