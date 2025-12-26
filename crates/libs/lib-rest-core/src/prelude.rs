//! This is a prelude for all .._rest modules to avoid redundant imports.
//! NOTE: This is only for the `rests` module and sub-modules.
//!
//! IMPORTANT: You must import `CtxW` from `lib_web::middleware::mw_auth::CtxW`
//! in your rest module for the macro to work.

pub use crate::generate_common_rest_fns;
pub use crate::rest_result::{PageInfo, RestCreated, RestDeleted, RestPagedResponse, RestResponse};
pub use crate::{PathId, QueryList};
pub use crate::Result;
pub use axum::extract::{Json, Path, Query, State};
pub use axum::Router;
pub use lib_core::ctx::Ctx;
pub use lib_core::model::ModelManager;
pub use paste::paste;
