use ::libc::iovec;
use ::libc::mode_t;
use ::libc::timespec;
use ::libc::wchar_t;

// Horrible hack to get libpmemobj_tx_begin() to compile
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum jmp_buf
{
}
