// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.


#[repr(C)]
pub struct __jmp_buf_tag
{
	pub __jb: __jmp_buf,
	pub __fl: c_ulong,
	pub __ss: [c_ulong; 16usize],
}

impl Default for __jmp_buf_tag
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for __jmp_buf_tag
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "__jmp_buf_tag {{  }}")
	}
}
