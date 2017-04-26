// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.

extern "C"
{
	pub fn pmemobj_tx_abort(errnum: c_int);
	pub fn pmemobj_tx_add_range(oid: PMEMoid, off: uint64_t, size: size_t) -> c_int;
	pub fn pmemobj_tx_add_range_direct(ptr: *const c_void, size: size_t) -> c_int;
	pub fn pmemobj_tx_alloc(size: size_t, type_num: uint64_t) -> PMEMoid;
	pub fn pmemobj_tx_begin(pop: *mut PMEMobjpool, env: jmp_buf, ...) -> c_int;
	pub fn pmemobj_tx_commit();
	pub fn pmemobj_tx_end() -> c_int;
	pub fn pmemobj_tx_errno() -> c_int;
	pub fn pmemobj_tx_free(oid: PMEMoid) -> c_int;
	pub fn pmemobj_tx_lock(type_: pobj_tx_param, lockp: *mut c_void) -> c_int;
	pub fn pmemobj_tx_process();
	pub fn pmemobj_tx_realloc(oid: PMEMoid, size: size_t, type_num: uint64_t) -> PMEMoid;
	pub fn pmemobj_tx_stage() -> pobj_tx_stage;
	pub fn pmemobj_tx_strdup(s: *const c_char, type_num: uint64_t) -> PMEMoid;
	pub fn pmemobj_tx_wcsdup(s: *const wchar_t, type_num: uint64_t) -> PMEMoid;
	pub fn pmemobj_tx_xadd_range(oid: PMEMoid, off: uint64_t, size: size_t, flags: uint64_t) -> c_int;
	pub fn pmemobj_tx_xadd_range_direct(ptr: *const c_void, size: size_t, flags: uint64_t) -> c_int;
	pub fn pmemobj_tx_xalloc(size: size_t, type_num: uint64_t, flags: uint64_t) -> PMEMoid;
	pub fn pmemobj_tx_zalloc(size: size_t, type_num: uint64_t) -> PMEMoid;
	pub fn pmemobj_tx_zrealloc(oid: PMEMoid, size: size_t, type_num: uint64_t) -> PMEMoid;
}
