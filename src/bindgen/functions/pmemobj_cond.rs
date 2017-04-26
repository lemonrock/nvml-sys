// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.

extern "C"
{
	pub fn pmemobj_cond_broadcast(pop: *mut PMEMobjpool, condp: *mut PMEMcond) -> c_int;
	pub fn pmemobj_cond_signal(pop: *mut PMEMobjpool, condp: *mut PMEMcond) -> c_int;
	pub fn pmemobj_cond_timedwait(pop: *mut PMEMobjpool, condp: *mut PMEMcond, mutexp: *mut PMEMmutex, abs_timeout: *const timespec) -> c_int;
	pub fn pmemobj_cond_wait(pop: *mut PMEMobjpool, condp: *mut PMEMcond, mutexp: *mut PMEMmutex) -> c_int;
	pub fn pmemobj_cond_zero(pop: *mut PMEMobjpool, condp: *mut PMEMcond);
}
