// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.

#[repr(C)]
#[derive(Copy)]
#[allow(missing_debug_implementations)]
pub struct PMEMcond
{
	pub _bindgen_data_: [u64; 8usize],
}

impl PMEMcond
{
	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn align(&mut self) -> *mut c_longlong
	{
		let raw = &mut self._bindgen_data_ as *mut _ as *mut u8;
		raw as *mut _
	}

	#[inline(always)]
	#[allow(trivial_casts)]
	pub fn padding(&mut self) -> *mut [c_char; 64usize]
	{
		let raw = &mut self._bindgen_data_ as *mut _ as *mut u8;
		raw as *mut _
	}
}

impl Clone for PMEMcond
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for PMEMcond
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
