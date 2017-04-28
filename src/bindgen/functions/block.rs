// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.

extern "C"
{
	pub fn pmemblk_bsize(pbp: *mut PMEMblkpool) -> usize;
	pub fn pmemblk_check(path: *const c_char, bsize: usize) -> c_int;
	pub fn pmemblk_check_version(major_required: c_uint, minor_required: c_uint) -> *const c_char;
	pub fn pmemblk_close(pbp: *mut PMEMblkpool);
	pub fn pmemblk_create(path: *const c_char, bsize: usize, poolsize: usize, mode: mode_t) -> *mut PMEMblkpool;
	pub fn pmemblk_errormsg() -> *const c_char;
	pub fn pmemblk_nblock(pbp: *mut PMEMblkpool) -> usize;
	pub fn pmemblk_open(path: *const c_char, bsize: usize) -> *mut PMEMblkpool;
	pub fn pmemblk_read(pbp: *mut PMEMblkpool, buf: *mut c_void, blockno: c_longlong) -> c_int;
	pub fn pmemblk_set_error(pbp: *mut PMEMblkpool, blockno: c_longlong) -> c_int;
	pub fn pmemblk_set_funcs(malloc_func: Option<unsafe extern "C" fn(size: usize) -> *mut c_void>, free_func: Option<unsafe extern "C" fn(ptr: *mut c_void)>, realloc_func: Option<unsafe extern "C" fn(ptr: *mut c_void, size: usize) -> *mut c_void>, strdup_func: Option<unsafe extern "C" fn(s: *const c_char) -> *mut c_char>);
	pub fn pmemblk_set_zero(pbp: *mut PMEMblkpool, blockno: c_longlong) -> c_int;
	pub fn pmemblk_write(pbp: *mut PMEMblkpool, buf: *const c_void, blockno: c_longlong) -> c_int;
}
