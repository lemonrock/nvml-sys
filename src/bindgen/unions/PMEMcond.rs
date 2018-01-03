// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub union PMEMcond
{
    pub align: c_longlong,
    pub padding: [c_char; 64usize],
    _bindgen_union_align: [u64; 8usize],
}

impl Clone for PMEMcond
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for PMEMcond
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
