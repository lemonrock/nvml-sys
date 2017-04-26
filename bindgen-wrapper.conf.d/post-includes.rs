extern crate libc;


use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_longlong;
use ::libc::c_uchar;
use ::libc::c_uint;
use ::libc::c_void;
use ::libc::iovec;
use ::libc::mode_t;
use ::libc::size_t;
use ::libc::timespec;
use ::libc::uint32_t;
use ::libc::uint64_t;
use ::libc::wchar_t;

// Horrible hack to get libpmemobj_tx_begin() to compile
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum jmp_buf
{
}
