//! Arch-dependent portions of the kernel.

#[cfg(target_arch = "x86_64")]
mod x64;
#[cfg(target_arch = "x86_64")]
pub use x64::*;

use self::mm::frames::RawFrame;

pub const PAGE_SIZE: usize = RawFrame::size();
