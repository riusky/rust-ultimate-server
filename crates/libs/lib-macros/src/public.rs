//! Public route handler macro implementation

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Implementation of #[public] - marks a route handler as public (no permission check)
pub fn public_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let input_fn = parse_macro_input!(item as ItemFn);

	let fn_name = &input_fn.sig.ident;
	let fn_name_str = fn_name.to_string();

	let fn_vis = &input_fn.vis;
	let fn_sig = &input_fn.sig;
	let fn_block = &input_fn.block;
	let fn_attrs = &input_fn.attrs;

	let output = quote! {
		// Register route handler as public
		::inventory::submit! {
			::lib_core::model::acs::RegisteredRouteHandler {
				name: #fn_name_str,
				kind: ::lib_core::model::acs::RouteHandlerKind::Public,
				has_check: true, // Public handlers are explicitly marked, no check needed
				source: module_path!(),
			}
		}

		#(#fn_attrs)*
		#fn_vis #fn_sig #fn_block
	};

	output.into()
}
