use lib_core::model::agent::{
	Agent, AgentBmc, AgentFilter, AgentForCreate, AgentForUpdate,
};
use lib_rest_core::prelude::*;

generate_common_rest_fns!(
	Bmc: AgentBmc,
	Entity: Agent,
	ForCreate: AgentForCreate,
	ForUpdate: AgentForUpdate,
	Filter: AgentFilter,
	Suffix: agent
);
