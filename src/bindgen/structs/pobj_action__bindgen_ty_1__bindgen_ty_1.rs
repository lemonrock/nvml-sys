// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.


#[repr(C)]
pub struct pobj_action__bindgen_ty_1__bindgen_ty_1
{
	pub offset: u64,
}

impl Default for pobj_action__bindgen_ty_1__bindgen_ty_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for pobj_action__bindgen_ty_1__bindgen_ty_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "pobj_action__bindgen_ty_1__bindgen_ty_1 {{  }}")
	}
}

impl Default for pobj_action__bindgen_ty_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for pobj_action__bindgen_ty_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "pobj_action__bindgen_ty_1 {{ union }}")
	}
}

impl Default for pobj_action
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for pobj_action
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "pobj_action {{ type: {:?}, data: {:?}, __bindgen_anon_1: {:?} }}", self.type_, self.data, self.__bindgen_anon_1)
	}
}
