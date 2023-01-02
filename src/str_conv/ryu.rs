use crate::f_iof;

#[cfg(features = "faster-strconv")]
#[inline]
pub(crate) unsafe fn _maybe_ryu(f: f_iof) -> String {
    use core::mem::MaybeUninit;
    const RYU_MAXLEN: usize = 24;

    let mut buffer = [MaybeUninit::<u8>::uninit(); RYU_MAXLEN];
    #[cfg(feature = "x64-backing-store")]
    let len = ::ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
    #[cfg(not(feature = "x64-backing-store"))]
    let len = ::ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
    String::from_raw_parts(buffer.as_ptr() as *mut u8, RYU_MAXLEN, RYU_MAXLEN)
}

#[cfg(not(features = "faster-strconv"))]
#[inline]
pub(crate) fn _maybe_ryu(f: f_iof) -> String {
    f.to_string()
}

#[cfg(features = "faster-strconv")]
macro_rules! _unsafe {
    ($($all:block)*) => {
        unsafe {
            $($all)*
        }
    }
}

#[cfg(not(features = "faster-strconv"))]
macro_rules! _unsafe {
    ($($all:block)*) => {
        {
            $($all)*
        }
    }
}

macro_rules! maybe {
    ($f:ident) => {
        _unsafe!( { $crate::str_conv::ryu::_maybe_ryu($f) } )
    }
}

#[cfg(feature = "ryu")]
#[test]
fn test_ryu() {
    let f = core::f64::consts::PI as f_iof;
    let f2 = core::f32::consts::PI as f_iof;
    let mut s = String::new();
    for _ in 0..100000 {
        s.push_str(&maybe!(f));
        s.push('\n');
        s.push_str(&maybe!(f2));
        s.push('\n');
    }
    eprintln!("{}_f64", maybe!(f));
    eprintln!("{}_f32", maybe!(f2));
}
