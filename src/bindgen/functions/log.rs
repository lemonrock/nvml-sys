// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.

extern "C"
{
	pub fn pmemlog_append(plp: *mut PMEMlogpool, buf: *const c_void, count: size_t) -> c_int;
	pub fn pmemlog_appendv(plp: *mut PMEMlogpool, iov: *const iovec, iovcnt: c_int) -> c_int;
	pub fn pmemlog_check(path: *const c_char) -> c_int;
	pub fn pmemlog_check_version(major_required: c_uint, minor_required: c_uint) -> *const c_char;
	pub fn pmemlog_close(plp: *mut PMEMlogpool);
	pub fn pmemlog_create(path: *const c_char, poolsize: size_t, mode: mode_t) -> *mut PMEMlogpool;
	pub fn pmemlog_errormsg() -> *const c_char;
	pub fn pmemlog_nbyte(plp: *mut PMEMlogpool) -> size_t;
	pub fn pmemlog_open(path: *const c_char) -> *mut PMEMlogpool;
	pub fn pmemlog_rewind(plp: *mut PMEMlogpool);
	pub fn pmemlog_set_funcs(malloc_func: Option<extern "C" fn(size: size_t) -> *mut c_void>, free_func: Option<unsafe extern "C" fn(ptr: *mut c_void)>, realloc_func: Option<unsafe extern "C" fn(ptr: *mut c_void, size: size_t) -> *mut c_void>, strdup_func: Option<unsafe extern "C" fn(s: *const c_char) -> *mut c_char>);
	pub fn pmemlog_tell(plp: *mut PMEMlogpool) -> c_longlong;
	pub fn pmemlog_walk(plp: *mut PMEMlogpool, chunksize: size_t, process_chunk: Option<unsafe extern "C" fn(buf: *const c_void, len: size_t, arg: *mut c_void) -> c_int>, arg: *mut c_void);
}
