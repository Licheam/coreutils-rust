#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]


extern crate f128;
extern crate libc;
pub mod src {
pub mod lib {
pub mod c_strcasecmp;
pub mod close_stream;
pub mod closeout;
pub mod dtoastr;
pub mod exitfail;
pub mod fclose;
pub mod fflush;
pub mod fseeko;
pub mod ftoastr;
pub mod hard_locale;
pub mod ldtoastr;
pub mod localcharset;
pub mod long_options;
pub mod mbrtowc;
pub mod progname;
pub mod quotearg;
pub mod setlocale_null;
pub mod version_etc;
pub mod version_etc_fsf;
pub mod xalloc_die;
pub mod xmalloc;
} // mod lib
} // mod src
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn ldtoastr(
        buf: *mut libc::c_char,
        bufsize: size_t,
        flags: libc::c_int,
        width: libc::c_int,
        x: f128::f128,
    ) -> libc::c_int;
    fn ftoastr(
        buf: *mut libc::c_char,
        bufsize: size_t,
        flags: libc::c_int,
        width: libc::c_int,
        x: libc::c_float,
    ) -> libc::c_int;
    fn dtoastr(
        buf: *mut libc::c_char,
        bufsize: size_t,
        flags: libc::c_int,
        width: libc::c_int,
        x: libc::c_double,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut exit_failure: libc::c_int;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn close_stdout();
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn exit(_: libc::c_int) -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn parse_long_options(
        _argc: libc::c_int,
        _argv: *mut *mut libc::c_char,
        _command_name: *const libc::c_char,
        _package: *const libc::c_char,
        _version: *const libc::c_char,
        _usage: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        _: ...
    );
}
pub type size_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
pub type C2RustUnnamed = libc::c_uint;
pub const FTOASTR_UPPER_E: C2RustUnnamed = 16;
pub const FTOASTR_ZERO_PAD: C2RustUnnamed = 8;
pub const FTOASTR_SPACE_POSITIVE: C2RustUnnamed = 4;
pub const FTOASTR_ALWAYS_SIGNED: C2RustUnnamed = 2;
pub const FTOASTR_LEFT_JUSTIFY: C2RustUnnamed = 1;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct infomap {
    pub program: *const libc::c_char,
    pub node: *const libc::c_char,
}
#[no_mangle]
pub unsafe extern "C" fn usage(mut status: libc::c_int) -> ! {
    if status != 0 as libc::c_int {
        fprintf(
            stderr,
            gettext(
                b"Try '%s --help' for more information.\n\0" as *const u8
                    as *const libc::c_char,
            ),
            program_name,
        );
    } else {
        printf(
            gettext(b"Usage: %s\n\0" as *const u8 as *const libc::c_char),
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"Output platform dependent limits in a format useful for shell scripts.\n\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"      --help     display this help and exit\n\0" as *const u8
                    as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"      --version  output version information and exit\n\0" as *const u8
                    as *const libc::c_char,
            ),
            stdout,
        );
        emit_ancillary_info(b"getlimits\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
#[inline]
unsafe extern "C" fn emit_ancillary_info(mut program: *const libc::c_char) {
    let infomap_0: [infomap; 7] = [
        {
            let mut init = infomap {
                program: b"[\0" as *const u8 as *const libc::c_char,
                node: b"test invocation\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: b"coreutils\0" as *const u8 as *const libc::c_char,
                node: b"Multi-call invocation\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: b"sha224sum\0" as *const u8 as *const libc::c_char,
                node: b"sha2 utilities\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: b"sha256sum\0" as *const u8 as *const libc::c_char,
                node: b"sha2 utilities\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: b"sha384sum\0" as *const u8 as *const libc::c_char,
                node: b"sha2 utilities\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: b"sha512sum\0" as *const u8 as *const libc::c_char,
                node: b"sha2 utilities\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: 0 as *const libc::c_char,
                node: 0 as *const libc::c_char,
            };
            init
        },
    ];
    let mut node: *const libc::c_char = program;
    let mut map_prog: *const infomap = infomap_0.as_ptr();
    while !((*map_prog).program).is_null()
        && !(strcmp(program, (*map_prog).program) == 0 as libc::c_int)
    {
        map_prog = map_prog.offset(1);
        map_prog;
    }
    if !((*map_prog).node).is_null() {
        node = (*map_prog).node;
    }
    printf(
        gettext(b"\n%s online help: <%s>\n\0" as *const u8 as *const libc::c_char),
        b"GNU coreutils\0" as *const u8 as *const libc::c_char,
        b"https://www.gnu.org/software/coreutils/\0" as *const u8 as *const libc::c_char,
    );
    let mut lc_messages: *const libc::c_char = setlocale(
        5 as libc::c_int,
        0 as *const libc::c_char,
    );
    if !lc_messages.is_null()
        && strncmp(
            lc_messages,
            b"en_\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) != 0
    {
        fputs_unlocked(
            gettext(
                b"Report any translation bugs to <https://translationproject.org/team/>\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
    }
    printf(
        gettext(b"Full documentation <%s%s>\n\0" as *const u8 as *const libc::c_char),
        b"https://www.gnu.org/software/coreutils/\0" as *const u8 as *const libc::c_char,
        program,
    );
    printf(
        gettext(
            b"or available locally via: info '(coreutils) %s%s'\n\0" as *const u8
                as *const libc::c_char,
        ),
        node,
        if node == program {
            b" invocation\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
}
#[inline]
unsafe extern "C" fn initialize_exit_failure(mut status: libc::c_int) {
    if status != 1 as libc::c_int {
        ::core::ptr::write_volatile(&mut exit_failure as *mut libc::c_int, status);
    }
}
unsafe extern "C" fn decimal_absval_add_one(
    mut buf: *mut libc::c_char,
) -> *const libc::c_char {
    let mut negative: bool = *buf.offset(1 as libc::c_int as isize) as libc::c_int
        == '-' as i32;
    let mut absnum: *mut libc::c_char = buf
        .offset(1 as libc::c_int as isize)
        .offset(negative as libc::c_int as isize);
    let mut p: *mut libc::c_char = absnum.offset(strlen(absnum) as isize);
    *absnum.offset(-(1 as libc::c_int) as isize) = '0' as i32 as libc::c_char;
    loop {
        p = p.offset(-1);
        if !(*p as libc::c_int == '9' as i32) {
            break;
        }
        *p = '0' as i32 as libc::c_char;
    }
    *p += 1;
    *p;
    let mut result: *mut libc::c_char = if absnum < p { absnum } else { p };
    if negative {
        result = result.offset(-1);
        *result = '-' as i32 as libc::c_char;
    }
    return result;
}
unsafe extern "C" fn print_FLT(mut x: libc::c_float) {
    let mut buf: [libc::c_char; 31] = [0; 31];
    ftoastr(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong,
        FTOASTR_LEFT_JUSTIFY as libc::c_int,
        0 as libc::c_int,
        x,
    );
    puts(buf.as_mut_ptr());
}
unsafe extern "C" fn print_DBL(mut x: libc::c_double) {
    let mut buf: [libc::c_char; 40] = [0; 40];
    dtoastr(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
        FTOASTR_LEFT_JUSTIFY as libc::c_int,
        0 as libc::c_int,
        x,
    );
    puts(buf.as_mut_ptr());
}
unsafe extern "C" fn print_LDBL(mut x: f128::f128) {
    let mut buf: [libc::c_char; 45] = [0; 45];
    ldtoastr(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 45]>() as libc::c_ulong,
        FTOASTR_LEFT_JUSTIFY as libc::c_int,
        0 as libc::c_int,
        x,
    );
    puts(buf.as_mut_ptr());
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut limit: [libc::c_char; 22] = [0; 22];
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    initialize_exit_failure(1 as libc::c_int);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    parse_long_options(
        argc,
        argv,
        b"getlimits\0" as *const u8 as *const libc::c_char,
        b"GNU coreutils\0" as *const u8 as *const libc::c_char,
        b"9.0\0" as *const u8 as *const libc::c_char,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> !>,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(Some(usage as unsafe extern "C" fn(libc::c_int) -> !)),
        b"Padraig Brady\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    sprintf(
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        b"%lu\0" as *const u8 as *const libc::c_char,
        127 as libc::c_int as uintmax_t,
    );
    printf(
        b"CHAR_MAX=%s\n\0" as *const u8 as *const libc::c_char,
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    printf(
        b"CHAR_OFLOW=%s\n\0" as *const u8 as *const libc::c_char,
        decimal_absval_add_one(limit.as_mut_ptr()),
    );
    if -(127 as libc::c_int) - 1 as libc::c_int != 0 {
        sprintf(
            limit.as_mut_ptr().offset(1 as libc::c_int as isize),
            b"%ld\0" as *const u8 as *const libc::c_char,
            (-(127 as libc::c_int) - 1 as libc::c_int) as intmax_t,
        );
        printf(
            b"CHAR_MIN=%s\n\0" as *const u8 as *const libc::c_char,
            limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        printf(
            b"CHAR_UFLOW=%s\n\0" as *const u8 as *const libc::c_char,
            decimal_absval_add_one(limit.as_mut_ptr()),
        );
    }
    sprintf(
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        b"%lu\0" as *const u8 as *const libc::c_char,
        127 as libc::c_int as uintmax_t,
    );
    printf(
        b"SCHAR_MAX=%s\n\0" as *const u8 as *const libc::c_char,
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    printf(
        b"SCHAR_OFLOW=%s\n\0" as *const u8 as *const libc::c_char,
        decimal_absval_add_one(limit.as_mut_ptr()),
    );
    if -(127 as libc::c_int) - 1 as libc::c_int != 0 {
        sprintf(
            limit.as_mut_ptr().offset(1 as libc::c_int as isize),
            b"%ld\0" as *const u8 as *const libc::c_char,
            (-(127 as libc::c_int) - 1 as libc::c_int) as intmax_t,
        );
        printf(
            b"SCHAR_MIN=%s\n\0" as *const u8 as *const libc::c_char,
            limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        printf(
            b"SCHAR_UFLOW=%s\n\0" as *const u8 as *const libc::c_char,
            decimal_absval_add_one(limit.as_mut_ptr()),
        );
    }
    sprintf(
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        b"%lu\0" as *const u8 as *const libc::c_char,
        (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as uintmax_t,
    );
    printf(
        b"UCHAR_MAX=%s\n\0" as *const u8 as *const libc::c_char,
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    printf(
        b"UCHAR_OFLOW=%s\n\0" as *const u8 as *const libc::c_char,
        decimal_absval_add_one(limit.as_mut_ptr()),
    );
    sprintf(
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        b"%lu\0" as *const u8 as *const libc::c_char,
        32767 as libc::c_int as uintmax_t,
    );
    printf(
        b"SHRT_MAX=%s\n\0" as *const u8 as *const libc::c_char,
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    printf(
        b"SHRT_OFLOW=%s\n\0" as *const u8 as *const libc::c_char,
        decimal_absval_add_one(limit.as_mut_ptr()),
    );
    if -(32767 as libc::c_int) - 1 as libc::c_int != 0 {
        sprintf(
            limit.as_mut_ptr().offset(1 as libc::c_int as isize),
            b"%ld\0" as *const u8 as *const libc::c_char,
            (-(32767 as libc::c_int) - 1 as libc::c_int) as intmax_t,
        );
        printf(
            b"SHRT_MIN=%s\n\0" as *const u8 as *const libc::c_char,
            limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        printf(
            b"SHRT_UFLOW=%s\n\0" as *const u8 as *const libc::c_char,
            decimal_absval_add_one(limit.as_mut_ptr()),
        );
    }
    sprintf(
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        b"%lu\0" as *const u8 as *const libc::c_char,
        2147483647 as libc::c_int as uintmax_t,
    );
    printf(
        b"INT_MAX=%s\n\0" as *const u8 as *const libc::c_char,
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    printf(
        b"INT_OFLOW=%s\n\0" as *const u8 as *const libc::c_char,
        decimal_absval_add_one(limit.as_mut_ptr()),
    );
    if -(2147483647 as libc::c_int) - 1 as libc::c_int != 0 {
        sprintf(
            limit.as_mut_ptr().offset(1 as libc::c_int as isize),
            b"%ld\0" as *const u8 as *const libc::c_char,
            (-(2147483647 as libc::c_int) - 1 as libc::c_int) as intmax_t,
        );
        printf(
            b"INT_MIN=%s\n\0" as *const u8 as *const libc::c_char,
            limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        printf(
            b"INT_UFLOW=%s\n\0" as *const u8 as *const libc::c_char,
            decimal_absval_add_one(limit.as_mut_ptr()),
        );
    }
    sprintf(
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        b"%lu\0" as *const u8 as *const libc::c_char,
        (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as uintmax_t,
    );
    printf(
        b"UINT_MAX=%s\n\0" as *const u8 as *const libc::c_char,
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    printf(
        b"UINT_OFLOW=%s\n\0" as *const u8 as *const libc::c_char,
        decimal_absval_add_one(limit.as_mut_ptr()),
    );
    sprintf(
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        b"%lu\0" as *const u8 as *const libc::c_char,
        9223372036854775807 as libc::c_long as uintmax_t,
    );
    printf(
        b"LONG_MAX=%s\n\0" as *const u8 as *const libc::c_char,
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    printf(
        b"LONG_OFLOW=%s\n\0" as *const u8 as *const libc::c_char,
        decimal_absval_add_one(limit.as_mut_ptr()),
    );
    if -(9223372036854775807 as libc::c_long) - 1 as libc::c_long != 0 {
        sprintf(
            limit.as_mut_ptr().offset(1 as libc::c_int as isize),
            b"%ld\0" as *const u8 as *const libc::c_char,
            -(9223372036854775807 as libc::c_long) - 1 as libc::c_long,
        );
        printf(
            b"LONG_MIN=%s\n\0" as *const u8 as *const libc::c_char,
            limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        printf(
            b"LONG_UFLOW=%s\n\0" as *const u8 as *const libc::c_char,
            decimal_absval_add_one(limit.as_mut_ptr()),
        );
    }
    sprintf(
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        b"%lu\0" as *const u8 as *const libc::c_char,
        (9223372036854775807 as libc::c_long as libc::c_ulong)
            .wrapping_mul(2 as libc::c_ulong)
            .wrapping_add(1 as libc::c_ulong),
    );
    printf(
        b"ULONG_MAX=%s\n\0" as *const u8 as *const libc::c_char,
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    printf(
        b"ULONG_OFLOW=%s\n\0" as *const u8 as *const libc::c_char,
        decimal_absval_add_one(limit.as_mut_ptr()),
    );
    sprintf(
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        b"%lu\0" as *const u8 as *const libc::c_char,
        18446744073709551615 as libc::c_ulong,
    );
    printf(
        b"SIZE_MAX=%s\n\0" as *const u8 as *const libc::c_char,
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    printf(
        b"SIZE_OFLOW=%s\n\0" as *const u8 as *const libc::c_char,
        decimal_absval_add_one(limit.as_mut_ptr()),
    );
    sprintf(
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        b"%lu\0" as *const u8 as *const libc::c_char,
        9223372036854775807 as libc::c_long as uintmax_t,
    );
    printf(
        b"SSIZE_MAX=%s\n\0" as *const u8 as *const libc::c_char,
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    printf(
        b"SSIZE_OFLOW=%s\n\0" as *const u8 as *const libc::c_char,
        decimal_absval_add_one(limit.as_mut_ptr()),
    );
    if !if (0 as libc::c_int as ssize_t) < -(1 as libc::c_int) as ssize_t {
        -(1 as libc::c_int) as ssize_t
    } else {
        (((1 as libc::c_int as ssize_t)
            << (::core::mem::size_of::<ssize_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
            - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long
    } != 0
    {
        sprintf(
            limit.as_mut_ptr().offset(1 as libc::c_int as isize),
            b"%ld\0" as *const u8 as *const libc::c_char,
            !if (0 as libc::c_int as ssize_t) < -(1 as libc::c_int) as ssize_t {
                -(1 as libc::c_int) as ssize_t
            } else {
                (((1 as libc::c_int as ssize_t)
                    << (::core::mem::size_of::<ssize_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            },
        );
        printf(
            b"SSIZE_MIN=%s\n\0" as *const u8 as *const libc::c_char,
            limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        printf(
            b"SSIZE_UFLOW=%s\n\0" as *const u8 as *const libc::c_char,
            decimal_absval_add_one(limit.as_mut_ptr()),
        );
    }
    sprintf(
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        b"%lu\0" as *const u8 as *const libc::c_char,
        (if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
            -(1 as libc::c_int) as time_t
        } else {
            (((1 as libc::c_int as time_t)
                << (::core::mem::size_of::<time_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        }) as uintmax_t,
    );
    printf(
        b"TIME_T_MAX=%s\n\0" as *const u8 as *const libc::c_char,
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    printf(
        b"TIME_T_OFLOW=%s\n\0" as *const u8 as *const libc::c_char,
        decimal_absval_add_one(limit.as_mut_ptr()),
    );
    if !if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
        -(1 as libc::c_int) as time_t
    } else {
        (((1 as libc::c_int as time_t)
            << (::core::mem::size_of::<time_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
            - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long
    } != 0
    {
        sprintf(
            limit.as_mut_ptr().offset(1 as libc::c_int as isize),
            b"%ld\0" as *const u8 as *const libc::c_char,
            !if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
                -(1 as libc::c_int) as time_t
            } else {
                (((1 as libc::c_int as time_t)
                    << (::core::mem::size_of::<time_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            },
        );
        printf(
            b"TIME_T_MIN=%s\n\0" as *const u8 as *const libc::c_char,
            limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        printf(
            b"TIME_T_UFLOW=%s\n\0" as *const u8 as *const libc::c_char,
            decimal_absval_add_one(limit.as_mut_ptr()),
        );
    }
    sprintf(
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        b"%lu\0" as *const u8 as *const libc::c_char,
        (if (0 as libc::c_int as uid_t) < -(1 as libc::c_int) as uid_t {
            -(1 as libc::c_int) as uid_t
        } else {
            ((1 as libc::c_int as uid_t)
                << (::core::mem::size_of::<uid_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        }) as uintmax_t,
    );
    printf(
        b"UID_T_MAX=%s\n\0" as *const u8 as *const libc::c_char,
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    printf(
        b"UID_T_OFLOW=%s\n\0" as *const u8 as *const libc::c_char,
        decimal_absval_add_one(limit.as_mut_ptr()),
    );
    sprintf(
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        b"%lu\0" as *const u8 as *const libc::c_char,
        (if (0 as libc::c_int as gid_t) < -(1 as libc::c_int) as gid_t {
            -(1 as libc::c_int) as gid_t
        } else {
            ((1 as libc::c_int as gid_t)
                << (::core::mem::size_of::<gid_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        }) as uintmax_t,
    );
    printf(
        b"GID_T_MAX=%s\n\0" as *const u8 as *const libc::c_char,
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    printf(
        b"GID_T_OFLOW=%s\n\0" as *const u8 as *const libc::c_char,
        decimal_absval_add_one(limit.as_mut_ptr()),
    );
    sprintf(
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        b"%lu\0" as *const u8 as *const libc::c_char,
        (if (0 as libc::c_int) < -(1 as libc::c_int) {
            -(1 as libc::c_int)
        } else {
            (((1 as libc::c_int)
                << (::core::mem::size_of::<pid_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong)) - 1 as libc::c_int)
                * 2 as libc::c_int + 1 as libc::c_int
        }) as uintmax_t,
    );
    printf(
        b"PID_T_MAX=%s\n\0" as *const u8 as *const libc::c_char,
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    printf(
        b"PID_T_OFLOW=%s\n\0" as *const u8 as *const libc::c_char,
        decimal_absval_add_one(limit.as_mut_ptr()),
    );
    if !if (0 as libc::c_int) < -(1 as libc::c_int) {
        -(1 as libc::c_int)
    } else {
        (((1 as libc::c_int)
            << (::core::mem::size_of::<pid_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong)) - 1 as libc::c_int)
            * 2 as libc::c_int + 1 as libc::c_int
    } != 0
    {
        sprintf(
            limit.as_mut_ptr().offset(1 as libc::c_int as isize),
            b"%ld\0" as *const u8 as *const libc::c_char,
            !if (0 as libc::c_int) < -(1 as libc::c_int) {
                -(1 as libc::c_int)
            } else {
                (((1 as libc::c_int)
                    << (::core::mem::size_of::<pid_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
            } as intmax_t,
        );
        printf(
            b"PID_T_MIN=%s\n\0" as *const u8 as *const libc::c_char,
            limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        printf(
            b"PID_T_UFLOW=%s\n\0" as *const u8 as *const libc::c_char,
            decimal_absval_add_one(limit.as_mut_ptr()),
        );
    }
    sprintf(
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        b"%lu\0" as *const u8 as *const libc::c_char,
        (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
            -(1 as libc::c_int) as off_t
        } else {
            (((1 as libc::c_int as off_t)
                << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        }) as uintmax_t,
    );
    printf(
        b"OFF_T_MAX=%s\n\0" as *const u8 as *const libc::c_char,
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    printf(
        b"OFF_T_OFLOW=%s\n\0" as *const u8 as *const libc::c_char,
        decimal_absval_add_one(limit.as_mut_ptr()),
    );
    if !if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
        -(1 as libc::c_int) as off_t
    } else {
        (((1 as libc::c_int as off_t)
            << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
            - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long
    } != 0
    {
        sprintf(
            limit.as_mut_ptr().offset(1 as libc::c_int as isize),
            b"%ld\0" as *const u8 as *const libc::c_char,
            !if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                -(1 as libc::c_int) as off_t
            } else {
                (((1 as libc::c_int as off_t)
                    << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            },
        );
        printf(
            b"OFF_T_MIN=%s\n\0" as *const u8 as *const libc::c_char,
            limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        printf(
            b"OFF_T_UFLOW=%s\n\0" as *const u8 as *const libc::c_char,
            decimal_absval_add_one(limit.as_mut_ptr()),
        );
    }
    sprintf(
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        b"%lu\0" as *const u8 as *const libc::c_char,
        9223372036854775807 as libc::c_long as uintmax_t,
    );
    printf(
        b"INTMAX_MAX=%s\n\0" as *const u8 as *const libc::c_char,
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    printf(
        b"INTMAX_OFLOW=%s\n\0" as *const u8 as *const libc::c_char,
        decimal_absval_add_one(limit.as_mut_ptr()),
    );
    if -(9223372036854775807 as libc::c_long) - 1 as libc::c_int as libc::c_long != 0 {
        sprintf(
            limit.as_mut_ptr().offset(1 as libc::c_int as isize),
            b"%ld\0" as *const u8 as *const libc::c_char,
            -(9223372036854775807 as libc::c_long) - 1 as libc::c_int as libc::c_long,
        );
        printf(
            b"INTMAX_MIN=%s\n\0" as *const u8 as *const libc::c_char,
            limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        printf(
            b"INTMAX_UFLOW=%s\n\0" as *const u8 as *const libc::c_char,
            decimal_absval_add_one(limit.as_mut_ptr()),
        );
    }
    sprintf(
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
        b"%lu\0" as *const u8 as *const libc::c_char,
        18446744073709551615 as libc::c_ulong,
    );
    printf(
        b"UINTMAX_MAX=%s\n\0" as *const u8 as *const libc::c_char,
        limit.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    printf(
        b"UINTMAX_OFLOW=%s\n\0" as *const u8 as *const libc::c_char,
        decimal_absval_add_one(limit.as_mut_ptr()),
    );
    printf(b"FLT_MIN=\0" as *const u8 as *const libc::c_char);
    print_FLT(1.17549435e-38f32);
    printf(b"FLT_MAX=\0" as *const u8 as *const libc::c_char);
    print_FLT(3.40282347e+38f32);
    printf(b"DBL_MIN=\0" as *const u8 as *const libc::c_char);
    print_DBL(2.2250738585072014e-308f64);
    printf(b"DBL_MAX=\0" as *const u8 as *const libc::c_char);
    print_DBL(1.7976931348623157e+308f64);
    printf(b"LDBL_MIN=\0" as *const u8 as *const libc::c_char);
    print_LDBL(f128::f128::new(3.36210314311209350626e-4932));
    printf(b"LDBL_MAX=\0" as *const u8 as *const libc::c_char);
    print_LDBL(f128::f128::MAX);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
