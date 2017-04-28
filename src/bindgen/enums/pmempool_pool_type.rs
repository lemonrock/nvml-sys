// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum pmempool_pool_type
{
	PMEMPOOL_POOL_TYPE_DETECT = 0,
	PMEMPOOL_POOL_TYPE_LOG = 1,
	PMEMPOOL_POOL_TYPE_BLK = 2,
	PMEMPOOL_POOL_TYPE_OBJ = 3,
	PMEMPOOL_POOL_TYPE_BTT = 4,
}
