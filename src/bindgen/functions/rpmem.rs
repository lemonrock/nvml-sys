// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.

extern "C"
{
	pub fn rpmem_check_version(major_required: c_uint, minor_required: c_uint) -> *const c_char;
	pub fn rpmem_close(rpp: *mut RPMEMpool) -> c_int;
	pub fn rpmem_create(target: *const c_char, pool_set_name: *const c_char, pool_addr: *mut c_void, pool_size: usize, nlanes: *mut c_uint, create_attr: *const rpmem_pool_attr) -> *mut RPMEMpool;
	pub fn rpmem_errormsg() -> *const c_char;
	pub fn rpmem_open(target: *const c_char, pool_set_name: *const c_char, pool_addr: *mut c_void, pool_size: usize, nlanes: *mut c_uint, open_attr: *mut rpmem_pool_attr) -> *mut RPMEMpool;
	pub fn rpmem_persist(rpp: *mut RPMEMpool, offset: usize, length: usize, lane: c_uint) -> c_int;
	pub fn rpmem_read(rpp: *mut RPMEMpool, buff: *mut c_void, offset: usize, length: usize, lane: c_uint) -> c_int;
	pub fn rpmem_remove(target: *const c_char, pool_set: *const c_char, flags: c_int) -> c_int;
	pub fn rpmem_set_attr(rpp: *mut RPMEMpool, attr: *const rpmem_pool_attr) -> c_int;
}
