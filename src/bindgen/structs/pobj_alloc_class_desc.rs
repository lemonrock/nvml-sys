// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.


#[repr(C)]
pub struct pobj_alloc_class_desc
{
	pub unit_size: usize,
	pub units_per_block: c_uint,
	pub header_type: pobj_header_type,
	pub class_id: c_uint,
}

impl Default for pobj_alloc_class_desc
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for pobj_alloc_class_desc
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "pobj_alloc_class_desc {{ header_type: {:?} }}", self.header_type)
	}
}
