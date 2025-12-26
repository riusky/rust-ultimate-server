//! lib-macros - Procedural macros for ACS (Access Control System)
//!
//! This crate provides attribute macros for permission registration and checking:
//!
//! - `#[register_permission]` - Register a permission at compile time
//! - `#[require_permission]` - Check permission at runtime
//! - `#[permission]` - Both register and check (convenience macro)
//! - `#[public]` - Mark handler as public (no permission check)

use proc_macro::TokenStream;

mod permission;
mod public;
mod require;

/// Register a permission at compile time without runtime checking.
///
/// # Usage
///
/// ```rust,ignore
/// #[register_permission("agent:export")]
/// pub async fn export_agents(...) { ... }
///
/// #[register_permission(key = "agent:clone", group = "Agent Management", display = "Clone Agent")]
/// pub async fn clone_agent(...) { ... }
/// ```
#[proc_macro_attribute]
pub fn register_permission(attr: TokenStream, item: TokenStream) -> TokenStream {
	permission::register_permission_impl(attr, item)
}

/// Check permission at runtime. Does not register the permission.
///
/// # Usage
///
/// ```rust,ignore
/// #[require_permission("agent:read")]
/// pub async fn get_agent(ctx: CtxW, ...) -> Result<...> { ... }
/// ```
#[proc_macro_attribute]
pub fn require_permission(attr: TokenStream, item: TokenStream) -> TokenStream {
	require::require_permission_impl(attr, item)
}

/// Check multiple permissions at runtime (all must be satisfied).
///
/// # Usage
///
/// ```rust,ignore
/// #[require_permissions("agent:read", "report:view")]
/// pub async fn agent_report(ctx: CtxW, ...) -> Result<...> { ... }
/// ```
#[proc_macro_attribute]
pub fn require_permissions(attr: TokenStream, item: TokenStream) -> TokenStream {
	require::require_permissions_impl(attr, item)
}

/// Check multiple permissions at runtime (any one must be satisfied).
///
/// # Usage
///
/// ```rust,ignore
/// #[require_any_permission("agent:read", "agent:admin")]
/// pub async fn view_agent(ctx: CtxW, ...) -> Result<...> { ... }
/// ```
#[proc_macro_attribute]
pub fn require_any_permission(attr: TokenStream, item: TokenStream) -> TokenStream {
	require::require_any_permission_impl(attr, item)
}

/// Both register and check permission (convenience macro).
///
/// # Usage
///
/// ```rust,ignore
/// #[permission("agent:export")]
/// pub async fn export_agents(ctx: Ctx, ...) -> Result<...> { ... }
///
/// #[permission(key = "agent:clone", group = "Agent Management", display = "Clone Agent")]
/// pub async fn clone_agent(ctx: Ctx, ...) -> Result<...> { ... }
/// ```
#[proc_macro_attribute]
pub fn permission(attr: TokenStream, item: TokenStream) -> TokenStream {
	permission::permission_impl(attr, item)
}

// ============================================================================
// REST-specific macros (use ctx.0 for CtxW wrapper)
// ============================================================================

/// Both register and check permission for REST handlers (uses ctx.0 for CtxW).
///
/// # Usage
///
/// ```rust,ignore
/// #[rest_permission("agent:export")]
/// pub async fn export_agents(ctx: CtxW, ...) -> Result<...> { ... }
///
/// #[rest_permission(key = "agent:clone", group = "Agent Management", display = "Clone Agent")]
/// pub async fn clone_agent(ctx: CtxW, ...) -> Result<...> { ... }
/// ```
#[proc_macro_attribute]
pub fn rest_permission(attr: TokenStream, item: TokenStream) -> TokenStream {
	permission::rest_permission_impl(attr, item)
}

/// Check permission at runtime for REST handlers (uses ctx.0 for CtxW).
///
/// # Usage
///
/// ```rust,ignore
/// #[rest_require_permission("agent:read")]
/// pub async fn get_agent(ctx: CtxW, ...) -> Result<...> { ... }
/// ```
#[proc_macro_attribute]
pub fn rest_require_permission(attr: TokenStream, item: TokenStream) -> TokenStream {
	require::rest_require_permission_impl(attr, item)
}

/// Check multiple permissions at runtime for REST handlers (all must be satisfied).
///
/// # Usage
///
/// ```rust,ignore
/// #[rest_require_permissions("agent:read", "report:view")]
/// pub async fn agent_report(ctx: CtxW, ...) -> Result<...> { ... }
/// ```
#[proc_macro_attribute]
pub fn rest_require_permissions(attr: TokenStream, item: TokenStream) -> TokenStream {
	require::rest_require_permissions_impl(attr, item)
}

/// Check multiple permissions at runtime for REST handlers (any one must be satisfied).
///
/// # Usage
///
/// ```rust,ignore
/// #[rest_require_any_permission("agent:read", "agent:admin")]
/// pub async fn view_agent(ctx: CtxW, ...) -> Result<...> { ... }
/// ```
#[proc_macro_attribute]
pub fn rest_require_any_permission(attr: TokenStream, item: TokenStream) -> TokenStream {
	require::rest_require_any_permission_impl(attr, item)
}

// ============================================================================
// Public handler macro (no permission check)
// ============================================================================

/// Mark a route handler as public (no permission check required).
/// This still registers the handler for startup validation.
///
/// # Usage
///
/// ```rust,ignore
/// #[public]
/// pub async fn health_check() -> Result<...> { ... }
///
/// #[public]
/// pub async fn login(payload: Json<LoginPayload>) -> Result<...> { ... }
/// ```
#[proc_macro_attribute]
pub fn public(attr: TokenStream, item: TokenStream) -> TokenStream {
	public::public_impl(attr, item)
}
