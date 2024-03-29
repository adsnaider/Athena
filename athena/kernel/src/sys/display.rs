//! System display and console.

use bootloader_api::info::{FrameBuffer, PixelFormat};
use critical_section::CriticalSection;
use framed::console::Console;
use framed::{Frame, Pixel};
use singleton::Singleton;

/// The main console for the kernel.
static CONSOLE: Singleton<Console<Display>> = Singleton::uninit();

/// Initializes the console. It's reasonable to use the print! and println! macros after this call.
pub(super) fn init(console: Console<Display>, cs: CriticalSection) {
    CONSOLE.initialize(console, cs);
}

/// The display struct implements the `Frame` trait from the framebuffer pointer.
#[derive(Debug)]
pub struct Display {
    framebuffer: FrameBuffer,
}

impl Display {
    /// Create a new display with the framebuffer.
    ///
    /// # Safety
    ///
    /// * The framebuffer must be correct.
    /// * There should only be one framebuffer (i.e. the memory in the framebuffer is now owned
    /// by the display).
    pub unsafe fn new(framebuffer: FrameBuffer) -> Self {
        Self { framebuffer }
    }
}

// SAFETY: We correctly define the width and height of the display since the framebuffer is correct
// (precondition).
unsafe impl Frame for Display {
    unsafe fn set_pixel_unchecked(&mut self, row: usize, col: usize, pixel: Pixel) {
        match self.framebuffer.info().pixel_format {
            PixelFormat::Rgb => {
                // Each pixel has 4 bytes.
                const PIXEL_SIZE: usize = 4;
                let offset = row * self.framebuffer.info().stride * PIXEL_SIZE + col * PIXEL_SIZE;
                let color: u32 =
                    ((pixel.blue as u32) << 16) + ((pixel.green as u32) << 8) + (pixel.red as u32);
                // SAFETY: The framebuffer structure is correct (precondition).
                unsafe {
                    core::ptr::write_volatile(
                        self.framebuffer.buffer_mut().as_mut_ptr().add(offset) as *mut u32,
                        color,
                    )
                };
            }
            PixelFormat::Bgr => {
                // Each pixel has 4 bytes.
                const PIXEL_SIZE: usize = 4;
                let offset = row * self.framebuffer.info().stride * PIXEL_SIZE + col * PIXEL_SIZE;
                let color: u32 =
                    ((pixel.red as u32) << 16) + ((pixel.green as u32) << 8) + (pixel.blue as u32);
                // SAFETY: The framebuffer structure is correct (precondition).
                unsafe {
                    core::ptr::write_volatile(
                        self.framebuffer.buffer_mut().as_mut_ptr().add(offset) as *mut u32,
                        color,
                    )
                };
            }
            _ => todo!(),
        }
    }

    fn width(&self) -> usize {
        self.framebuffer.info().width
    }

    fn height(&self) -> usize {
        self.framebuffer.info().height
    }
}

/// Prints the arguments to the console. May panic!.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {$crate::sys::_print(format_args!($($arg)*))};
}

/// Prints the arguments to the console and moves to the next line. May panic!.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Prints the arguments to the screen, panicking if unable to.
#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    critical_section::with(|cs| {
        use core::fmt::Write;
        CONSOLE.lock(cs).write_fmt(args).unwrap();
    });
}

/// Prints a debug expression.
#[macro_export]
macro_rules! dbg {
    () => {
        $crate::println!("[{}:{}]", file!(), line!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::println!("[{}:{}] {} = {:#?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}

/// Prints a debug expression.
#[macro_export]
macro_rules! ldbg {
    ($level:expr) => {
        log::log!($level, "[{}:{}]", file!(), line!())
    };
    ($level:expr, $val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::println!("[{}:{}] {} = {:#?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($level:expr, $($val:expr),+ $(,)?) => {
        ($($crate::ldbg!($level, $val)),+,)
    };
}
