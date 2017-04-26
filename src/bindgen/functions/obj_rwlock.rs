// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.

extern "C"
{
	pub fn pmemobj_rwlock_rdlock(pop: *mut PMEMobjpool, rwlockp: *mut PMEMrwlock) -> c_int;
	pub fn pmemobj_rwlock_timedrdlock(pop: *mut PMEMobjpool, rwlockp: *mut PMEMrwlock, abs_timeout: *const timespec) -> c_int;
	pub fn pmemobj_rwlock_timedwrlock(pop: *mut PMEMobjpool, rwlockp: *mut PMEMrwlock, abs_timeout: *const timespec) -> c_int;
	pub fn pmemobj_rwlock_tryrdlock(pop: *mut PMEMobjpool, rwlockp: *mut PMEMrwlock) -> c_int;
	pub fn pmemobj_rwlock_trywrlock(pop: *mut PMEMobjpool, rwlockp: *mut PMEMrwlock) -> c_int;
	pub fn pmemobj_rwlock_unlock(pop: *mut PMEMobjpool, rwlockp: *mut PMEMrwlock) -> c_int;
	pub fn pmemobj_rwlock_wrlock(pop: *mut PMEMobjpool, rwlockp: *mut PMEMrwlock) -> c_int;
	pub fn pmemobj_rwlock_zero(pop: *mut PMEMobjpool, rwlockp: *mut PMEMrwlock);
}
