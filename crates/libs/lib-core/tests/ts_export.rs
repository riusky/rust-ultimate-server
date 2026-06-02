//! Test file to export TypeScript types via ts-rs
//!
//! Run with: cargo test -p lib-core --features with-ts export_ts_types -- --nocapture

#[cfg(feature = "with-ts")]
mod ts_export {
	use lib_core::model::ts_export::export_registered_ts_types;

	#[test]
	fn export_ts_types() {
		let exported = export_registered_ts_types().expect("Failed to export TypeScript types");

		assert!(
			exported > 0,
			"Expected at least one registered TypeScript export"
		);
		println!("TypeScript types exported successfully! ({exported} types)");
	}
}
