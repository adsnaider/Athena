pub const PAGE_SIZE: usize = 4096;
pub const FRAME_SIZE: u64 = 4096;

pub mod frames;
pub use frames::RawFrame;

mod physical_address;
pub use physical_address::PhysAddr;
