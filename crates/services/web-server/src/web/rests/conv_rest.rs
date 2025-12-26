use lib_core::model::conv::{
	Conv, ConvBmc, ConvFilter, ConvForCreate, ConvForUpdate,
};
use lib_rest_core::prelude::*;
use lib_web::middleware::mw_auth::CtxW;

generate_common_rest_fns!(
	Bmc: ConvBmc,
	Entity: Conv,
	ForCreate: ConvForCreate,
	ForUpdate: ConvForUpdate,
	Filter: ConvFilter,
	Suffix: conv,
	ResourceDisplay: "Conversation",
	ResourceGroup: "Conversation Management"
);
