//! Test file to export TypeScript types via ts-rs
//!
//! Run with: cargo test -p lib-core --features with-ts export_ts_types -- --nocapture

#[cfg(feature = "with-ts")]
mod ts_export {
    use lib_core::model::acs::{Permission, PermissionLite, Role, RoleLite};
    use lib_core::model::agent::Agent;
    use lib_core::model::user_info::{UserGender, UserInfo, UserStatus};
    use lib_core::model::{User, UserTyp};
    use ts_rs::TS;

    #[test]
    fn export_ts_types() {
        // Export User types
        UserTyp::export_all().expect("Failed to export UserTyp");
        User::export_all().expect("Failed to export User");

        // Export UserInfo types
        UserGender::export_all().expect("Failed to export UserGender");
        UserStatus::export_all().expect("Failed to export UserStatus");
        UserInfo::export_all().expect("Failed to export UserInfo");

        // Export ACS types
        Permission::export_all().expect("Failed to export Permission");
        PermissionLite::export_all().expect("Failed to export PermissionLite");
        Role::export_all().expect("Failed to export Role");
        RoleLite::export_all().expect("Failed to export RoleLite");

        // Export Agent types
        Agent::export_all().expect("Failed to export Agent");

        println!("TypeScript types exported successfully!");
    }
}
