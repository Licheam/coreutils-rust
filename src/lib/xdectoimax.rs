extern "C" {
    fn xstrtoimax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut intmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn __errno_location() -> *mut libc::c_int;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
}
pub type __intmax_t = libc::c_long;
pub type intmax_t = __intmax_t;
pub const LONGINT_OK: strtol_error = 0;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
#[no_mangle]
pub unsafe extern "C" fn xnumtoimax(
    mut n_str: *const libc::c_char,
    mut base: libc::c_int,
    mut min: intmax_t,
    mut max: intmax_t,
    mut suffixes: *const libc::c_char,
    mut err: *const libc::c_char,
    mut err_exit: libc::c_int,
) -> intmax_t {
    let mut s_err: strtol_error = LONGINT_OK;
    let mut tnum: intmax_t = 0;
    s_err = xstrtoimax(n_str, 0 as *mut *mut libc::c_char, base, &mut tnum, suffixes);
    if s_err as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint {
        if tnum < min || max < tnum {
            s_err = LONGINT_OVERFLOW;
            if tnum > (2147483647 as libc::c_int / 2 as libc::c_int) as libc::c_long {
                *__errno_location() = 75 as libc::c_int;
            } else if tnum
                < ((-(2147483647 as libc::c_int) - 1 as libc::c_int) / 2 as libc::c_int)
                    as libc::c_long
            {
                *__errno_location() = 75 as libc::c_int;
            } else {
                *__errno_location() = 34 as libc::c_int;
            }
        }
    } else if s_err as libc::c_uint == LONGINT_OVERFLOW as libc::c_int as libc::c_uint {
        *__errno_location() = 75 as libc::c_int;
    } else if s_err as libc::c_uint
        == LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW as libc::c_int as libc::c_uint
    {
        *__errno_location() = 0 as libc::c_int;
    }
    if s_err as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint {
        error(
            if err_exit != 0 { err_exit } else { 1 as libc::c_int },
            if *__errno_location() == 22 as libc::c_int {
                0 as libc::c_int
            } else {
                *__errno_location()
            },
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            err,
            quote(n_str),
        );
    }
    return tnum;
}
#[no_mangle]
pub unsafe extern "C" fn xdectoimax(
    mut n_str: *const libc::c_char,
    mut min: intmax_t,
    mut max: intmax_t,
    mut suffixes: *const libc::c_char,
    mut err: *const libc::c_char,
    mut err_exit: libc::c_int,
) -> intmax_t {
    return xnumtoimax(n_str, 10 as libc::c_int, min, max, suffixes, err, err_exit);
}
