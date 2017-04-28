// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum pobj_tx_stage
{
	TX_STAGE_NONE = 0,
	TX_STAGE_WORK = 1,
	TX_STAGE_ONCOMMIT = 2,
	TX_STAGE_ONABORT = 3,
	TX_STAGE_FINALLY = 4,
	MAX_TX_STAGE = 5,
}
