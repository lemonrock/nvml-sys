// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.

#[repr(C)]
#[derive(Debug, Copy)]
pub struct pmempool_check_statusU__bindgen_ty_1
{
	pub msg: *const c_char,
	pub answer: *const c_char,
}

impl Clone for pmempool_check_statusU__bindgen_ty_1
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for pmempool_check_statusU__bindgen_ty_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Clone for pmempool_check_statusU
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for pmempool_check_statusU
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
