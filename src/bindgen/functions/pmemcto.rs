// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.


extern "C"
{
	pub fn pmemcto_aligned_alloc(pcp: *mut PMEMctopool, alignment: usize, size: usize) -> *mut c_void;
	pub fn pmemcto_calloc(pcp: *mut PMEMctopool, nmemb: usize, size: usize) -> *mut c_void;
	pub fn pmemcto_check(path: *const c_char, layout: *const c_char) -> c_int;
	pub fn pmemcto_check_version(major_required: c_uint, minor_required: c_uint) -> *const c_char;
	pub fn pmemcto_close(pcp: *mut PMEMctopool);
	pub fn pmemcto_create(path: *const c_char, layout: *const c_char, poolsize: usize, mode: mode_t) -> *mut PMEMctopool;
	pub fn pmemcto_errormsg() -> *const c_char;
	pub fn pmemcto_free(pcp: *mut PMEMctopool, ptr: *mut c_void);
	pub fn pmemcto_get_root_pointer(pcp: *mut PMEMctopool) -> *mut c_void;
	pub fn pmemcto_malloc(pcp: *mut PMEMctopool, size: usize) -> *mut c_void;
	pub fn pmemcto_malloc_usable_size(pcp: *mut PMEMctopool, ptr: *mut c_void) -> usize;
	pub fn pmemcto_open(path: *const c_char, layout: *const c_char) -> *mut PMEMctopool;
	pub fn pmemcto_realloc(pcp: *mut PMEMctopool, ptr: *mut c_void, size: usize) -> *mut c_void;
	pub fn pmemcto_set_funcs(malloc_func: Option<unsafe extern "C" fn(size: usize) -> *mut c_void>, free_func: Option<unsafe extern "C" fn(ptr: *mut c_void)>, realloc_func: Option<unsafe extern "C" fn(ptr: *mut c_void, size: usize) -> *mut c_void>, strdup_func: Option<unsafe extern "C" fn(s: *const c_char) -> *mut c_char>, print_func: Option<unsafe extern "C" fn(s: *const c_char)>);
	pub fn pmemcto_set_root_pointer(pcp: *mut PMEMctopool, ptr: *mut c_void);
	pub fn pmemcto_stats_print(pcp: *mut PMEMctopool, opts: *const c_char);
	pub fn pmemcto_strdup(pcp: *mut PMEMctopool, s: *const c_char) -> *mut c_char;
	pub fn pmemcto_wcsdup(pcp: *mut PMEMctopool, s: *const wchar_t) -> *mut wchar_t;
}
