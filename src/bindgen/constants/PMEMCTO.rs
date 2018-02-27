// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.


pub const PMEMCTO_MAJOR_VERSION: u32 = 1;
pub const PMEMCTO_MINOR_VERSION: u32 = 0;

// Accurate as of Jan 4th 2017
pub const PMEMCTO_MIN_POOL: size_t = 1024 * 1024 * 16;

// Accurate as of Jan 4th 2017
pub const PMEMCTO_MIN_PART: size_t = 1024 * 1024 * 2;

// Accurate as of Jan 4th 2017
pub const PMEMCTO_MAX_LAYOUT: size_t = 1024;
