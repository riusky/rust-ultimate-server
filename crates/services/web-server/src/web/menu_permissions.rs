//! Menu Permissions - Register frontend menu access permissions
//!
//! This module registers permissions for controlling frontend menu visibility.
//! These permissions are used for UI display control, not for data access.
//!
//! Permission naming convention: `menu:{module}:{page}`
//! - menu:system:* - System management menus
//! - menu:demo:* - Demo feature menus

// region:    --- System Menus

// System management menu permissions
::inventory::submit! {
	::lib_core::model::acs::RegisteredPermission {
		key: "menu:system",
		group: "System Menus",
		display: "System Management",
		description: "Access to system management section",
		source: module_path!(),
	}
}

::inventory::submit! {
	::lib_core::model::acs::RegisteredPermission {
		key: "menu:system:user",
		group: "System Menus",
		display: "User Management Menu",
		description: "Access to user management page",
		source: module_path!(),
	}
}

::inventory::submit! {
	::lib_core::model::acs::RegisteredPermission {
		key: "menu:system:role",
		group: "System Menus",
		display: "Role Management Menu",
		description: "Access to role management page",
		source: module_path!(),
	}
}

::inventory::submit! {
	::lib_core::model::acs::RegisteredPermission {
		key: "menu:system:permission",
		group: "System Menus",
		display: "Permission Management Menu",
		description: "Access to permission management page",
		source: module_path!(),
	}
}

// endregion: --- System Menus

// region:    --- Demo Menus

// Demo feature menu permissions
::inventory::submit! {
	::lib_core::model::acs::RegisteredPermission {
		key: "menu:demo",
		group: "Demo Menus",
		display: "Demo Section",
		description: "Access to demo feature section",
		source: module_path!(),
	}
}

::inventory::submit! {
	::lib_core::model::acs::RegisteredPermission {
		key: "menu:demo:agent",
		group: "Demo Menus",
		display: "Agent Demo Menu",
		description: "Access to agent demo page",
		source: module_path!(),
	}
}

::inventory::submit! {
	::lib_core::model::acs::RegisteredPermission {
		key: "menu:demo:task",
		group: "Demo Menus",
		display: "Task Demo Menu",
		description: "Access to task demo page",
		source: module_path!(),
	}
}

::inventory::submit! {
	::lib_core::model::acs::RegisteredPermission {
		key: "menu:demo:conv",
		group: "Demo Menus",
		display: "Conversation Demo Menu",
		description: "Access to conversation demo page",
		source: module_path!(),
	}
}

::inventory::submit! {
	::lib_core::model::acs::RegisteredPermission {
		key: "menu:demo:data-table",
		group: "Demo Menus",
		display: "Data Table Demo Menu",
		description: "Access to data table demo page",
		source: module_path!(),
	}
}

::inventory::submit! {
	::lib_core::model::acs::RegisteredPermission {
		key: "menu:demo:charts",
		group: "Demo Menus",
		display: "Charts Demo Menu",
		description: "Access to charts demo page",
		source: module_path!(),
	}
}

// endregion: --- Demo Menus

// region:    --- Dashboard Menus

::inventory::submit! {
	::lib_core::model::acs::RegisteredPermission {
		key: "menu:dashboard",
		group: "Dashboard Menus",
		display: "Dashboard",
		description: "Access to main dashboard",
		source: module_path!(),
	}
}

::inventory::submit! {
	::lib_core::model::acs::RegisteredPermission {
		key: "menu:dashboard:overview",
		group: "Dashboard Menus",
		display: "Overview Dashboard",
		description: "Access to overview dashboard page",
		source: module_path!(),
	}
}

::inventory::submit! {
	::lib_core::model::acs::RegisteredPermission {
		key: "menu:dashboard:analytics",
		group: "Dashboard Menus",
		display: "Analytics Dashboard",
		description: "Access to analytics dashboard page",
		source: module_path!(),
	}
}

// endregion: --- Dashboard Menus
