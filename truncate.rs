#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]


extern crate selinux_sys;
extern crate libc;
pub mod src {
pub mod lib {
pub mod c_strcasecmp;
pub mod close_stream;
pub mod closeout;
pub mod exitfail;
pub mod fclose;
pub mod fflush;
pub mod fseeko;
pub mod hard_locale;
pub mod localcharset;
pub mod mbrtowc;
pub mod progname;
pub mod quotearg;
pub mod setlocale_null;
pub mod version_etc;
pub mod version_etc_fsf;
pub mod xalloc_die;
pub mod xdectoimax;
pub mod xmalloc;
pub mod xstrtoimax;
} // mod lib
pub mod src {
pub mod version;
} // mod src
} // mod src
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    static mut Version: *const libc::c_char;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn xdectoimax(
        n_str: *const libc::c_char,
        min: intmax_t,
        max: intmax_t,
        suffixes: *const libc::c_char,
        err: *const libc::c_char,
        err_exit: libc::c_int,
    ) -> intmax_t;
}
pub type size_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type ptrdiff_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type intmax_t = __intmax_t;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type C2RustUnnamed_0 = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed_0 = -131;
pub const GETOPT_HELP_CHAR: C2RustUnnamed_0 = -130;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct infomap {
    pub program: *const libc::c_char,
    pub node: *const libc::c_char,
}
pub type quoting_style = libc::c_uint;
pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const escape_quoting_style: quoting_style = 7;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
pub type rel_mode_t = libc::c_uint;
pub const rm_rup: rel_mode_t = 5;
pub const rm_rdn: rel_mode_t = 4;
pub const rm_max: rel_mode_t = 3;
pub const rm_min: rel_mode_t = 2;
pub const rm_rel: rel_mode_t = 1;
pub const rm_abs: rel_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub _gl_dummy: libc::c_int,
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
            gettext(
                b"Usage: %s OPTION... FILE...\n\0" as *const u8 as *const libc::c_char,
            ),
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"Shrink or extend the size of each FILE to the specified size\n\nA FILE argument that does not exist is created.\n\nIf a FILE is larger than the specified size, the extra data is lost.\nIf a FILE is shorter, it is extended and the sparse extended part (hole)\nreads as zero bytes.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            gettext(
                b"  -c, --no-create        do not create any files\n\0" as *const u8
                    as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -o, --io-blocks        treat SIZE as number of IO blocks instead of bytes\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -r, --reference=RFILE  base size on RFILE\n  -s, --size=SIZE        set or adjust the file size by SIZE bytes\n\0"
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
        emit_size_note();
        fputs_unlocked(
            gettext(
                b"\nSIZE may also be prefixed by one of the following modifying characters:\n'+' extend by, '-' reduce by, '<' at most, '>' at least,\n'/' round down to multiple of, '%' round up to multiple of.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_ancillary_info(b"truncate\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
#[inline]
unsafe extern "C" fn usable_st_size(mut sb: *const stat) -> bool {
    return (*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
        || (*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
        || ((*sb).st_mode).wrapping_sub((*sb).st_mode) != 0 || 0 as libc::c_int != 0;
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
unsafe extern "C" fn emit_size_note() {
    fputs_unlocked(
        gettext(
            b"\nThe SIZE argument is an integer and optional unit (example: 10K is 10*1024).\nUnits are K,M,G,T,P,E,Z,Y (powers of 1024) or KB,MB,... (powers of 1000).\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n\0"
                as *const u8 as *const libc::c_char,
        ),
        stdout,
    );
}
#[inline]
unsafe extern "C" fn emit_mandatory_arg_note() {
    fputs_unlocked(
        gettext(
            b"\nMandatory arguments to long options are mandatory for short options too.\n\0"
                as *const u8 as *const libc::c_char,
        ),
        stdout,
    );
}
#[inline]
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
}
static mut no_create: bool = false;
static mut block_mode: bool = false;
static mut ref_file: *const libc::c_char = 0 as *const libc::c_char;
static mut longopts: [option; 7] = [
    {
        let mut init = option {
            name: b"no-create\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"io-blocks\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"reference\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"size\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: GETOPT_HELP_CHAR as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: GETOPT_VERSION_CHAR as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn do_ftruncate(
    mut fd: libc::c_int,
    mut fname: *const libc::c_char,
    mut ssize: off_t,
    mut rsize: off_t,
    mut rel_mode: rel_mode_t,
) -> bool {
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut nsize: off_t = 0;
    if (block_mode as libc::c_int != 0
        || rel_mode as libc::c_uint != 0 && rsize < 0 as libc::c_int as libc::c_long)
        && fstat(fd, &mut sb) != 0 as libc::c_int
    {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(b"cannot fstat %s\0" as *const u8 as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, fname),
        );
        return 0 as libc::c_int != 0;
    }
    if block_mode {
        let mut blksize: ptrdiff_t = if (0 as libc::c_int as libc::c_long)
            < sb.st_blksize
            && sb.st_blksize as libc::c_ulong
                <= (-(1 as libc::c_int) as size_t)
                    .wrapping_div(8 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            sb.st_blksize
        } else {
            512 as libc::c_int as libc::c_long
        };
        let mut ssize0: intmax_t = ssize;
        if if ::core::mem::size_of::<off_t>() as libc::c_ulong
            == ::core::mem::size_of::<libc::c_schar>() as libc::c_ulong
        {
            if !((0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t) {
                if if blksize < 0 as libc::c_int as libc::c_long {
                    if ssize < 0 as libc::c_int as libc::c_long {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                127 as libc::c_int
                            }) as libc::c_long + blksize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            (ssize < 127 as libc::c_int as libc::c_long / blksize)
                                as libc::c_int
                        } else {
                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 0 as libc::c_int as libc::c_long
                            }) < 0 as libc::c_int as libc::c_long
                            {
                                (blksize
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            blksize
                                        }) + 1 as libc::c_int as libc::c_long)
                                            << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            blksize
                                        }) - 1 as libc::c_int as libc::c_long
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long) < blksize)
                                    as libc::c_int
                            }) != 0
                            {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 127 as libc::c_int as libc::c_long
                                    >> (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            } else {
                                127 as libc::c_int as libc::c_long / -blksize
                            }) <= -(1 as libc::c_int) as libc::c_long - ssize)
                                as libc::c_int
                        }
                    } else if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            })
                                + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            })
                                + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                })
                                    + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    })
                                        + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    })
                                        + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            })
                                + (-(127 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long) as libc::c_int
                    }) != 0 && blksize == -(1 as libc::c_int) as libc::c_long
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((0 as libc::c_int as libc::c_long)
                                < ssize
                                    + (-(127 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < ssize
                                && ((-(1 as libc::c_int)
                                    - (-(127 as libc::c_int) - 1 as libc::c_int))
                                    as libc::c_long) < ssize - 1 as libc::c_int as libc::c_long)
                                as libc::c_int
                        }
                    } else {
                        ((-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                            / blksize < ssize) as libc::c_int
                    }
                } else if blksize == 0 as libc::c_int as libc::c_long {
                    0 as libc::c_int
                } else if ssize < 0 as libc::c_int as libc::c_long {
                    if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                ssize
                            })
                                + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                ssize
                            })
                                + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    ssize
                                })
                                    + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        ssize
                                    })
                                        + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        ssize
                                    })
                                        + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                ssize
                            })
                                + (-(127 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long) as libc::c_int
                    }) != 0 && ssize == -(1 as libc::c_int) as libc::c_long
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((0 as libc::c_int as libc::c_long)
                                < blksize
                                    + (-(127 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long) as libc::c_int
                        } else {
                            (((-(1 as libc::c_int)
                                - (-(127 as libc::c_int) - 1 as libc::c_int))
                                as libc::c_long)
                                < blksize - 1 as libc::c_int as libc::c_long) as libc::c_int
                        }
                    } else {
                        ((-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                            / ssize < blksize) as libc::c_int
                    }
                } else {
                    (127 as libc::c_int as libc::c_long / blksize < ssize) as libc::c_int
                } != 0
                {
                    ssize = (ssize as libc::c_uint).wrapping_mul(blksize as libc::c_uint)
                        as libc::c_schar as off_t;
                    1 as libc::c_int
                } else {
                    ssize = (ssize as libc::c_uint).wrapping_mul(blksize as libc::c_uint)
                        as libc::c_schar as off_t;
                    0 as libc::c_int
                }
            } else if if blksize < 0 as libc::c_int as libc::c_long {
                if ssize < 0 as libc::c_int as libc::c_long {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                        }) as libc::c_long + blksize
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        (ssize
                            < (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                as libc::c_long / blksize) as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            (blksize
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    }) - 1 as libc::c_int as libc::c_long
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < blksize) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            })
                                + (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                    as libc::c_long
                                >> (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                as libc::c_long / -blksize
                        }) <= -(1 as libc::c_int) as libc::c_long - ssize) as libc::c_int
                    }
                } else if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blksize
                    }) + 0 as libc::c_int as libc::c_long
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + 0 as libc::c_int as libc::c_long
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + 0 as libc::c_int as libc::c_long
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blksize
                    }) + 0 as libc::c_int as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 0 as libc::c_int as libc::c_long
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 0 as libc::c_int as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                }) != 0 && blksize == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        ssize
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < ssize + 0 as libc::c_int as libc::c_long) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < ssize
                            && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                                < ssize - 1 as libc::c_int as libc::c_long) as libc::c_int
                    }
                } else {
                    (0 as libc::c_int as libc::c_long / blksize < ssize) as libc::c_int
                }
            } else if blksize == 0 as libc::c_int as libc::c_long {
                0 as libc::c_int
            } else if ssize < 0 as libc::c_int as libc::c_long {
                if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        ssize
                    }) + 0 as libc::c_int as libc::c_long
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) + 0 as libc::c_int as libc::c_long
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) + 0 as libc::c_int as libc::c_long
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        ssize
                    }) + 0 as libc::c_int as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                ssize
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    ssize
                                }) + 0 as libc::c_int as libc::c_long
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    ssize
                                }) + 0 as libc::c_int as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                }) != 0 && ssize == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blksize
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < blksize + 0 as libc::c_int as libc::c_long) as libc::c_int
                    } else {
                        (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                            < blksize - 1 as libc::c_int as libc::c_long) as libc::c_int
                    }
                } else {
                    (0 as libc::c_int as libc::c_long / ssize < blksize) as libc::c_int
                }
            } else {
                ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                    as libc::c_long / blksize < ssize) as libc::c_int
            } != 0
            {
                ssize = (ssize as libc::c_uint).wrapping_mul(blksize as libc::c_uint)
                    as libc::c_uchar as off_t;
                1 as libc::c_int
            } else {
                ssize = (ssize as libc::c_uint).wrapping_mul(blksize as libc::c_uint)
                    as libc::c_uchar as off_t;
                0 as libc::c_int
            }
        } else if ::core::mem::size_of::<off_t>() as libc::c_ulong
            == ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
        {
            if !((0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t) {
                if if blksize < 0 as libc::c_int as libc::c_long {
                    if ssize < 0 as libc::c_int as libc::c_long {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                32767 as libc::c_int
                            }) as libc::c_long + blksize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            (ssize < 32767 as libc::c_int as libc::c_long / blksize)
                                as libc::c_int
                        } else {
                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 0 as libc::c_int as libc::c_long
                            }) < 0 as libc::c_int as libc::c_long
                            {
                                (blksize
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            blksize
                                        }) + 1 as libc::c_int as libc::c_long)
                                            << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            blksize
                                        }) - 1 as libc::c_int as libc::c_long
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long) < blksize)
                                    as libc::c_int
                            }) != 0
                            {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 32767 as libc::c_int as libc::c_long
                                    >> (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            } else {
                                32767 as libc::c_int as libc::c_long / -blksize
                            }) <= -(1 as libc::c_int) as libc::c_long - ssize)
                                as libc::c_int
                        }
                    } else if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            })
                                + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            })
                                + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        })
                            + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_long)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                })
                                    + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    })
                                        + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    })
                                        + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            })
                                + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long) as libc::c_int
                    }) != 0 && blksize == -(1 as libc::c_int) as libc::c_long
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((0 as libc::c_int as libc::c_long)
                                < ssize
                                    + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < ssize
                                && ((-(1 as libc::c_int)
                                    - (-(32767 as libc::c_int) - 1 as libc::c_int))
                                    as libc::c_long) < ssize - 1 as libc::c_int as libc::c_long)
                                as libc::c_int
                        }
                    } else {
                        ((-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                            / blksize < ssize) as libc::c_int
                    }
                } else if blksize == 0 as libc::c_int as libc::c_long {
                    0 as libc::c_int
                } else if ssize < 0 as libc::c_int as libc::c_long {
                    if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) + (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                ssize
                            })
                                + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                ssize
                            })
                                + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        })
                            + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_long)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    ssize
                                })
                                    + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        ssize
                                    })
                                        + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        ssize
                                    })
                                        + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                ssize
                            })
                                + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long) as libc::c_int
                    }) != 0 && ssize == -(1 as libc::c_int) as libc::c_long
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((0 as libc::c_int as libc::c_long)
                                < blksize
                                    + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long) as libc::c_int
                        } else {
                            (((-(1 as libc::c_int)
                                - (-(32767 as libc::c_int) - 1 as libc::c_int))
                                as libc::c_long)
                                < blksize - 1 as libc::c_int as libc::c_long) as libc::c_int
                        }
                    } else {
                        ((-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                            / ssize < blksize) as libc::c_int
                    }
                } else {
                    (32767 as libc::c_int as libc::c_long / blksize < ssize)
                        as libc::c_int
                } != 0
                {
                    ssize = (ssize as libc::c_uint).wrapping_mul(blksize as libc::c_uint)
                        as libc::c_short as off_t;
                    1 as libc::c_int
                } else {
                    ssize = (ssize as libc::c_uint).wrapping_mul(blksize as libc::c_uint)
                        as libc::c_short as off_t;
                    0 as libc::c_int
                }
            } else if if blksize < 0 as libc::c_int as libc::c_long {
                if ssize < 0 as libc::c_int as libc::c_long {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                        }) as libc::c_long + blksize
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        (ssize
                            < (32767 as libc::c_int * 2 as libc::c_int
                                + 1 as libc::c_int) as libc::c_long / blksize)
                            as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            (blksize
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    }) - 1 as libc::c_int as libc::c_long
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < blksize) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            })
                                + (32767 as libc::c_int * 2 as libc::c_int
                                    + 1 as libc::c_int) as libc::c_long
                                >> (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                as libc::c_long / -blksize
                        }) <= -(1 as libc::c_int) as libc::c_long - ssize) as libc::c_int
                    }
                } else if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blksize
                    }) + 0 as libc::c_int as libc::c_long
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + 0 as libc::c_int as libc::c_long
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + 0 as libc::c_int as libc::c_long
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blksize
                    }) + 0 as libc::c_int as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 0 as libc::c_int as libc::c_long
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 0 as libc::c_int as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                }) != 0 && blksize == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        ssize
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < ssize + 0 as libc::c_int as libc::c_long) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < ssize
                            && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                                < ssize - 1 as libc::c_int as libc::c_long) as libc::c_int
                    }
                } else {
                    (0 as libc::c_int as libc::c_long / blksize < ssize) as libc::c_int
                }
            } else if blksize == 0 as libc::c_int as libc::c_long {
                0 as libc::c_int
            } else if ssize < 0 as libc::c_int as libc::c_long {
                if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        ssize
                    }) + 0 as libc::c_int as libc::c_long
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) + 0 as libc::c_int as libc::c_long
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) + 0 as libc::c_int as libc::c_long
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        ssize
                    }) + 0 as libc::c_int as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                ssize
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    ssize
                                }) + 0 as libc::c_int as libc::c_long
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    ssize
                                }) + 0 as libc::c_int as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                }) != 0 && ssize == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blksize
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < blksize + 0 as libc::c_int as libc::c_long) as libc::c_int
                    } else {
                        (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                            < blksize - 1 as libc::c_int as libc::c_long) as libc::c_int
                    }
                } else {
                    (0 as libc::c_int as libc::c_long / ssize < blksize) as libc::c_int
                }
            } else {
                ((32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                    as libc::c_long / blksize < ssize) as libc::c_int
            } != 0
            {
                ssize = (ssize as libc::c_uint).wrapping_mul(blksize as libc::c_uint)
                    as libc::c_ushort as off_t;
                1 as libc::c_int
            } else {
                ssize = (ssize as libc::c_uint).wrapping_mul(blksize as libc::c_uint)
                    as libc::c_ushort as off_t;
                0 as libc::c_int
            }
        } else if ::core::mem::size_of::<off_t>() as libc::c_ulong
            == ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
        {
            if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                ssize
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                if if blksize < 0 as libc::c_int as libc::c_long {
                    if ssize < 0 as libc::c_int as libc::c_long {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2147483647 as libc::c_int
                            }) as libc::c_long + blksize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            (ssize < 2147483647 as libc::c_int as libc::c_long / blksize)
                                as libc::c_int
                        } else {
                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 0 as libc::c_int as libc::c_long
                            }) < 0 as libc::c_int as libc::c_long
                            {
                                (blksize
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            blksize
                                        }) + 1 as libc::c_int as libc::c_long)
                                            << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            blksize
                                        }) - 1 as libc::c_int as libc::c_long
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long) < blksize)
                                    as libc::c_int
                            }) != 0
                            {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 2147483647 as libc::c_int as libc::c_long
                                    >> (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            } else {
                                2147483647 as libc::c_int as libc::c_long / -blksize
                            }) <= -(1 as libc::c_int) as libc::c_long - ssize)
                                as libc::c_int
                        }
                    } else if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        })
                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            })
                                + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            })
                                + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        })
                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_long)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                })
                                    + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    })
                                        + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    })
                                        + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            })
                                + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long) as libc::c_int
                    }) != 0 && blksize == -(1 as libc::c_int) as libc::c_long
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((0 as libc::c_int as libc::c_long)
                                < ssize
                                    + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < ssize
                                && ((-(1 as libc::c_int)
                                    - (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                    as libc::c_long) < ssize - 1 as libc::c_int as libc::c_long)
                                as libc::c_int
                        }
                    } else {
                        ((-(2147483647 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_long / blksize < ssize) as libc::c_int
                    }
                } else if blksize == 0 as libc::c_int as libc::c_long {
                    0 as libc::c_int
                } else if ssize < 0 as libc::c_int as libc::c_long {
                    if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        })
                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                ssize
                            })
                                + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                ssize
                            })
                                + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        })
                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_long)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    ssize
                                })
                                    + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        ssize
                                    })
                                        + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        ssize
                                    })
                                        + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                ssize
                            })
                                + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long) as libc::c_int
                    }) != 0 && ssize == -(1 as libc::c_int) as libc::c_long
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((0 as libc::c_int as libc::c_long)
                                < blksize
                                    + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long) as libc::c_int
                        } else {
                            (((-(1 as libc::c_int)
                                - (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                as libc::c_long)
                                < blksize - 1 as libc::c_int as libc::c_long) as libc::c_int
                        }
                    } else {
                        ((-(2147483647 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_long / ssize < blksize) as libc::c_int
                    }
                } else {
                    (2147483647 as libc::c_int as libc::c_long / blksize < ssize)
                        as libc::c_int
                } != 0
                {
                    ssize = (ssize as libc::c_uint).wrapping_mul(blksize as libc::c_uint)
                        as libc::c_int as off_t;
                    1 as libc::c_int
                } else {
                    ssize = (ssize as libc::c_uint).wrapping_mul(blksize as libc::c_uint)
                        as libc::c_int as off_t;
                    0 as libc::c_int
                }
            } else if if blksize < 0 as libc::c_int as libc::c_long {
                if ssize < 0 as libc::c_int as libc::c_long {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_uint
                        } else {
                            (2147483647 as libc::c_int as libc::c_uint)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_add(1 as libc::c_uint)
                        }) as libc::c_long + blksize
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        (ssize
                            < (2147483647 as libc::c_int as libc::c_uint)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_add(1 as libc::c_uint) as libc::c_long / blksize)
                            as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            (blksize
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    }) - 1 as libc::c_int as libc::c_long
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < blksize) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            })
                                + (2147483647 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_add(1 as libc::c_uint) as libc::c_long
                                >> (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (2147483647 as libc::c_int as libc::c_uint)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_add(1 as libc::c_uint) as libc::c_long / -blksize
                        }) <= -(1 as libc::c_int) as libc::c_long - ssize) as libc::c_int
                    }
                } else if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blksize
                    }) + 0 as libc::c_int as libc::c_long
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + 0 as libc::c_int as libc::c_long
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + 0 as libc::c_int as libc::c_long
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blksize
                    }) + 0 as libc::c_int as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 0 as libc::c_int as libc::c_long
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 0 as libc::c_int as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                }) != 0 && blksize == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        ssize
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < ssize + 0 as libc::c_int as libc::c_long) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < ssize
                            && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                                < ssize - 1 as libc::c_int as libc::c_long) as libc::c_int
                    }
                } else {
                    (0 as libc::c_int as libc::c_long / blksize < ssize) as libc::c_int
                }
            } else if blksize == 0 as libc::c_int as libc::c_long {
                0 as libc::c_int
            } else if ssize < 0 as libc::c_int as libc::c_long {
                if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        ssize
                    }) + 0 as libc::c_int as libc::c_long
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) + 0 as libc::c_int as libc::c_long
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) + 0 as libc::c_int as libc::c_long
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        ssize
                    }) + 0 as libc::c_int as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                ssize
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    ssize
                                }) + 0 as libc::c_int as libc::c_long
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    ssize
                                }) + 0 as libc::c_int as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                }) != 0 && ssize == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blksize
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < blksize + 0 as libc::c_int as libc::c_long) as libc::c_int
                    } else {
                        (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                            < blksize - 1 as libc::c_int as libc::c_long) as libc::c_int
                    }
                } else {
                    (0 as libc::c_int as libc::c_long / ssize < blksize) as libc::c_int
                }
            } else {
                ((2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_add(1 as libc::c_uint) as libc::c_long / blksize < ssize)
                    as libc::c_int
            } != 0
            {
                ssize = (ssize as libc::c_uint).wrapping_mul(blksize as libc::c_uint)
                    as off_t;
                1 as libc::c_int
            } else {
                ssize = (ssize as libc::c_uint).wrapping_mul(blksize as libc::c_uint)
                    as off_t;
                0 as libc::c_int
            }
        } else if ::core::mem::size_of::<off_t>() as libc::c_ulong
            == ::core::mem::size_of::<libc::c_long>() as libc::c_ulong
        {
            if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                ssize
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                if if blksize < 0 as libc::c_int as libc::c_long {
                    if ssize < 0 as libc::c_int as libc::c_long {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                9223372036854775807 as libc::c_long
                            }) + blksize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            (ssize < 9223372036854775807 as libc::c_long / blksize)
                                as libc::c_int
                        } else {
                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 0 as libc::c_int as libc::c_long
                            }) < 0 as libc::c_int as libc::c_long
                            {
                                (blksize
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            blksize
                                        }) + 1 as libc::c_int as libc::c_long)
                                            << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            blksize
                                        }) - 1 as libc::c_int as libc::c_long
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long) < blksize)
                                    as libc::c_int
                            }) != 0
                            {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 9223372036854775807 as libc::c_long
                                    >> (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            } else {
                                9223372036854775807 as libc::c_long / -blksize
                            }) <= -(1 as libc::c_int) as libc::c_long - ssize)
                                as libc::c_int
                        }
                    } else if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            })
                                + (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long)
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            })
                                + (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long)
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                })
                                    + (-(9223372036854775807 as libc::c_long)
                                        - 1 as libc::c_long)
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    })
                                        + (-(9223372036854775807 as libc::c_long)
                                            - 1 as libc::c_long)
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    })
                                        + (-(9223372036854775807 as libc::c_long)
                                            - 1 as libc::c_long)
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            })
                                + (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long)) as libc::c_int
                    }) != 0 && blksize == -(1 as libc::c_int) as libc::c_long
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((0 as libc::c_int as libc::c_long)
                                < ssize
                                    + (-(9223372036854775807 as libc::c_long)
                                        - 1 as libc::c_long)) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < ssize
                                && -(1 as libc::c_int) as libc::c_long
                                    - (-(9223372036854775807 as libc::c_long)
                                        - 1 as libc::c_long)
                                    < ssize - 1 as libc::c_int as libc::c_long) as libc::c_int
                        }
                    } else {
                        ((-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                            / blksize < ssize) as libc::c_int
                    }
                } else if blksize == 0 as libc::c_int as libc::c_long {
                    0 as libc::c_int
                } else if ssize < 0 as libc::c_int as libc::c_long {
                    if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) + (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                ssize
                            })
                                + (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long)
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                ssize
                            })
                                + (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long)
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) + (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    ssize
                                })
                                    + (-(9223372036854775807 as libc::c_long)
                                        - 1 as libc::c_long)
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        ssize
                                    })
                                        + (-(9223372036854775807 as libc::c_long)
                                            - 1 as libc::c_long)
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        ssize
                                    })
                                        + (-(9223372036854775807 as libc::c_long)
                                            - 1 as libc::c_long)
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                ssize
                            })
                                + (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long)) as libc::c_int
                    }) != 0 && ssize == -(1 as libc::c_int) as libc::c_long
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((0 as libc::c_int as libc::c_long)
                                < blksize
                                    + (-(9223372036854775807 as libc::c_long)
                                        - 1 as libc::c_long)) as libc::c_int
                        } else {
                            (-(1 as libc::c_int) as libc::c_long
                                - (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long)
                                < blksize - 1 as libc::c_int as libc::c_long) as libc::c_int
                        }
                    } else {
                        ((-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                            / ssize < blksize) as libc::c_int
                    }
                } else {
                    (9223372036854775807 as libc::c_long / blksize < ssize)
                        as libc::c_int
                } != 0
                {
                    ssize = (ssize as libc::c_ulong)
                        .wrapping_mul(blksize as libc::c_ulong) as libc::c_long;
                    1 as libc::c_int
                } else {
                    ssize = (ssize as libc::c_ulong)
                        .wrapping_mul(blksize as libc::c_ulong) as libc::c_long;
                    0 as libc::c_int
                }
            } else if if blksize < 0 as libc::c_int as libc::c_long {
                if ssize < 0 as libc::c_int as libc::c_long {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                        })
                            .wrapping_add(blksize as libc::c_ulong)
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        ((ssize as libc::c_ulong)
                            < (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(blksize as libc::c_ulong)) as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            (blksize
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    }) - 1 as libc::c_int as libc::c_long
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < blksize) as libc::c_int
                        }) != 0
                        {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) as libc::c_ulong)
                                .wrapping_add(
                                    (9223372036854775807 as libc::c_long as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_ulong),
                                )
                                >> (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(-blksize as libc::c_ulong)
                        })
                            <= (-(1 as libc::c_int) as libc::c_long - ssize)
                                as libc::c_ulong) as libc::c_int
                    }
                } else if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blksize
                    }) + 0 as libc::c_int as libc::c_long
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + 0 as libc::c_int as libc::c_long
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + 0 as libc::c_int as libc::c_long
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blksize
                    }) + 0 as libc::c_int as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 0 as libc::c_int as libc::c_long
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 0 as libc::c_int as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                }) != 0 && blksize == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        ssize
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < ssize + 0 as libc::c_int as libc::c_long) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < ssize
                            && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                                < ssize - 1 as libc::c_int as libc::c_long) as libc::c_int
                    }
                } else {
                    (0 as libc::c_int as libc::c_long / blksize < ssize) as libc::c_int
                }
            } else if blksize == 0 as libc::c_int as libc::c_long {
                0 as libc::c_int
            } else if ssize < 0 as libc::c_int as libc::c_long {
                if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        ssize
                    }) + 0 as libc::c_int as libc::c_long
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) + 0 as libc::c_int as libc::c_long
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) + 0 as libc::c_int as libc::c_long
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        ssize
                    }) + 0 as libc::c_int as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                ssize
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    ssize
                                }) + 0 as libc::c_int as libc::c_long
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    ssize
                                }) + 0 as libc::c_int as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                }) != 0 && ssize == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blksize
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < blksize + 0 as libc::c_int as libc::c_long) as libc::c_int
                    } else {
                        (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                            < blksize - 1 as libc::c_int as libc::c_long) as libc::c_int
                    }
                } else {
                    (0 as libc::c_int as libc::c_long / ssize < blksize) as libc::c_int
                }
            } else {
                ((9223372036854775807 as libc::c_long as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_ulong)
                    .wrapping_add(1 as libc::c_ulong)
                    .wrapping_div(blksize as libc::c_ulong) < ssize as libc::c_ulong)
                    as libc::c_int
            } != 0
            {
                ssize = (ssize as libc::c_ulong).wrapping_mul(blksize as libc::c_ulong)
                    as off_t;
                1 as libc::c_int
            } else {
                ssize = (ssize as libc::c_ulong).wrapping_mul(blksize as libc::c_ulong)
                    as off_t;
                0 as libc::c_int
            }
        } else if ((if 1 as libc::c_int != 0 {
            0 as libc::c_int as libc::c_long
        } else {
            ssize
        }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
        {
            if if blksize < 0 as libc::c_int as libc::c_long {
                if ssize < 0 as libc::c_int as libc::c_long {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_longlong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_longlong
                        } else {
                            9223372036854775807 as libc::c_longlong
                        }) + blksize as libc::c_longlong
                    }) - 1 as libc::c_int as libc::c_longlong)
                        < 0 as libc::c_int as libc::c_longlong
                    {
                        ((ssize as libc::c_longlong)
                            < 9223372036854775807 as libc::c_longlong
                                / blksize as libc::c_longlong) as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            (blksize
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    }) - 1 as libc::c_int as libc::c_long
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < blksize) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) as libc::c_longlong
                                + 9223372036854775807 as libc::c_longlong
                                >> (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            9223372036854775807 as libc::c_longlong
                                / -blksize as libc::c_longlong
                        })
                            <= (-(1 as libc::c_int) as libc::c_long - ssize)
                                as libc::c_longlong) as libc::c_int
                    }
                } else if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_longlong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blksize
                    }) as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                }) - 1 as libc::c_int as libc::c_longlong)
                    < 0 as libc::c_int as libc::c_longlong
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_longlong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)
                    }) + 1 as libc::c_int as libc::c_longlong)
                        << (::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_longlong)
                        * 2 as libc::c_int as libc::c_longlong
                        + 1 as libc::c_int as libc::c_longlong)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_longlong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)
                    }) + 0 as libc::c_int as libc::c_longlong
                }) < 0 as libc::c_int as libc::c_longlong
                {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blksize
                    }) as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_longlong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) as libc::c_longlong
                                + (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong)
                        }) - 1 as libc::c_int as libc::c_longlong)
                            < 0 as libc::c_int as libc::c_longlong
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_longlong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) as libc::c_longlong
                                    + (-(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong)
                            }) + 1 as libc::c_int as libc::c_longlong)
                                << (::core::mem::size_of::<libc::c_longlong>()
                                    as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_longlong)
                                * 2 as libc::c_int as libc::c_longlong
                                + 1 as libc::c_int as libc::c_longlong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_longlong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) as libc::c_longlong
                                    + (-(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong)
                            }) - 1 as libc::c_int as libc::c_longlong
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_longlong)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)) as libc::c_int
                }) != 0 && blksize == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        ssize
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_longlong)
                            < ssize as libc::c_longlong
                                + (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong)) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < ssize
                            && -(1 as libc::c_int) as libc::c_longlong
                                - (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong)
                                < (ssize - 1 as libc::c_int as libc::c_long)
                                    as libc::c_longlong) as libc::c_int
                    }
                } else {
                    (((-(9223372036854775807 as libc::c_longlong)
                        - 1 as libc::c_longlong) / blksize as libc::c_longlong)
                        < ssize as libc::c_longlong) as libc::c_int
                }
            } else if blksize == 0 as libc::c_int as libc::c_long {
                0 as libc::c_int
            } else if ssize < 0 as libc::c_int as libc::c_long {
                if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_longlong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        ssize
                    }) as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                }) - 1 as libc::c_int as libc::c_longlong)
                    < 0 as libc::c_int as libc::c_longlong
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_longlong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)
                    }) + 1 as libc::c_int as libc::c_longlong)
                        << (::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_longlong)
                        * 2 as libc::c_int as libc::c_longlong
                        + 1 as libc::c_int as libc::c_longlong)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_longlong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)
                    }) + 0 as libc::c_int as libc::c_longlong
                }) < 0 as libc::c_int as libc::c_longlong
                {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        ssize
                    }) as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_longlong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                ssize
                            }) as libc::c_longlong
                                + (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong)
                        }) - 1 as libc::c_int as libc::c_longlong)
                            < 0 as libc::c_int as libc::c_longlong
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_longlong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    ssize
                                }) as libc::c_longlong
                                    + (-(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong)
                            }) + 1 as libc::c_int as libc::c_longlong)
                                << (::core::mem::size_of::<libc::c_longlong>()
                                    as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_longlong)
                                * 2 as libc::c_int as libc::c_longlong
                                + 1 as libc::c_int as libc::c_longlong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_longlong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    ssize
                                }) as libc::c_longlong
                                    + (-(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong)
                            }) - 1 as libc::c_int as libc::c_longlong
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_longlong)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)) as libc::c_int
                }) != 0 && ssize == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blksize
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_longlong)
                            < blksize as libc::c_longlong
                                + (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong)) as libc::c_int
                    } else {
                        (-(1 as libc::c_int) as libc::c_longlong
                            - (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)
                            < (blksize - 1 as libc::c_int as libc::c_long)
                                as libc::c_longlong) as libc::c_int
                    }
                } else {
                    (((-(9223372036854775807 as libc::c_longlong)
                        - 1 as libc::c_longlong) / ssize as libc::c_longlong)
                        < blksize as libc::c_longlong) as libc::c_int
                }
            } else {
                ((9223372036854775807 as libc::c_longlong / blksize as libc::c_longlong)
                    < ssize as libc::c_longlong) as libc::c_int
            } != 0
            {
                ssize = (ssize as libc::c_ulonglong)
                    .wrapping_mul(blksize as libc::c_ulonglong) as libc::c_longlong
                    as off_t;
                1 as libc::c_int
            } else {
                ssize = (ssize as libc::c_ulonglong)
                    .wrapping_mul(blksize as libc::c_ulonglong) as libc::c_longlong
                    as off_t;
                0 as libc::c_int
            }
        } else if if blksize < 0 as libc::c_int as libc::c_long {
            if ssize < 0 as libc::c_int as libc::c_long {
                if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulonglong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulonglong
                    } else {
                        (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                            .wrapping_mul(2 as libc::c_ulonglong)
                            .wrapping_add(1 as libc::c_ulonglong)
                    })
                        .wrapping_add(blksize as libc::c_ulonglong)
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                    < 0 as libc::c_int as libc::c_ulonglong
                {
                    ((ssize as libc::c_ulonglong)
                        < (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                            .wrapping_mul(2 as libc::c_ulonglong)
                            .wrapping_add(1 as libc::c_ulonglong)
                            .wrapping_div(blksize as libc::c_ulonglong)) as libc::c_int
                } else {
                    ((if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blksize
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (blksize
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < blksize) as libc::c_int
                    }) != 0
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) as libc::c_ulonglong)
                            .wrapping_add(
                                (9223372036854775807 as libc::c_longlong
                                    as libc::c_ulonglong)
                                    .wrapping_mul(2 as libc::c_ulonglong)
                                    .wrapping_add(1 as libc::c_ulonglong),
                            )
                            >> (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                            .wrapping_mul(2 as libc::c_ulonglong)
                            .wrapping_add(1 as libc::c_ulonglong)
                            .wrapping_div(-blksize as libc::c_ulonglong)
                    })
                        <= (-(1 as libc::c_int) as libc::c_long - ssize)
                            as libc::c_ulonglong) as libc::c_int
                }
            } else if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    blksize
                }) + 0 as libc::c_int as libc::c_long
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blksize
                    }) + 0 as libc::c_int as libc::c_long
                }) + 1 as libc::c_int as libc::c_long)
                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long
                    + 1 as libc::c_int as libc::c_long)
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blksize
                    }) + 0 as libc::c_int as libc::c_long
                }) + 0 as libc::c_int as libc::c_long
            }) < 0 as libc::c_int as libc::c_long
            {
                (((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    blksize
                }) + 0 as libc::c_int as libc::c_long)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + 0 as libc::c_int as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) + 0 as libc::c_int as libc::c_long
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_long)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blksize
                    }) + 0 as libc::c_int as libc::c_long) as libc::c_int
            }) != 0 && blksize == -(1 as libc::c_int) as libc::c_long
            {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    ssize
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((0 as libc::c_int as libc::c_long)
                        < ssize + 0 as libc::c_int as libc::c_long) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long) < ssize
                        && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                            < ssize - 1 as libc::c_int as libc::c_long) as libc::c_int
                }
            } else {
                (0 as libc::c_int as libc::c_long / blksize < ssize) as libc::c_int
            }
        } else if blksize == 0 as libc::c_int as libc::c_long {
            0 as libc::c_int
        } else if ssize < 0 as libc::c_int as libc::c_long {
            if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    ssize
                }) + 0 as libc::c_int as libc::c_long
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        ssize
                    }) + 0 as libc::c_int as libc::c_long
                }) + 1 as libc::c_int as libc::c_long)
                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long
                    + 1 as libc::c_int as libc::c_long)
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        ssize
                    }) + 0 as libc::c_int as libc::c_long
                }) + 0 as libc::c_int as libc::c_long
            }) < 0 as libc::c_int as libc::c_long
            {
                (((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    ssize
                }) + 0 as libc::c_int as libc::c_long)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) + 0 as libc::c_int as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                ssize
                            }) + 0 as libc::c_int as libc::c_long
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                ssize
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_long)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        ssize
                    }) + 0 as libc::c_int as libc::c_long) as libc::c_int
            }) != 0 && ssize == -(1 as libc::c_int) as libc::c_long
            {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    blksize
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((0 as libc::c_int as libc::c_long)
                        < blksize + 0 as libc::c_int as libc::c_long) as libc::c_int
                } else {
                    (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                        < blksize - 1 as libc::c_int as libc::c_long) as libc::c_int
                }
            } else {
                (0 as libc::c_int as libc::c_long / ssize < blksize) as libc::c_int
            }
        } else {
            ((9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                .wrapping_mul(2 as libc::c_ulonglong)
                .wrapping_add(1 as libc::c_ulonglong)
                .wrapping_div(blksize as libc::c_ulonglong) < ssize as libc::c_ulonglong)
                as libc::c_int
        } != 0
        {
            ssize = (ssize as libc::c_ulonglong)
                .wrapping_mul(blksize as libc::c_ulonglong) as off_t;
            1 as libc::c_int
        } else {
            ssize = (ssize as libc::c_ulonglong)
                .wrapping_mul(blksize as libc::c_ulonglong) as off_t;
            0 as libc::c_int
        } != 0
        {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"overflow in %ld * %ld byte blocks for file %s\0" as *const u8
                        as *const libc::c_char,
                ),
                ssize0,
                blksize,
                quotearg_style(shell_escape_always_quoting_style, fname),
            );
            return 0 as libc::c_int != 0;
        }
    }
    if rel_mode as u64 != 0 {
        let mut fsize: off_t = 0;
        if 0 as libc::c_int as libc::c_long <= rsize {
            fsize = rsize;
        } else if usable_st_size(&mut sb) {
            fsize = sb.st_size;
            if fsize < 0 as libc::c_int as libc::c_long {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"%s has unusable, apparently negative size\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, fname),
                );
                return 0 as libc::c_int != 0;
            }
        } else {
            fsize = lseek(fd, 0 as libc::c_int as __off_t, 2 as libc::c_int);
            if fsize < 0 as libc::c_int as libc::c_long {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"cannot get the size of %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, fname),
                );
                return 0 as libc::c_int != 0;
            }
        }
        if rel_mode as libc::c_uint == rm_min as libc::c_int as libc::c_uint {
            nsize = if fsize > ssize { fsize } else { ssize };
        } else if rel_mode as libc::c_uint == rm_max as libc::c_int as libc::c_uint {
            nsize = if fsize < ssize { fsize } else { ssize };
        } else if rel_mode as libc::c_uint == rm_rdn as libc::c_int as libc::c_uint {
            nsize = fsize - fsize % ssize;
        } else {
            if rel_mode as libc::c_uint == rm_rup as libc::c_int as libc::c_uint {
                let mut r: off_t = fsize % ssize;
                ssize = if r == 0 as libc::c_int as libc::c_long {
                    0 as libc::c_int as libc::c_long
                } else {
                    ssize - r
                };
            }
            let (fresh0, fresh1) = fsize.overflowing_add(ssize);
            *(&mut nsize as *mut off_t) = fresh0;
            if fresh1 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"overflow extending size of file %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, fname),
                );
                return 0 as libc::c_int != 0;
            }
        }
    } else {
        nsize = ssize;
    }
    if nsize < 0 as libc::c_int as libc::c_long {
        nsize = 0 as libc::c_int as off_t;
    }
    if ftruncate(fd, nsize) == -(1 as libc::c_int) {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(
                b"failed to truncate %s at %ld bytes\0" as *const u8
                    as *const libc::c_char,
            ),
            quotearg_style(shell_escape_always_quoting_style, fname),
            nsize,
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut got_size: bool = 0 as libc::c_int != 0;
    let mut errors: bool = 0 as libc::c_int != 0;
    let mut size: off_t = 0;
    let mut rsize: off_t = -(1 as libc::c_int) as off_t;
    let mut rel_mode: rel_mode_t = rm_abs;
    let mut c: libc::c_int = 0;
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut oflags: libc::c_int = 0;
    let mut fname: *const libc::c_char = 0 as *const libc::c_char;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    loop {
        c = getopt_long(
            argc,
            argv,
            b"cor:s:\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            99 => {
                no_create = 1 as libc::c_int != 0;
            }
            111 => {
                block_mode = 1 as libc::c_int != 0;
            }
            114 => {
                ref_file = optarg;
            }
            115 => {
                while *(*__ctype_b_loc())
                    .offset(to_uchar(*optarg) as libc::c_int as isize) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    optarg = optarg.offset(1);
                    optarg;
                }
                match *optarg as libc::c_int {
                    60 => {
                        rel_mode = rm_max;
                        optarg = optarg.offset(1);
                        optarg;
                    }
                    62 => {
                        rel_mode = rm_min;
                        optarg = optarg.offset(1);
                        optarg;
                    }
                    47 => {
                        rel_mode = rm_rdn;
                        optarg = optarg.offset(1);
                        optarg;
                    }
                    37 => {
                        rel_mode = rm_rup;
                        optarg = optarg.offset(1);
                        optarg;
                    }
                    _ => {}
                }
                while *(*__ctype_b_loc())
                    .offset(to_uchar(*optarg) as libc::c_int as isize) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    optarg = optarg.offset(1);
                    optarg;
                }
                if *optarg as libc::c_int == '+' as i32
                    || *optarg as libc::c_int == '-' as i32
                {
                    if rel_mode as u64 != 0 {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"multiple relative modifiers specified\0" as *const u8
                                    as *const libc::c_char,
                            ),
                        );
                        usage(1 as libc::c_int);
                    }
                    rel_mode = rm_rel;
                }
                size = xdectoimax(
                    optarg,
                    !if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                        -(1 as libc::c_int) as off_t
                    } else {
                        (((1 as libc::c_int as off_t)
                            << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    },
                    if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                        -(1 as libc::c_int) as off_t
                    } else {
                        (((1 as libc::c_int as off_t)
                            << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    },
                    b"EgGkKmMPtTYZ0\0" as *const u8 as *const libc::c_char,
                    gettext(b"Invalid number\0" as *const u8 as *const libc::c_char),
                    0 as libc::c_int,
                );
                if (rel_mode as libc::c_uint == rm_rup as libc::c_int as libc::c_uint
                    || rel_mode as libc::c_uint == rm_rdn as libc::c_int as libc::c_uint)
                    && size == 0 as libc::c_int as libc::c_long
                {
                    if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"division by zero\0" as *const u8 as *const libc::c_char,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"division by zero\0" as *const u8 as *const libc::c_char,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                got_size = 1 as libc::c_int != 0;
            }
            -130 => {
                usage(0 as libc::c_int);
            }
            -131 => {
                version_etc(
                    stdout,
                    b"truncate\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Padraig Brady\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    argv = argv.offset(optind as isize);
    argc -= optind;
    if ref_file.is_null() && !got_size {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"you must specify either %s or %s\0" as *const u8 as *const libc::c_char,
            ),
            quote_n(0 as libc::c_int, b"--size\0" as *const u8 as *const libc::c_char),
            quote_n(
                1 as libc::c_int,
                b"--reference\0" as *const u8 as *const libc::c_char,
            ),
        );
        usage(1 as libc::c_int);
    }
    if !ref_file.is_null() && got_size as libc::c_int != 0 && rel_mode as u64 == 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"you must specify a relative %s with %s\0" as *const u8
                    as *const libc::c_char,
            ),
            quote_n(0 as libc::c_int, b"--size\0" as *const u8 as *const libc::c_char),
            quote_n(
                1 as libc::c_int,
                b"--reference\0" as *const u8 as *const libc::c_char,
            ),
        );
        usage(1 as libc::c_int);
    }
    if block_mode as libc::c_int != 0 && !got_size {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"%s was specified but %s was not\0" as *const u8 as *const libc::c_char,
            ),
            quote_n(
                0 as libc::c_int,
                b"--io-blocks\0" as *const u8 as *const libc::c_char,
            ),
            quote_n(1 as libc::c_int, b"--size\0" as *const u8 as *const libc::c_char),
        );
        usage(1 as libc::c_int);
    }
    if argc < 1 as libc::c_int {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(b"missing file operand\0" as *const u8 as *const libc::c_char),
        );
        usage(1 as libc::c_int);
    }
    if !ref_file.is_null() {
        let mut sb: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        let mut file_size: off_t = -(1 as libc::c_int) as off_t;
        if stat(ref_file, &mut sb) != 0 as libc::c_int {
            if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(b"cannot stat %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, ref_file),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(b"cannot stat %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, ref_file),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if usable_st_size(&mut sb) {
            file_size = sb.st_size;
        } else {
            let mut ref_fd: libc::c_int = open(ref_file, 0 as libc::c_int);
            if 0 as libc::c_int <= ref_fd {
                let mut file_end: off_t = lseek(
                    ref_fd,
                    0 as libc::c_int as __off_t,
                    2 as libc::c_int,
                );
                let mut saved_errno: libc::c_int = *__errno_location();
                close(ref_fd);
                if 0 as libc::c_int as libc::c_long <= file_end {
                    file_size = file_end;
                } else {
                    *__errno_location() = saved_errno;
                }
            }
        }
        if file_size < 0 as libc::c_int as libc::c_long {
            if ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"cannot get the size of %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, ref_file),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"cannot get the size of %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, ref_file),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if !got_size {
            size = file_size;
        } else {
            rsize = file_size;
        }
    }
    oflags = 0o1 as libc::c_int
        | (if no_create as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            0o100 as libc::c_int
        }) | 0o4000 as libc::c_int;
    loop {
        let fresh2 = argv;
        argv = argv.offset(1);
        fname = *fresh2;
        if fname.is_null() {
            break;
        }
        fd = open(
            fname,
            oflags,
            0o400 as libc::c_int | 0o200 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int,
        );
        if fd == -(1 as libc::c_int) {
            if !(no_create as libc::c_int != 0
                && *__errno_location() == 2 as libc::c_int)
            {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"cannot open %s for writing\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, fname),
                );
                errors = 1 as libc::c_int != 0;
            }
        } else if fd != -(1 as libc::c_int) {
            errors = (errors as libc::c_int
                | !do_ftruncate(fd, fname, size, rsize, rel_mode) as libc::c_int) != 0;
            if close(fd) != 0 as libc::c_int {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(b"failed to close %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, fname),
                );
                errors = 1 as libc::c_int != 0;
            }
        }
    }
    return if errors as libc::c_int != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
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
