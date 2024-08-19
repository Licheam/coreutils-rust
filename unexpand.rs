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
pub mod fadvise;
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
pub mod xmalloc;
} // mod lib
pub mod src {
pub mod expand_common;
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
    fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    static mut Version: *const libc::c_char;
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
    fn xmalloc(s: size_t) -> *mut libc::c_void;
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
    static mut convert_entire_line: bool;
    static mut max_column_width: size_t;
    static mut exit_status: libc::c_int;
    fn add_tab_stop(tabval: uintmax_t);
    fn parse_tab_stops(stops: *const libc::c_char);
    fn get_next_tab_column(
        column: uintmax_t,
        tab_index: *mut size_t,
        last_tab: *mut bool,
    ) -> uintmax_t;
    fn finalize_tab_stops();
    fn set_file_list(file_list: *mut *mut libc::c_char);
    fn next_file(fp: *mut FILE) -> *mut FILE;
    fn cleanup_file_list_stdin();
    fn emit_tab_list_info();
}
pub type size_t = libc::c_ulong;
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type uintmax_t = __uintmax_t;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const CONVERT_FIRST_ONLY_OPTION: C2RustUnnamed_1 = 128;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
                b"Usage: %s [OPTION]... [FILE]...\n\0" as *const u8
                    as *const libc::c_char,
            ),
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"Convert blanks in each FILE to tabs, writing to standard output.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_stdin_note();
        emit_mandatory_arg_note();
        fputs_unlocked(
            gettext(
                b"  -a, --all        convert all blanks, instead of just initial blanks\n      --first-only  convert only leading sequences of blanks (overrides -a)\n  -t, --tabs=N     have tabs N characters apart instead of 8 (enables -a)\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_tab_list_info();
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
        emit_ancillary_info(b"unexpand\0" as *const u8 as *const libc::c_char);
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
unsafe extern "C" fn emit_stdin_note() {
    fputs_unlocked(
        gettext(
            b"\nWith no FILE, or when FILE is -, read standard input.\n\0" as *const u8
                as *const libc::c_char,
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
static mut longopts: [option; 6] = [
    {
        let mut init = option {
            name: b"tabs\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"all\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"first-only\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: CONVERT_FIRST_ONLY_OPTION as libc::c_int,
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
unsafe extern "C" fn unexpand() {
    let mut fp: *mut FILE = next_file(0 as *mut FILE);
    let mut pending_blank: *mut libc::c_char = 0 as *mut libc::c_char;
    if fp.is_null() {
        return;
    }
    pending_blank = xmalloc(max_column_width) as *mut libc::c_char;
    loop {
        let mut c: libc::c_int = 0;
        let mut convert: bool = 1 as libc::c_int != 0;
        let mut column: uintmax_t = 0 as libc::c_int as uintmax_t;
        let mut next_tab_column: uintmax_t = 0 as libc::c_int as uintmax_t;
        let mut tab_index: size_t = 0 as libc::c_int as size_t;
        let mut one_blank_before_tab_stop: bool = 0 as libc::c_int != 0;
        let mut prev_blank: bool = 1 as libc::c_int != 0;
        let mut pending: size_t = 0 as libc::c_int as size_t;
        let mut current_block_44: u64;
        loop {
            loop {
                c = getc_unlocked(fp);
                if !(c < 0 as libc::c_int
                    && {
                        fp = next_file(fp);
                        !fp.is_null()
                    })
                {
                    break;
                }
            }
            if convert {
                let mut blank: bool = *(*__ctype_b_loc()).offset(c as isize)
                    as libc::c_int
                    & _ISblank as libc::c_int as libc::c_ushort as libc::c_int != 0;
                if blank {
                    let mut last_tab: bool = false;
                    next_tab_column = get_next_tab_column(
                        column,
                        &mut tab_index,
                        &mut last_tab,
                    );
                    if last_tab {
                        convert = 0 as libc::c_int != 0;
                    }
                    if convert {
                        if next_tab_column < column {
                            if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong
                                != 0
                            {
                                error(
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    gettext(
                                        b"input line is too long\0" as *const u8
                                            as *const libc::c_char,
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
                                        b"input line is too long\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                );
                                if 0 as libc::c_int != 0 {} else {
                                    unreachable!();
                                };
                            };
                        }
                        if c == '\t' as i32 {
                            column = next_tab_column;
                            if pending != 0 {
                                *pending_blank
                                    .offset(
                                        0 as libc::c_int as isize,
                                    ) = '\t' as i32 as libc::c_char;
                            }
                            current_block_44 = 16203760046146113240;
                        } else {
                            column = column.wrapping_add(1);
                            column;
                            if !(prev_blank as libc::c_int != 0
                                && column == next_tab_column)
                            {
                                if column == next_tab_column {
                                    one_blank_before_tab_stop = 1 as libc::c_int != 0;
                                }
                                let fresh0 = pending;
                                pending = pending.wrapping_add(1);
                                *pending_blank.offset(fresh0 as isize) = c as libc::c_char;
                                prev_blank = 1 as libc::c_int != 0;
                                current_block_44 = 10879442775620481940;
                            } else {
                                c = '\t' as i32;
                                *pending_blank
                                    .offset(0 as libc::c_int as isize) = c as libc::c_char;
                                current_block_44 = 16203760046146113240;
                            }
                        }
                        match current_block_44 {
                            10879442775620481940 => {}
                            _ => {
                                pending = one_blank_before_tab_stop as size_t;
                                current_block_44 = 9853141518545631134;
                            }
                        }
                    } else {
                        current_block_44 = 9853141518545631134;
                    }
                } else {
                    if c == '\u{8}' as i32 {
                        column = (column as libc::c_ulong)
                            .wrapping_sub((column != 0) as libc::c_int as libc::c_ulong)
                            as uintmax_t as uintmax_t;
                        next_tab_column = column;
                        tab_index = (tab_index as libc::c_ulong)
                            .wrapping_sub(
                                (tab_index != 0) as libc::c_int as libc::c_ulong,
                            ) as size_t as size_t;
                    } else {
                        column = column.wrapping_add(1);
                        column;
                        if column == 0 {
                            if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong
                                != 0
                            {
                                error(
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    gettext(
                                        b"input line is too long\0" as *const u8
                                            as *const libc::c_char,
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
                                        b"input line is too long\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                );
                                if 0 as libc::c_int != 0 {} else {
                                    unreachable!();
                                };
                            };
                        }
                    }
                    current_block_44 = 9853141518545631134;
                }
                match current_block_44 {
                    10879442775620481940 => {}
                    _ => {
                        if pending != 0 {
                            if pending > 1 as libc::c_int as libc::c_ulong
                                && one_blank_before_tab_stop as libc::c_int != 0
                            {
                                *pending_blank
                                    .offset(
                                        0 as libc::c_int as isize,
                                    ) = '\t' as i32 as libc::c_char;
                            }
                            if fwrite_unlocked(
                                pending_blank as *const libc::c_void,
                                1 as libc::c_int as size_t,
                                pending,
                                stdout,
                            ) != pending
                            {
                                if ::core::mem::size_of::<C2RustUnnamed_3>()
                                    as libc::c_ulong != 0
                                {
                                    error(
                                        1 as libc::c_int,
                                        *__errno_location(),
                                        gettext(
                                            b"write error\0" as *const u8 as *const libc::c_char,
                                        ),
                                    );
                                    if 0 as libc::c_int != 0 {} else {
                                        unreachable!();
                                    };
                                } else {
                                    error(
                                        1 as libc::c_int,
                                        *__errno_location(),
                                        gettext(
                                            b"write error\0" as *const u8 as *const libc::c_char,
                                        ),
                                    );
                                    if 0 as libc::c_int != 0 {} else {
                                        unreachable!();
                                    };
                                };
                            }
                            pending = 0 as libc::c_int as size_t;
                            one_blank_before_tab_stop = 0 as libc::c_int != 0;
                        }
                        prev_blank = blank;
                        convert = (convert as libc::c_int
                            & (convert_entire_line as libc::c_int != 0
                                || blank as libc::c_int != 0) as libc::c_int) != 0;
                        current_block_44 = 15512526488502093901;
                    }
                }
            } else {
                current_block_44 = 15512526488502093901;
            }
            match current_block_44 {
                15512526488502093901 => {
                    if c < 0 as libc::c_int {
                        free(pending_blank as *mut libc::c_void);
                        return;
                    }
                    if putchar_unlocked(c) < 0 as libc::c_int {
                        if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong
                            != 0
                        {
                            error(
                                1 as libc::c_int,
                                *__errno_location(),
                                gettext(
                                    b"write error\0" as *const u8 as *const libc::c_char,
                                ),
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        } else {
                            error(
                                1 as libc::c_int,
                                *__errno_location(),
                                gettext(
                                    b"write error\0" as *const u8 as *const libc::c_char,
                                ),
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                }
                _ => {}
            }
            if !(c != '\n' as i32) {
                break;
            }
        }
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut have_tabval: bool = 0 as libc::c_int != 0;
    let mut tabval: uintmax_t = 0;
    let mut c: libc::c_int = 0;
    let mut convert_first_only: bool = 0 as libc::c_int != 0;
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
            b",0123456789at:\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            63 => {
                usage(1 as libc::c_int);
            }
            97 => {
                convert_entire_line = 1 as libc::c_int != 0;
            }
            116 => {
                convert_entire_line = 1 as libc::c_int != 0;
                parse_tab_stops(optarg);
            }
            128 => {
                convert_first_only = 1 as libc::c_int != 0;
            }
            44 => {
                if have_tabval {
                    add_tab_stop(tabval);
                }
                have_tabval = 0 as libc::c_int != 0;
            }
            -130 => {
                usage(0 as libc::c_int);
            }
            -131 => {
                version_etc(
                    stdout,
                    b"unexpand\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                if !have_tabval {
                    tabval = 0 as libc::c_int as uintmax_t;
                    have_tabval = 1 as libc::c_int != 0;
                }
                &mut tabval as *mut uintmax_t;
                if (if ::core::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong != 0 {
                    if (-(1 as libc::c_int) as uintmax_t)
                        .wrapping_div(10 as libc::c_int as libc::c_ulong) < tabval
                        || tabval
                            .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                            .wrapping_add((c - '0' as i32) as libc::c_ulong) < tabval
                    {
                        0 as libc::c_int
                    } else {
                        tabval = tabval
                            .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                            .wrapping_add((c - '0' as i32) as libc::c_ulong);
                        1 as libc::c_int
                    }
                } else {
                    if (-(1 as libc::c_int) as uintmax_t)
                        .wrapping_div(10 as libc::c_int as libc::c_ulong) < tabval
                        || tabval
                            .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                            .wrapping_add((c - '0' as i32) as libc::c_ulong) < tabval
                    {
                        0 as libc::c_int
                    } else {
                        tabval = tabval
                            .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                            .wrapping_add((c - '0' as i32) as libc::c_ulong);
                        1 as libc::c_int
                    }
                }) == 0
                {
                    if ::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"tab stop value is too large\0" as *const u8
                                    as *const libc::c_char,
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
                                b"tab stop value is too large\0" as *const u8
                                    as *const libc::c_char,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            }
        }
    }
    if convert_first_only {
        convert_entire_line = 0 as libc::c_int != 0;
    }
    if have_tabval {
        add_tab_stop(tabval);
    }
    finalize_tab_stops();
    set_file_list(
        if optind < argc {
            &mut *argv.offset(optind as isize)
        } else {
            0 as *mut *mut libc::c_char
        },
    );
    unexpand();
    cleanup_file_list_stdin();
    return exit_status;
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
