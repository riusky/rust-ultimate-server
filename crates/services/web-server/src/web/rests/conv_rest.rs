use lib_core::model::conv::{
	Conv, ConvBmc, ConvFilter, ConvForCreate, ConvForUpdate,
};
use lib_rest_core::prelude::*;

generate_common_rest_fns!(
	Bmc: ConvBmc,
	Entity: Conv,
	ForCreate: ConvForCreate,
	ForUpdate: ConvForUpdate,
	Filter: ConvFilter,
	Suffix: conv
);
