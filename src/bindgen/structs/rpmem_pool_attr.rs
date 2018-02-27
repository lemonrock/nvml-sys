// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.


#[repr(C)]
pub struct rpmem_pool_attr
{
	pub signature: [c_char; 8usize],
	pub major: u32,
	pub compat_features: u32,
	pub incompat_features: u32,
	pub ro_compat_features: u32,
	pub poolset_uuid: [c_uchar; 16usize],
	pub uuid: [c_uchar; 16usize],
	pub next_uuid: [c_uchar; 16usize],
	pub prev_uuid: [c_uchar; 16usize],
	pub user_flags: [c_uchar; 16usize],
}

impl Default for rpmem_pool_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rpmem_pool_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rpmem_pool_attr {{ signature: {:?}, poolset_uuid: {:?}, uuid: {:?}, next_uuid: {:?}, prev_uuid: {:?}, user_flags: {:?} }}", self.signature, self.poolset_uuid, self.uuid, self.next_uuid, self.prev_uuid, self.user_flags)
	}
}
