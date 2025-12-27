//! Derive macro for implementing IntoResponse with extension storage.
//!
//! This macro generates an IntoResponse implementation that stores
//! the error in response extensions for middleware to process.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Generates IntoResponse implementation that stores error in extensions.
///
/// # Generated Code
///
/// ```rust,ignore
/// impl axum::response::IntoResponse for Error {
///     fn into_response(self) -> axum::response::Response {
///         let mut response = axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
///         response.extensions_mut().insert(std::sync::Arc::new(self));
///         response
///     }
/// }
/// ```
pub fn into_response_ext_impl(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = &input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let expanded = quote! {
		impl #impl_generics ::axum::response::IntoResponse for #name #ty_generics #where_clause {
			fn into_response(self) -> ::axum::response::Response {
				use ::axum::response::IntoResponse;
				let mut response = ::axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
				response.extensions_mut().insert(::std::sync::Arc::new(self));
				response
			}
		}
	};

	TokenStream::from(expanded)
}
