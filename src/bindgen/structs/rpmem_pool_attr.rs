// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct rpmem_pool_attr
{
	pub signature: [c_char; 8usize],
	pub major: uint32_t,
	pub compat_features: uint32_t,
	pub incompat_features: uint32_t,
	pub ro_compat_features: uint32_t,
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
