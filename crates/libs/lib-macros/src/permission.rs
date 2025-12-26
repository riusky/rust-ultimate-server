//! Permission registration macro implementation

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Ident, ItemFn, LitStr, Token};

/// Permission attributes parsed from macro arguments
pub struct PermissionAttrs {
	pub key: String,
	pub group: String,
	pub display: String,
	pub description: String,
}

impl Parse for PermissionAttrs {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		// Try to parse as simple string literal first: "agent:create"
		if input.peek(LitStr) {
			let lit: LitStr = input.parse()?;
			let key = lit.value();
			return Ok(Self {
				key: key.clone(),
				group: String::new(),
				display: key,
				description: String::new(),
			});
		}

		// Parse as key-value pairs: key = "...", group = "...", display = "...", description = "..."
		let mut key = String::new();
		let mut group = String::new();
		let mut display = String::new();
		let mut description = String::new();

		while !input.is_empty() {
			let ident: Ident = input.parse()?;
			let _: Token![=] = input.parse()?;
			let lit: LitStr = input.parse()?;

			match ident.to_string().as_str() {
				"key" => key = lit.value(),
				"group" => group = lit.value(),
				"display" => display = lit.value(),
				"description" | "desc" => description = lit.value(),
				_ => {
					return Err(syn::Error::new_spanned(
						ident,
						"Unknown attribute. Expected: key, group, display, or description",
					))
				}
			}

			// Parse optional comma
			if input.peek(Token![,]) {
				let _: Token![,] = input.parse()?;
			}
		}

		if key.is_empty() {
			return Err(syn::Error::new(
				input.span(),
				"Permission key is required",
			));
		}

		if display.is_empty() {
			display = key.clone();
		}

		Ok(Self { key, group, display, description })
	}
}

/// Generate permission registration code
fn generate_registration(attrs: &PermissionAttrs) -> TokenStream2 {
	let key = &attrs.key;
	let group = &attrs.group;
	let display = &attrs.display;
	let description = &attrs.description;

	quote! {
		// Permission registration using inventory (compile-time collection)
		::inventory::submit! {
			::lib_core::model::acs::RegisteredPermission {
				key: #key,
				group: #group,
				display: #display,
				description: #description,
				source: module_path!(),
			}
		}
	}
}

/// Generate route handler registration code
fn generate_handler_registration(fn_name: &str) -> TokenStream2 {
	quote! {
		// Route handler registration for startup validation
		::inventory::submit! {
			::lib_core::model::acs::RegisteredRouteHandler {
				name: #fn_name,
				kind: ::lib_core::model::acs::RouteHandlerKind::Protected,
				source: module_path!(),
			}
		}
	}
}

/// Generate permission check code
fn generate_check(key: &str) -> TokenStream2 {
	quote! {
		ctx.require_permission(#key)?;
	}
}

/// Implementation of #[register_permission]
pub fn register_permission_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
	let attrs = parse_macro_input!(attr as PermissionAttrs);
	let input_fn = parse_macro_input!(item as ItemFn);

	let fn_name = &input_fn.sig.ident;
	let fn_name_str = fn_name.to_string();

	let registration = generate_registration(&attrs);
	let handler_registration = generate_handler_registration(&fn_name_str);

	let fn_vis = &input_fn.vis;
	let fn_sig = &input_fn.sig;
	let fn_block = &input_fn.block;
	let fn_attrs = &input_fn.attrs;

	let output = quote! {
		#registration
		#handler_registration

		#(#fn_attrs)*
		#fn_vis #fn_sig #fn_block
	};

	output.into()
}

/// Implementation of #[permission] - both register and check
pub fn permission_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
	let attrs = parse_macro_input!(attr as PermissionAttrs);
	let input_fn = parse_macro_input!(item as ItemFn);

	let fn_name = &input_fn.sig.ident;
	let fn_name_str = fn_name.to_string();

	let registration = generate_registration(&attrs);
	let handler_registration = generate_handler_registration(&fn_name_str);
	let check = generate_check(&attrs.key);

	let fn_vis = &input_fn.vis;
	let fn_sig = &input_fn.sig;
	let fn_block = &input_fn.block;
	let fn_attrs = &input_fn.attrs;

	// Extract the original function body statements
	let stmts = &fn_block.stmts;

	let output = quote! {
		#registration
		#handler_registration

		#(#fn_attrs)*
		#fn_vis #fn_sig {
			// Permission check
			#check

			// Original function body
			#(#stmts)*
		}
	};

	output.into()
}

// ============================================================================
// REST-specific macros (use ctx.0 for CtxW wrapper)
// ============================================================================

/// Generate permission check code for REST (uses ctx.0)
fn generate_rest_check(key: &str) -> TokenStream2 {
	quote! {
		ctx.0.require_permission(#key)?;
	}
}

/// Implementation of #[rest_permission] - both register and check for REST handlers
pub fn rest_permission_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
	let attrs = parse_macro_input!(attr as PermissionAttrs);
	let input_fn = parse_macro_input!(item as ItemFn);

	let fn_name = &input_fn.sig.ident;
	let fn_name_str = fn_name.to_string();

	let registration = generate_registration(&attrs);
	let handler_registration = generate_handler_registration(&fn_name_str);
	let check = generate_rest_check(&attrs.key);

	let fn_vis = &input_fn.vis;
	let fn_sig = &input_fn.sig;
	let fn_block = &input_fn.block;
	let fn_attrs = &input_fn.attrs;

	let stmts = &fn_block.stmts;

	let output = quote! {
		#registration
		#handler_registration

		#(#fn_attrs)*
		#fn_vis #fn_sig {
			// Permission check (REST: unwrap CtxW)
			#check

			// Original function body
			#(#stmts)*
		}
	};

	output.into()
}
