//! Permission synchronization and validation
//!
//! This module provides functions for:
//! - Syncing registered permissions to the database at startup
//! - Validating that all permissions are properly registered

use crate::ctx::Ctx;
use crate::model::acs::{PermissionBmc, RegisteredPermission};
use crate::model::ModelManager;
use crate::model::Result;
use std::collections::HashSet;
use tracing::{debug, info, warn};

/// Synchronize registered permissions to the database.
///
/// This function should be called at application startup to ensure
/// all permissions defined in code are available in the database.
///
/// # Process
/// 1. Collect all permissions registered via inventory
/// 2. Get existing permissions from database
/// 3. Create new permissions that don't exist
/// 4. Update permissions with changed metadata
/// 5. Warn about orphaned permissions (in DB but not in code)
pub async fn sync_permissions(mm: &ModelManager) -> Result<SyncResult> {
	let ctx = Ctx::root_ctx();
	let mut result = SyncResult::default();

	// Collect all registered permissions
	let registered: Vec<&RegisteredPermission> =
		inventory::iter::<RegisteredPermission>.into_iter().collect();

	info!(
		"Permission sync: Found {} registered permissions",
		registered.len()
	);

	// Get existing permissions from database
	let existing_keys: HashSet<String> =
		PermissionBmc::list_all_keys(&ctx, mm).await?.into_iter().collect();

	// Process each registered permission
	for perm in &registered {
		if existing_keys.contains(perm.key) {
			// Permission exists, update if needed
			result.existing += 1;
			debug!("Permission exists: {}", perm.key);
		} else {
			// Create new permission
			match PermissionBmc::upsert_from_registered(
				&ctx,
				mm,
				perm.key,
				perm.group,
				perm.display,
			)
			.await
			{
				Ok(_) => {
					result.created += 1;
					info!("Created permission: {} ({})", perm.key, perm.display);
				}
				Err(e) => {
					result.errors += 1;
					warn!("Failed to create permission {}: {:?}", perm.key, e);
				}
			}
		}
	}

	// Check for orphaned permissions (in DB but not in code)
	let registered_keys: HashSet<&str> = registered.iter().map(|p| p.key).collect();
	for key in &existing_keys {
		if !registered_keys.contains(key.as_str()) {
			result.orphaned.push(key.clone());
			warn!(
				"Orphaned permission in database (not in code): {}",
				key
			);
		}
	}

	info!(
		"Permission sync complete: {} created, {} existing, {} orphaned, {} errors",
		result.created, result.existing, result.orphaned.len(), result.errors
	);

	Ok(result)
}

/// Result of permission synchronization
#[derive(Debug, Default)]
pub struct SyncResult {
	/// Number of new permissions created
	pub created: usize,
	/// Number of existing permissions (already in DB)
	pub existing: usize,
	/// Permissions in DB but not in code
	pub orphaned: Vec<String>,
	/// Number of errors during sync
	pub errors: usize,
}

impl SyncResult {
	pub fn is_success(&self) -> bool {
		self.errors == 0
	}

	pub fn has_orphaned(&self) -> bool {
		!self.orphaned.is_empty()
	}
}

/// Validate that all required permissions are registered.
///
/// This is a compile-time/startup check to ensure no permissions are missing.
/// Call this function at startup to get a list of all registered permissions.
pub fn list_registered_permissions() -> Vec<PermissionInfo> {
	inventory::iter::<RegisteredPermission>
		.into_iter()
		.map(|p| PermissionInfo {
			key: p.key.to_string(),
			group: p.group.to_string(),
			display: p.display.to_string(),
			source: p.source.to_string(),
		})
		.collect()
}

/// Permission information for display
#[derive(Debug, Clone)]
pub struct PermissionInfo {
	pub key: String,
	pub group: String,
	pub display: String,
	pub source: String,
}

/// Get permissions grouped by their group name
pub fn list_permissions_by_group() -> std::collections::HashMap<String, Vec<PermissionInfo>> {
	let permissions = list_registered_permissions();
	let mut grouped: std::collections::HashMap<String, Vec<PermissionInfo>> =
		std::collections::HashMap::new();

	for perm in permissions {
		let group = if perm.group.is_empty() {
			"Other".to_string()
		} else {
			perm.group.clone()
		};
		grouped.entry(group).or_default().push(perm);
	}

	grouped
}

// region:    --- Macros for CRUD permissions registration

/// Register standard CRUD permissions for a resource.
///
/// This macro registers the following permissions:
/// - `{resource}:create`
/// - `{resource}:read`
/// - `{resource}:update`
/// - `{resource}:delete`
/// - `{resource}:list`
///
/// # Usage
///
/// ```rust,ignore
/// // With literal string (no description):
/// register_crud_permissions!("agent", "Agent", "Agent Management");
///
/// // With description:
/// register_crud_permissions!("agent", "Agent", "Agent Management", "Manage agent entities");
///
/// // With stringify! (used by generate_common_rpc_fns/rest_fns):
/// register_crud_permissions!(stringify!(agent), "Agent", "Agent Management", "Manage agent entities");
/// ```
#[macro_export]
macro_rules! register_crud_permissions {
	// With description
	($resource:expr, $display:literal, $group:literal, $desc:literal) => {
		::inventory::submit! {
			$crate::model::acs::RegisteredPermission {
				key: concat!($resource, ":create"),
				group: $group,
				display: concat!("Create ", $display),
				description: concat!("Create a new ", $desc),
				source: module_path!(),
			}
		}
		::inventory::submit! {
			$crate::model::acs::RegisteredPermission {
				key: concat!($resource, ":read"),
				group: $group,
				display: concat!("Read ", $display),
				description: concat!("View details of ", $desc),
				source: module_path!(),
			}
		}
		::inventory::submit! {
			$crate::model::acs::RegisteredPermission {
				key: concat!($resource, ":update"),
				group: $group,
				display: concat!("Update ", $display),
				description: concat!("Modify existing ", $desc),
				source: module_path!(),
			}
		}
		::inventory::submit! {
			$crate::model::acs::RegisteredPermission {
				key: concat!($resource, ":delete"),
				group: $group,
				display: concat!("Delete ", $display),
				description: concat!("Remove ", $desc),
				source: module_path!(),
			}
		}
		::inventory::submit! {
			$crate::model::acs::RegisteredPermission {
				key: concat!($resource, ":list"),
				group: $group,
				display: concat!("List ", $display),
				description: concat!("List all ", $desc),
				source: module_path!(),
			}
		}
	};
	// Without description (backward compatible)
	($resource:expr, $display:literal, $group:literal) => {
		::inventory::submit! {
			$crate::model::acs::RegisteredPermission {
				key: concat!($resource, ":create"),
				group: $group,
				display: concat!("Create ", $display),
				description: "",
				source: module_path!(),
			}
		}
		::inventory::submit! {
			$crate::model::acs::RegisteredPermission {
				key: concat!($resource, ":read"),
				group: $group,
				display: concat!("Read ", $display),
				description: "",
				source: module_path!(),
			}
		}
		::inventory::submit! {
			$crate::model::acs::RegisteredPermission {
				key: concat!($resource, ":update"),
				group: $group,
				display: concat!("Update ", $display),
				description: "",
				source: module_path!(),
			}
		}
		::inventory::submit! {
			$crate::model::acs::RegisteredPermission {
				key: concat!($resource, ":delete"),
				group: $group,
				display: concat!("Delete ", $display),
				description: "",
				source: module_path!(),
			}
		}
		::inventory::submit! {
			$crate::model::acs::RegisteredPermission {
				key: concat!($resource, ":list"),
				group: $group,
				display: concat!("List ", $display),
				description: "",
				source: module_path!(),
			}
		}
	};
}

// endregion: --- Macros for CRUD permissions registration

// region:    --- Macros for CRUD route handlers registration

/// Register standard CRUD route handlers for startup validation.
///
/// This macro registers handlers for:
/// - `create_{resource}`
/// - `get_{resource}`
/// - `list_{resource}s`
/// - `update_{resource}`
/// - `delete_{resource}`
///
/// # Usage
///
/// ```rust,ignore
/// register_crud_handlers!(stringify!(agent));
/// ```
#[macro_export]
macro_rules! register_crud_handlers {
	($resource:expr) => {
		::inventory::submit! {
			$crate::model::acs::RegisteredRouteHandler {
				name: concat!("create_", $resource),
				kind: $crate::model::acs::RouteHandlerKind::Protected,
				has_check: true,
				source: module_path!(),
			}
		}
		::inventory::submit! {
			$crate::model::acs::RegisteredRouteHandler {
				name: concat!("get_", $resource),
				kind: $crate::model::acs::RouteHandlerKind::Protected,
				has_check: true,
				source: module_path!(),
			}
		}
		::inventory::submit! {
			$crate::model::acs::RegisteredRouteHandler {
				name: concat!("list_", $resource, "s"),
				kind: $crate::model::acs::RouteHandlerKind::Protected,
				has_check: true,
				source: module_path!(),
			}
		}
		::inventory::submit! {
			$crate::model::acs::RegisteredRouteHandler {
				name: concat!("update_", $resource),
				kind: $crate::model::acs::RouteHandlerKind::Protected,
				has_check: true,
				source: module_path!(),
			}
		}
		::inventory::submit! {
			$crate::model::acs::RegisteredRouteHandler {
				name: concat!("delete_", $resource),
				kind: $crate::model::acs::RouteHandlerKind::Protected,
				has_check: true,
				source: module_path!(),
			}
		}
	};
}

// endregion: --- Macros for CRUD route handlers registration

// region:    --- Macros for route handler validation

/// Validate that all specified handlers have permission annotations.
/// Use this macro at startup to ensure all route handlers are properly annotated.
///
/// # Usage
///
/// ```rust,ignore
/// validate_handlers!(create_agent, get_agent, list_agents);
/// ```
///
/// This will panic at startup if any handler is not registered
/// via permission macros (#[permission], #[rest_permission], #[public], etc.)
#[macro_export]
macro_rules! validate_handlers {
	($($handler:ident),* $(,)?) => {{
		let handler_names: &[&str] = &[
			$(stringify!($handler)),*
		];
		$crate::model::acs::ensure_all_handlers_registered(handler_names);
	}};
}

/// Combines validation and router building in one macro.
/// Generate RPC routes with permission validation.
/// Validates all handlers have permission annotations, then builds the RPC router.
/// Uses the current module path to filter handlers (avoids collision with REST handlers).
///
/// # Usage
///
/// ```rust,ignore
/// pub fn rpc_router_builder() -> RouterBuilder {
///     generate_rpc_routes!(
///         create_agent,
///         get_agent,
///         list_agents,
///     )
/// }
/// ```
///
/// This replaces the need for separate validate_handlers! and router_builder! calls.
#[macro_export]
macro_rules! generate_rpc_routes {
	($($handler:ident),* $(,)?) => {{
		// Validate all handlers have permission annotations in this module
		let handler_names: &[&str] = &[
			$(stringify!($handler)),*
		];
		$crate::model::acs::ensure_handlers_registered_in_module(
			handler_names,
			module_path!(),
		);

		// Build the router
		::rpc_router::router_builder!($($handler),*)
	}};
}

/// Validates REST handlers have permission annotations.
/// Uses the current module path to filter handlers (avoids collision with RPC handlers).
///
/// # Usage
///
/// ```rust,ignore
/// validate_rest_handlers!(clone_agent, get_agent_stats);
/// ```
#[macro_export]
macro_rules! validate_rest_handlers {
	($($handler:ident),* $(,)?) => {{
		let handler_names: &[&str] = &[
			$(stringify!($handler)),*
		];
		// Use module_path!() to only validate handlers from this module
		$crate::model::acs::ensure_handlers_registered_in_module(
			handler_names,
			module_path!(),
		);
	}};
}

/// Generate custom REST routes with permission validation.
/// Validates all handlers have permission annotations, then builds the Axum router.
/// This ensures you can't forget to add handlers to validation list.
///
/// # Usage
///
/// ```rust,ignore
/// pub fn custom_agent_routes() -> axum::Router<ModelManager> {
///     generate_custom_rest_routes!(
///         get "/stats" => get_agent_stats,
///         post "/{id}/clone" => clone_agent,
///         get "/{id}/export" => export_agent,
///     )
/// }
/// ```
#[macro_export]
macro_rules! generate_custom_rest_routes {
	($($method:ident $path:literal => $handler:ident),* $(,)?) => {{
		// Validate all handlers have permission annotations in this module
		let handler_names: &[&str] = &[
			$(stringify!($handler)),*
		];
		$crate::model::acs::ensure_handlers_registered_in_module(
			handler_names,
			module_path!(),
		);

		// Build the router
		use ::axum::routing::{get, post, put, patch, delete};
		::axum::Router::new()
			$(.route($path, $method($handler)))*
	}};
}

// endregion: --- Macros for route handler validation
