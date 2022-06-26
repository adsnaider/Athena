//! Memory allocation and paging utilities.
#![cfg_attr(not(test), no_std)]
#![feature(allocator_api)]
#![feature(result_option_inspect)]
#![feature(nonnull_slice_from_raw_parts)]
#![deny(absolute_paths_not_starting_with_crate)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(unsafe_op_in_unsafe_fn)]

pub mod allocation;
pub mod system;
pub mod structures;

#[cfg(test)]
pub(crate) mod test_utils {
    pub fn init_logging() {
        let _ = env_logger::builder().is_test(true).try_init();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::test_utils::init_logging();
        log::info!("Hello world!");
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}