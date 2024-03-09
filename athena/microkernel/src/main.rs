#![no_std]
#![no_main]
#![cfg_attr(
    test,
    feature(custom_test_frameworks),
    test_runner(crate::tests::runner),
    reexport_test_harness_main = "test_main"
)]
#![feature(naked_functions)]
#![cfg_attr(target_arch = "x86_64", feature(abi_x86_interrupt))]

pub mod arch;
pub mod bootstrap;

mod serial;
#[cfg(test)]
mod tests;

use limine::request::MemoryMapRequest;
use limine::BaseRevision;

use crate::bootstrap::FrameBumpAllocator;

/// Sets the base revision to the latest revision supported by the crate.
/// See specification for further info.
#[used]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[cfg(target_os = "none")]
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // FIXME: Reboot?
    log::error!("{}", info);
    loop {
        arch::instructions::hlt();
    }
}

fn init() {
    serial::init();
    log::info!("Serial logging initialized");

    assert!(BASE_REVISION.is_supported());
    arch::init();

    #[used]
    static mut MEMORY_MAP: MemoryMapRequest = MemoryMapRequest::new();

    let memory_map = unsafe {
        MEMORY_MAP
            .get_response_mut()
            .expect("Missing memory map from Limine")
            .entries_mut()
    };
    log::info!("Got memory map");
    let mut allocator = FrameBumpAllocator::new(memory_map);

    log::info!("Initialization sequence complete");
}

#[cfg(not(test))]
#[no_mangle]
unsafe extern "C" fn kmain() -> ! {
    init();
    loop {
        arch::instructions::hlt();
    }
}

struct SingleThreadCS();
critical_section::set_impl!(SingleThreadCS);
/// SAFETY: While the OS kernel is running in a single thread, then disabling interrupts is a safe
/// to guarantee a critical section's conditions.
unsafe impl critical_section::Impl for SingleThreadCS {
    unsafe fn acquire() -> critical_section::RawRestoreState {
        let interrupts_enabled = arch::interrupts::are_enabled();
        arch::interrupts::disable();
        interrupts_enabled
    }

    unsafe fn release(interrupts_were_enabled: critical_section::RawRestoreState) {
        if interrupts_were_enabled {
            // SAFETY: Precondition.
            unsafe {
                arch::interrupts::enable();
            }
        }
    }
}
