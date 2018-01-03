// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.


pub const POBJ_XALLOC_ZERO: u64 = POBJ_FLAG_ZERO;
pub const POBJ_XALLOC_NO_FLUSH: u64 = POBJ_FLAG_NO_FLUSH;
pub const POBJ_XALLOC_VALID_FLAGS: u64 = (POBJ_XALLOC_ZERO | POBJ_XALLOC_NO_FLUSH);
