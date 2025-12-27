//! Permission requirement macro implementation

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, ItemFn, LitStr, Token};

/// Single permission key
struct SinglePermission {
	key: String,
}

impl Parse for SinglePermission {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let lit: LitStr = input.parse()?;
		Ok(Self { key: lit.value() })
	}
}

/// Multiple permission keys
struct MultiplePermissions {
	keys: Vec<String>,
}

impl Parse for MultiplePermissions {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let lits: Punctuated<LitStr, Token![,]> = Punctuated::parse_terminated(input)?;
		Ok(Self {
			keys: lits.into_iter().map(|l| l.value()).collect(),
		})
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
				has_check: true, // These macros inject permission checks
				source: module_path!(),
			}
		}
	}
}

/// Implementation of #[require_permission]
pub fn require_permission_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
	let perm = parse_macro_input!(attr as SinglePermission);
	let input_fn = parse_macro_input!(item as ItemFn);

	let fn_name = &input_fn.sig.ident;
	let fn_name_str = fn_name.to_string();
	let handler_registration = generate_handler_registration(&fn_name_str);

	let key = &perm.key;

	let fn_vis = &input_fn.vis;
	let fn_sig = &input_fn.sig;
	let fn_block = &input_fn.block;
	let fn_attrs = &input_fn.attrs;

	let stmts = &fn_block.stmts;

	let output = quote! {
		#handler_registration

		#(#fn_attrs)*
		#fn_vis #fn_sig {
			// Permission check
			ctx.require_permission(#key)?;

			// Original function body
			#(#stmts)*
		}
	};

	output.into()
}

/// Implementation of #[require_permissions] (all must be satisfied)
pub fn require_permissions_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
	let perms = parse_macro_input!(attr as MultiplePermissions);
	let input_fn = parse_macro_input!(item as ItemFn);

	let fn_name = &input_fn.sig.ident;
	let fn_name_str = fn_name.to_string();
	let handler_registration = generate_handler_registration(&fn_name_str);

	let keys = &perms.keys;

	let fn_vis = &input_fn.vis;
	let fn_sig = &input_fn.sig;
	let fn_block = &input_fn.block;
	let fn_attrs = &input_fn.attrs;

	let stmts = &fn_block.stmts;

	// Generate check for each permission
	let checks: Vec<TokenStream2> = keys
		.iter()
		.map(|key| {
			quote! {
				ctx.require_permission(#key)?;
			}
		})
		.collect();

	let output = quote! {
		#handler_registration

		#(#fn_attrs)*
		#fn_vis #fn_sig {
			// Permission checks (all required)
			#(#checks)*

			// Original function body
			#(#stmts)*
		}
	};

	output.into()
}

/// Implementation of #[require_any_permission] (any one must be satisfied)
pub fn require_any_permission_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
	let perms = parse_macro_input!(attr as MultiplePermissions);
	let input_fn = parse_macro_input!(item as ItemFn);

	let fn_name = &input_fn.sig.ident;
	let fn_name_str = fn_name.to_string();
	let handler_registration = generate_handler_registration(&fn_name_str);

	let keys = &perms.keys;

	let fn_vis = &input_fn.vis;
	let fn_sig = &input_fn.sig;
	let fn_block = &input_fn.block;
	let fn_attrs = &input_fn.attrs;

	let stmts = &fn_block.stmts;

	let output = quote! {
		#handler_registration

		#(#fn_attrs)*
		#fn_vis #fn_sig {
			// Permission check (any one is sufficient)
			ctx.require_any_permission(&[#(#keys),*])?;

			// Original function body
			#(#stmts)*
		}
	};

	output.into()
}

// ============================================================================
// REST-specific macros (use ctx.0 for CtxW wrapper)
// ============================================================================

/// Implementation of #[rest_require_permission] - for REST handlers with CtxW
pub fn rest_require_permission_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
	let perm = parse_macro_input!(attr as SinglePermission);
	let input_fn = parse_macro_input!(item as ItemFn);

	let fn_name = &input_fn.sig.ident;
	let fn_name_str = fn_name.to_string();
	let handler_registration = generate_handler_registration(&fn_name_str);

	let key = &perm.key;

	let fn_vis = &input_fn.vis;
	let fn_sig = &input_fn.sig;
	let fn_block = &input_fn.block;
	let fn_attrs = &input_fn.attrs;

	let stmts = &fn_block.stmts;

	let output = quote! {
		#handler_registration

		#(#fn_attrs)*
		#fn_vis #fn_sig {
			// Permission check (REST: unwrap CtxW)
			ctx.0.require_permission(#key)?;

			// Original function body
			#(#stmts)*
		}
	};

	output.into()
}

/// Implementation of #[rest_require_permissions] - for REST handlers with CtxW (all must pass)
pub fn rest_require_permissions_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
	let perms = parse_macro_input!(attr as MultiplePermissions);
	let input_fn = parse_macro_input!(item as ItemFn);

	let fn_name = &input_fn.sig.ident;
	let fn_name_str = fn_name.to_string();
	let handler_registration = generate_handler_registration(&fn_name_str);

	let keys = &perms.keys;

	let fn_vis = &input_fn.vis;
	let fn_sig = &input_fn.sig;
	let fn_block = &input_fn.block;
	let fn_attrs = &input_fn.attrs;

	let stmts = &fn_block.stmts;

	let output = quote! {
		#handler_registration

		#(#fn_attrs)*
		#fn_vis #fn_sig {
			// Permission checks (REST: unwrap CtxW, all required)
			ctx.0.require_all_permissions(&[#(#keys),*])?;

			// Original function body
			#(#stmts)*
		}
	};

	output.into()
}

/// Implementation of #[rest_require_any_permission] - for REST handlers with CtxW (any one passes)
pub fn rest_require_any_permission_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
	let perms = parse_macro_input!(attr as MultiplePermissions);
	let input_fn = parse_macro_input!(item as ItemFn);

	let fn_name = &input_fn.sig.ident;
	let fn_name_str = fn_name.to_string();
	let handler_registration = generate_handler_registration(&fn_name_str);

	let keys = &perms.keys;

	let fn_vis = &input_fn.vis;
	let fn_sig = &input_fn.sig;
	let fn_block = &input_fn.block;
	let fn_attrs = &input_fn.attrs;

	let stmts = &fn_block.stmts;

	let output = quote! {
		#handler_registration

		#(#fn_attrs)*
		#fn_vis #fn_sig {
			// Permission check (REST: unwrap CtxW, any one is sufficient)
			ctx.0.require_any_permission(&[#(#keys),*])?;

			// Original function body
			#(#stmts)*
		}
	};

	output.into()
}
