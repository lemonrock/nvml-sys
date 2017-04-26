// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.

extern "C"
{
	pub fn pmemobj_list_insert(pop: *mut PMEMobjpool, pe_offset: size_t, head: *mut c_void, dest: PMEMoid, before: c_int, oid: PMEMoid) -> c_int;
	pub fn pmemobj_list_insert_new(pop: *mut PMEMobjpool, pe_offset: size_t, head: *mut c_void, dest: PMEMoid, before: c_int, size: size_t, type_num: uint64_t, constructor: pmemobj_constr, arg: *mut c_void) -> PMEMoid;
	pub fn pmemobj_list_move(pop: *mut PMEMobjpool, pe_old_offset: size_t, head_old: *mut c_void, pe_new_offset: size_t, head_new: *mut c_void, dest: PMEMoid, before: c_int, oid: PMEMoid) -> c_int;
	pub fn pmemobj_list_remove(pop: *mut PMEMobjpool, pe_offset: size_t, head: *mut c_void, oid: PMEMoid, free: c_int) -> c_int;
}
