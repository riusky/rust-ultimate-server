use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, parse_quote, Ident, Item, LitStr, Token};

struct FrontendTypeAttrs {
	dir: String,
}

impl Parse for FrontendTypeAttrs {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if input.peek(LitStr) {
			let lit: LitStr = input.parse()?;
			return Ok(Self { dir: lit.value() });
		}

		let ident: Ident = input.parse()?;
		let _: Token![=] = input.parse()?;
		let lit: LitStr = input.parse()?;

		if ident != "dir" {
			return Err(syn::Error::new_spanned(
				ident,
				"Unknown attribute. Expected: dir",
			));
		}

		Ok(Self { dir: lit.value() })
	}
}

pub fn frontend_type_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
	let attrs = parse_macro_input!(attr as FrontendTypeAttrs);
	let mut input_item = parse_macro_input!(item as Item);

	let ident = match append_ts_attrs(&mut input_item, &attrs.dir) {
		Ok(ident) => ident,
		Err(err) => return err.to_compile_error().into(),
	};

	let export_fn = format_ident!("__frontend_type_export_{}", ident);
	let type_name = ident.to_string();
	let registration = generate_registration(&ident, &export_fn, &type_name);

	quote! {
		#input_item

		#registration
	}
	.into()
}

fn append_ts_attrs(item: &mut Item, dir: &str) -> syn::Result<Ident> {
	let export_to = format!(
		"../../../../cmx-vue-ultimate-starter/src/services/types/{}/",
		dir.trim_matches('/')
	);
	let export_to_lit = LitStr::new(&export_to, proc_macro2::Span::call_site());
	let derive_attr = parse_quote!(#[derive(ts_rs::TS)]);
	let ts_attr = parse_quote!(#[ts(export, export_to = #export_to_lit)]);

	match item {
		Item::Struct(item_struct) => {
			if !item_struct.generics.params.is_empty() {
				return Err(syn::Error::new_spanned(
					&item_struct.generics,
					"frontend_type does not support generic structs",
				));
			}

			item_struct.attrs.push(derive_attr);
			item_struct.attrs.push(ts_attr);
			Ok(item_struct.ident.clone())
		}
		Item::Enum(item_enum) => {
			if !item_enum.generics.params.is_empty() {
				return Err(syn::Error::new_spanned(
					&item_enum.generics,
					"frontend_type does not support generic enums",
				));
			}

			item_enum.attrs.push(derive_attr);
			item_enum.attrs.push(ts_attr);
			Ok(item_enum.ident.clone())
		}
		_ => Err(syn::Error::new_spanned(
			item,
			"frontend_type can only be used on structs and enums",
		)),
	}
}

fn generate_registration(ident: &Ident, export_fn: &Ident, type_name: &str) -> TokenStream2 {
	quote! {
		#[allow(non_snake_case)]
		fn #export_fn() -> ::std::result::Result<(), ::ts_rs::ExportError> {
			<#ident as ::ts_rs::TS>::export_all()
		}

		::inventory::submit! {
			::lib_core::model::ts_export::RegisteredTsExport {
				name: #type_name,
				source: module_path!(),
				export: #export_fn,
			}
		}
	}
}
