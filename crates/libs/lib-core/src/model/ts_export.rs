use ts_rs::ExportError;

pub struct RegisteredTsExport {
	pub name: &'static str,
	pub source: &'static str,
	pub export: fn() -> Result<(), ExportError>,
}

inventory::collect!(RegisteredTsExport);

pub fn export_registered_ts_types() -> Result<usize, String> {
	let mut exported = 0;

	for registered in inventory::iter::<RegisteredTsExport> {
		(registered.export)().map_err(|err| {
			format!(
				"Failed to export TypeScript type {} from {}: {}",
				registered.name, registered.source, err
			)
		})?;
		exported += 1;
	}

	Ok(exported)
}
