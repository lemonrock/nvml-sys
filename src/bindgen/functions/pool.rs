// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.

extern "C"
{
	pub fn pmempool_check(ppc: *mut PMEMpoolcheck) -> *mut pmempool_check_statusU;
	pub fn pmempool_check_end(ppc: *mut PMEMpoolcheck) -> pmempool_check_result;
	pub fn pmempool_check_init(args: *mut pmempool_check_argsU, args_size: usize) -> *mut PMEMpoolcheck;
	pub fn pmempool_check_version(major_required: c_uint, minor_required: c_uint) -> *const c_char;
	pub fn pmempool_errormsg() -> *const c_char;
	pub fn pmempool_rm(path: *const c_char, flags: c_int) -> c_int;
	pub fn pmempool_sync(poolset_file: *const c_char, flags: c_uint) -> c_int;
	pub fn pmempool_transform(poolset_file_src: *const c_char, poolset_file_dst: *const c_char, flags: c_uint) -> c_int;
}
