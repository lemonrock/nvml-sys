// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.

extern "C"
{
	pub fn _pobj_debug_notice(func_name: *const c_char, file: *const c_char, line: c_int);
	pub fn pmemobj_alloc(pop: *mut PMEMobjpool, oidp: *mut PMEMoid, size: size_t, type_num: uint64_t, constructor: pmemobj_constr, arg: *mut c_void) -> c_int;
	pub fn pmemobj_alloc_usable_size(oid: PMEMoid) -> size_t;
	pub fn pmemobj_check(path: *const c_char, layout: *const c_char) -> c_int;
	pub fn pmemobj_check_version(major_required: c_uint, minor_required: c_uint) -> *const c_char;
	pub fn pmemobj_close(pop: *mut PMEMobjpool);
	pub fn pmemobj_create(path: *const c_char, layout: *const c_char, poolsize: size_t, mode: mode_t) -> *mut PMEMobjpool;
	pub fn pmemobj_drain(pop: *mut PMEMobjpool);
	pub fn pmemobj_errormsg() -> *const c_char;
	pub fn pmemobj_first(pop: *mut PMEMobjpool) -> PMEMoid;
	pub fn pmemobj_flush(pop: *mut PMEMobjpool, addr: *const c_void, len: size_t);
	pub fn pmemobj_free(oidp: *mut PMEMoid);
	pub fn pmemobj_memcpy_persist(pop: *mut PMEMobjpool, dest: *mut c_void, src: *const c_void, len: size_t) -> *mut c_void;
	pub fn pmemobj_memset_persist(pop: *mut PMEMobjpool, dest: *mut c_void, c: c_int, len: size_t) -> *mut c_void;
	pub fn pmemobj_next(oid: PMEMoid) -> PMEMoid;
	pub fn pmemobj_oid(addr: *const c_void) -> PMEMoid;
	pub fn pmemobj_open(path: *const c_char, layout: *const c_char) -> *mut PMEMobjpool;
	pub fn pmemobj_persist(pop: *mut PMEMobjpool, addr: *const c_void, len: size_t);
	pub fn pmemobj_realloc(pop: *mut PMEMobjpool, oidp: *mut PMEMoid, size: size_t, type_num: uint64_t) -> c_int;
	pub fn pmemobj_root(pop: *mut PMEMobjpool, size: size_t) -> PMEMoid;
	pub fn pmemobj_set_funcs(malloc_func: Option<extern "C" fn(size: size_t) -> *mut c_void>, free_func: Option<unsafe extern "C" fn(ptr: *mut c_void)>, realloc_func: Option<unsafe extern "C" fn(ptr: *mut c_void, size: size_t) -> *mut c_void>, strdup_func: Option<unsafe extern "C" fn(s: *const c_char) -> *mut c_char>);
	pub fn pmemobj_strdup(pop: *mut PMEMobjpool, oidp: *mut PMEMoid, s: *const c_char, type_num: uint64_t) -> c_int;
	pub fn pmemobj_type_num(oid: PMEMoid) -> uint64_t;
	pub fn pmemobj_wcsdup(pop: *mut PMEMobjpool, oidp: *mut PMEMoid, s: *const wchar_t, type_num: uint64_t) -> c_int;
	pub fn pmemobj_zalloc(pop: *mut PMEMobjpool, oidp: *mut PMEMoid, size: size_t, type_num: uint64_t) -> c_int;
	pub fn pmemobj_zrealloc(pop: *mut PMEMobjpool, oidp: *mut PMEMoid, size: size_t, type_num: uint64_t) -> c_int;
}
