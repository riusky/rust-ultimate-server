// region:    --- Modules

pub mod agent_rpc;
pub mod conv_rpc;
pub mod role_permission_rpc;
pub mod role_rpc;
pub mod user_info_rpc;
pub mod user_role_rpc;
pub mod user_rpc;

use rpc_router::{Router, RouterBuilder};

// endregion: --- Modules

pub fn all_rpc_router_builder() -> RouterBuilder {
	Router::builder()
		.extend(agent_rpc::rpc_router_builder())
		.extend(conv_rpc::rpc_router_builder())
		.extend(role_permission_rpc::rpc_router_builder())
		.extend(role_rpc::rpc_router_builder())
		.extend(user_info_rpc::rpc_router_builder())
		.extend(user_role_rpc::rpc_router_builder())
		.extend(user_rpc::rpc_router_builder())
}
