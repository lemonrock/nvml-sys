// This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.


extern crate libc;


use ::core::mem::zeroed;
use ::core::option::Option;
use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_longlong;
use ::libc::c_uchar;
use ::libc::c_uint;
use ::libc::c_ulong;
use ::libc::c_void;

use ::libc::iovec;
use ::libc::mode_t;
use ::libc::size_t;
use ::libc::timespec;
use ::libc::wchar_t;


#[link(name = "nvml", kind = "static-nobundle")]
extern "C"
{
}

include!("bindgen/constants.rs");
include!("bindgen/enums.rs");
include!("bindgen/functions.rs");
include!("bindgen/statics.rs");
include!("bindgen/structs.rs");
include!("bindgen/types.rs");
include!("bindgen/unions.rs");
include!("bindgen/opaques.rs");
