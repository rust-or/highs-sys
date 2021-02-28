#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_int;

include!(concat!(env!("OUT_DIR"), "/c_bindings.rs"));

pub const MODEL_STATUS_NOTSET: c_int = 0;
pub const MODEL_STATUS_LOAD_ERROR: c_int = 1;
pub const MODEL_STATUS_MODEL_ERROR: c_int = 2;
pub const MODEL_STATUS_PRESOLVE_ERROR: c_int = 3;
pub const MODEL_STATUS_SOLVE_ERROR: c_int = 4;
pub const MODEL_STATUS_POSTSOLVE_ERROR: c_int = 5;
pub const MODEL_STATUS_MODEL_EMPTY: c_int = 6;
pub const MODEL_STATUS_PRIMAL_INFEASIBLE: c_int = 7;
pub const MODEL_STATUS_PRIMAL_UNBOUNDED: c_int = 8;
pub const MODEL_STATUS_OPTIMAL: c_int = 9;
pub const MODEL_STATUS_REACHED_DUAL_OBJECTIVE_VALUE_UPPER_BOUND: c_int = 10;
pub const MODEL_STATUS_REACHED_TIME_LIMIT: c_int = 11;
pub const MODEL_STATUS_REACHED_ITERATION_LIMIT: c_int = 12;
pub const MODEL_STATUS_PRIMAL_DUAL_INFEASIBLE: c_int = 13;
pub const MODEL_STATUS_DUAL_INFEASIBLE: c_int = 14;

pub const STATUS_OK: c_int = 0;
pub const STATUS_WARNING: c_int = 1;
pub const STATUS_ERROR: c_int = 2;
