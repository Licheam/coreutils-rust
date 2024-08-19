#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]


extern crate libc;
pub mod src {
pub mod lib {
pub mod localcharset;
} // mod lib
} // mod src
extern "C" {
    fn locale_charset() -> *const libc::c_char;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, locale_charset());
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
