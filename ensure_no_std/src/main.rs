#![feature(const_extern_fn, default_alloc_error_handler)]
#![no_std]
#![no_main]

#[macro_use]
extern crate libc_print;
use num_format::{Buffer, Locale};

#[link(name = "c")]
extern "C" {
    fn exit(errno: i8) -> !;
    fn putc(_: u8, _: *const core::ffi::c_void);
    static stdout: *const core::ffi::c_void;
}

use integer_or_float::*;

#[allow(unused_imports)]
use integer_or_float;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    exit(1);
}

#[no_mangle]
unsafe fn _start() -> ! {
    main();
    exit(0);
}

#[inline(never)]
pub const extern "C" fn it_works() -> *const core::ffi::c_void {
    b"It works!\0" as *const _ as *const core::ffi::c_void
}

#[no_mangle]
fn main() {
    let mut buf = Buffer::default();
    buf.write_formatted(&Into::<usize>::into(Integer(1000000)), &Locale::en);
    let mut _buf2 = ryu::Buffer::default();
    let buf2 = _buf2.format(Into::<f64>::into(Float(f32::INFINITY)));
    unsafe {
        for c in b"It works!"
            .into_iter()
            .map(|b| *b)
            .chain(['\n' as u8])
            .chain(buf.bytes())
            .chain([0x20])
            .chain(buf2.bytes())
        {
            putc(c, stdout);
        }
    }
}
