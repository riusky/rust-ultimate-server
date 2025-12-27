// region:    --- Modules
pub mod routes_login;
pub mod routes_rest;
pub mod routes_rpc;
pub mod routes_user;
pub mod rests;
pub mod rpcs;

// endregion: --- Modules

// Note: Route handler validation is now done automatically in each module's
// rpc_router_builder() via generate_rpc_routes! macro.
// No need for a central validation function.
