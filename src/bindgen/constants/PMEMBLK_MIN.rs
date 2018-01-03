// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.


// Accurate as of May 7th 2017
pub const PMEMBLK_MIN_BLK: size_t = 512;

// Accurate as of May 7th 2017
// 16MB + 4KB (minimum BTT size + mmap alignment)
#[cfg(unix)] pub const PMEMBLK_MIN_POOL: size_t = (1 << 20) * 16 + (1 << 10) * 8;

// Accurate as of May 7th 2017
// 16MB + 64KB (minimum BTT size + mmap alignment)
#[cfg(windows)] pub const PMEMBLK_MIN_POOL: size_t = (1 << 20) * 16 + (1 << 10) * 64;
