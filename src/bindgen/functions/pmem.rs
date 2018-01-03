// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.


extern "C"
{
	pub fn pmem_check_version(major_required: c_uint, minor_required: c_uint) -> *const c_char;
	pub fn pmem_drain();
	pub fn pmem_errormsg() -> *const c_char;
	pub fn pmem_flush(addr: *const c_void, len: usize);
	pub fn pmem_has_hw_drain() -> c_int;
	pub fn pmem_is_pmem(addr: *const c_void, len: usize) -> c_int;
	pub fn pmem_map_file(path: *const c_char, len: usize, flags: c_int, mode: mode_t, mapped_lenp: *mut usize, is_pmemp: *mut c_int) -> *mut c_void;
	pub fn pmem_msync(addr: *const c_void, len: usize) -> c_int;
	pub fn pmem_persist(addr: *const c_void, len: usize);
	pub fn pmem_unmap(addr: *mut c_void, len: usize) -> c_int;
}
