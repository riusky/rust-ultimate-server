use lib_core::model::agent::{
	Agent, AgentBmc, AgentFilter, AgentForCreate, AgentForUpdate,
};
use lib_rest_core::prelude::*;
use lib_web::middleware::mw_auth::CtxW;

generate_common_rest_fns!(
	Bmc: AgentBmc,
	Entity: Agent,
	ForCreate: AgentForCreate,
	ForUpdate: AgentForUpdate,
	Filter: AgentFilter,
	Suffix: agent
);
