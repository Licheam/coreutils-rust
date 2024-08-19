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
pub mod freopen_safer;
pub mod fseeko;
pub mod hard_locale;
pub mod localcharset;
pub mod long_options;
pub mod mbrtowc;
pub mod progname;
pub mod quotearg;
pub mod readtokens;
pub mod setlocale_null;
pub mod version_etc;
pub mod version_etc_fsf;
pub mod xalloc_die;
pub mod xmalloc;
} // mod lib
pub mod src {
pub mod version;
} // mod src
} // mod src
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    static mut optind: libc::c_int;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    static mut Version: *const libc::c_char;
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
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn close_stdout();
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn parse_gnu_standard_options_only(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        scan_all: bool,
        usage_func: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        _: ...
    );
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn fadvise(fp: *mut FILE, advice: fadvice_t);
    fn init_tokenbuffer(tokenbuffer: *mut token_buffer);
    fn readtoken(
        stream: *mut FILE,
        delim: *const libc::c_char,
        n_delim: size_t,
        tokenbuffer: *mut token_buffer,
    ) -> size_t;
    fn freopen_safer(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut FILE,
    ) -> *mut FILE;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type fadvice_t = libc::c_uint;
pub const FADVISE_RANDOM: fadvice_t = 1;
pub const FADVISE_WILLNEED: fadvice_t = 3;
pub const FADVISE_DONTNEED: fadvice_t = 4;
pub const FADVISE_NOREUSE: fadvice_t = 5;
pub const FADVISE_SEQUENTIAL: fadvice_t = 2;
pub const FADVISE_NORMAL: fadvice_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tokenbuffer {
    pub size: size_t,
    pub buffer: *mut libc::c_char,
}
pub type token_buffer = tokenbuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct successor {
    pub suc: *mut item,
    pub next: *mut successor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct item {
    pub str_0: *const libc::c_char,
    pub left: *mut item,
    pub right: *mut item,
    pub balance: libc::c_int,
    pub count: size_t,
    pub qlink: *mut item,
    pub top: *mut successor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: libc::c_int,
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
                b"Usage: %s [OPTION] [FILE]\nWrite totally ordered list consistent with the partial ordering in FILE.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            program_name,
        );
        emit_stdin_note();
        fputs_unlocked(gettext(b"\n\0" as *const u8 as *const libc::c_char), stdout);
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
        emit_ancillary_info(b"tsort\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
static mut head: *mut item = 0 as *const item as *mut item;
static mut zeros: *mut item = 0 as *const item as *mut item;
static mut loop_0: *mut item = 0 as *const item as *mut item;
static mut n_strings: size_t = 0 as libc::c_int as size_t;
unsafe extern "C" fn new_item(mut str: *const libc::c_char) -> *mut item {
    let mut k: *mut item = xmalloc(::core::mem::size_of::<item>() as libc::c_ulong)
        as *mut item;
    (*k).str_0 = if !str.is_null() { xstrdup(str) } else { 0 as *mut libc::c_char };
    (*k).right = 0 as *mut item;
    (*k).left = (*k).right;
    (*k).balance = 0 as libc::c_int;
    (*k).count = 0 as libc::c_int as size_t;
    (*k).qlink = 0 as *mut item;
    (*k).top = 0 as *mut successor;
    return k;
}
unsafe extern "C" fn search_item(
    mut root: *mut item,
    mut str: *const libc::c_char,
) -> *mut item {
    let mut p: *mut item = 0 as *mut item;
    let mut q: *mut item = 0 as *mut item;
    let mut r: *mut item = 0 as *mut item;
    let mut s: *mut item = 0 as *mut item;
    let mut t: *mut item = 0 as *mut item;
    let mut a: libc::c_int = 0;
    if !root.is_null() {} else {
        __assert_fail(
            b"root\0" as *const u8 as *const libc::c_char,
            b"src/tsort.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"struct item *search_item(struct item *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_7612: {
        if !root.is_null() {} else {
            __assert_fail(
                b"root\0" as *const u8 as *const libc::c_char,
                b"src/tsort.c\0" as *const u8 as *const libc::c_char,
                132 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"struct item *search_item(struct item *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if ((*root).right).is_null() {
        (*root).right = new_item(str);
        return (*root).right;
    }
    t = root;
    p = (*root).right;
    s = p;
    loop {
        if !str.is_null() && !p.is_null() && !((*p).str_0).is_null() {} else {
            __assert_fail(
                b"str && p && p->str\0" as *const u8 as *const libc::c_char,
                b"src/tsort.c\0" as *const u8 as *const libc::c_char,
                146 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"struct item *search_item(struct item *, const char *)\0"))
                    .as_ptr(),
            );
        }
        'c_7531: {
            if !str.is_null() && !p.is_null() && !((*p).str_0).is_null() {} else {
                __assert_fail(
                    b"str && p && p->str\0" as *const u8 as *const libc::c_char,
                    b"src/tsort.c\0" as *const u8 as *const libc::c_char,
                    146 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 54],
                        &[libc::c_char; 54],
                    >(b"struct item *search_item(struct item *, const char *)\0"))
                        .as_ptr(),
                );
            }
        };
        a = strcmp(str, (*p).str_0);
        if a == 0 as libc::c_int {
            return p;
        }
        if a < 0 as libc::c_int {
            q = (*p).left;
        } else {
            q = (*p).right;
        }
        if q.is_null() {
            q = new_item(str);
            if a < 0 as libc::c_int {
                (*p).left = q;
            } else {
                (*p).right = q;
            }
            if !str.is_null() && !s.is_null() && !((*s).str_0).is_null()
                && !(strcmp(str, (*s).str_0) == 0 as libc::c_int)
            {} else {
                __assert_fail(
                    b"str && s && s->str && !STREQ (str, s->str)\0" as *const u8
                        as *const libc::c_char,
                    b"src/tsort.c\0" as *const u8 as *const libc::c_char,
                    169 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 54],
                        &[libc::c_char; 54],
                    >(b"struct item *search_item(struct item *, const char *)\0"))
                        .as_ptr(),
                );
            }
            'c_7387: {
                if !str.is_null() && !s.is_null() && !((*s).str_0).is_null()
                    && !(strcmp(str, (*s).str_0) == 0 as libc::c_int)
                {} else {
                    __assert_fail(
                        b"str && s && s->str && !STREQ (str, s->str)\0" as *const u8
                            as *const libc::c_char,
                        b"src/tsort.c\0" as *const u8 as *const libc::c_char,
                        169 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 54],
                            &[libc::c_char; 54],
                        >(b"struct item *search_item(struct item *, const char *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if strcmp(str, (*s).str_0) < 0 as libc::c_int {
                p = (*s).left;
                r = p;
                a = -(1 as libc::c_int);
            } else {
                p = (*s).right;
                r = p;
                a = 1 as libc::c_int;
            }
            while p != q {
                if !str.is_null() && !p.is_null() && !((*p).str_0).is_null()
                    && !(strcmp(str, (*p).str_0) == 0 as libc::c_int)
                {} else {
                    __assert_fail(
                        b"str && p && p->str && !STREQ (str, p->str)\0" as *const u8
                            as *const libc::c_char,
                        b"src/tsort.c\0" as *const u8 as *const libc::c_char,
                        183 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 54],
                            &[libc::c_char; 54],
                        >(b"struct item *search_item(struct item *, const char *)\0"))
                            .as_ptr(),
                    );
                }
                'c_7264: {
                    if !str.is_null() && !p.is_null() && !((*p).str_0).is_null()
                        && !(strcmp(str, (*p).str_0) == 0 as libc::c_int)
                    {} else {
                        __assert_fail(
                            b"str && p && p->str && !STREQ (str, p->str)\0" as *const u8
                                as *const libc::c_char,
                            b"src/tsort.c\0" as *const u8 as *const libc::c_char,
                            183 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 54],
                                &[libc::c_char; 54],
                            >(
                                b"struct item *search_item(struct item *, const char *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                if strcmp(str, (*p).str_0) < 0 as libc::c_int {
                    (*p).balance = -(1 as libc::c_int);
                    p = (*p).left;
                } else {
                    (*p).balance = 1 as libc::c_int;
                    p = (*p).right;
                }
            }
            if (*s).balance == 0 as libc::c_int || (*s).balance == -a {
                (*s).balance += a;
                return q;
            }
            if (*r).balance == a {
                p = r;
                if a < 0 as libc::c_int {
                    (*s).left = (*r).right;
                    (*r).right = s;
                } else {
                    (*s).right = (*r).left;
                    (*r).left = s;
                }
                (*r).balance = 0 as libc::c_int;
                (*s).balance = (*r).balance;
            } else {
                if a < 0 as libc::c_int {
                    p = (*r).right;
                    (*r).right = (*p).left;
                    (*p).left = r;
                    (*s).left = (*p).right;
                    (*p).right = s;
                } else {
                    p = (*r).left;
                    (*r).left = (*p).right;
                    (*p).right = r;
                    (*s).right = (*p).left;
                    (*p).left = s;
                }
                (*s).balance = 0 as libc::c_int;
                (*r).balance = 0 as libc::c_int;
                if (*p).balance == a {
                    (*s).balance = -a;
                } else if (*p).balance == -a {
                    (*r).balance = a;
                }
                (*p).balance = 0 as libc::c_int;
            }
            if s == (*t).right {
                (*t).right = p;
            } else {
                (*t).left = p;
            }
            return q;
        }
        if (*q).balance != 0 {
            t = p;
            s = q;
        }
        p = q;
    };
}
unsafe extern "C" fn record_relation(mut j: *mut item, mut k: *mut item) {
    let mut p: *mut successor = 0 as *mut successor;
    if !(strcmp((*j).str_0, (*k).str_0) == 0 as libc::c_int) {
        (*k).count = ((*k).count).wrapping_add(1);
        (*k).count;
        p = xmalloc(::core::mem::size_of::<successor>() as libc::c_ulong)
            as *mut successor;
        (*p).suc = k;
        (*p).next = (*j).top;
        (*j).top = p;
    }
}
unsafe extern "C" fn count_items(mut unused: *mut item) -> bool {
    n_strings = n_strings.wrapping_add(1);
    n_strings;
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn scan_zeros(mut k: *mut item) -> bool {
    if (*k).count == 0 as libc::c_int as libc::c_ulong && !((*k).str_0).is_null() {
        if head.is_null() {
            head = k;
        } else {
            (*zeros).qlink = k;
        }
        zeros = k;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn detect_loop(mut k: *mut item) -> bool {
    if (*k).count > 0 as libc::c_int as libc::c_ulong {
        if loop_0.is_null() {
            loop_0 = k;
        } else {
            let mut p: *mut *mut successor = &mut (*k).top;
            while !(*p).is_null() {
                if (**p).suc == loop_0 {
                    if !((*k).qlink).is_null() {
                        while !loop_0.is_null() {
                            let mut tmp: *mut item = (*loop_0).qlink;
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                b"%s\0" as *const u8 as *const libc::c_char,
                                (*loop_0).str_0,
                            );
                            if loop_0 == k {
                                (*(**p).suc).count = ((*(**p).suc).count).wrapping_sub(1);
                                (*(**p).suc).count;
                                *p = (**p).next;
                                break;
                            } else {
                                (*loop_0).qlink = 0 as *mut item;
                                loop_0 = tmp;
                            }
                        }
                        while !loop_0.is_null() {
                            let mut tmp_0: *mut item = (*loop_0).qlink;
                            (*loop_0).qlink = 0 as *mut item;
                            loop_0 = tmp_0;
                        }
                        return 1 as libc::c_int != 0;
                    } else {
                        (*k).qlink = loop_0;
                        loop_0 = k;
                        break;
                    }
                } else {
                    p = &mut (**p).next;
                }
            }
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn recurse_tree(
    mut root: *mut item,
    mut action: Option::<unsafe extern "C" fn(*mut item) -> bool>,
) -> bool {
    if ((*root).left).is_null() && ((*root).right).is_null() {
        return (Some(action.expect("non-null function pointer")))
            .expect("non-null function pointer")(root)
    } else {
        if !((*root).left).is_null() {
            if recurse_tree((*root).left, action) {
                return 1 as libc::c_int != 0;
            }
        }
        if (Some(action.expect("non-null function pointer")))
            .expect("non-null function pointer")(root)
        {
            return 1 as libc::c_int != 0;
        }
        if !((*root).right).is_null() {
            if recurse_tree((*root).right, action) {
                return 1 as libc::c_int != 0;
            }
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn walk_tree(
    mut root: *mut item,
    mut action: Option::<unsafe extern "C" fn(*mut item) -> bool>,
) {
    if !((*root).right).is_null() {
        recurse_tree((*root).right, action);
    }
}
unsafe extern "C" fn tsort(mut file: *const libc::c_char) -> bool {
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut root: *mut item = 0 as *mut item;
    let mut j: *mut item = 0 as *mut item;
    let mut k: *mut item = 0 as *mut item;
    let mut tokenbuffer: token_buffer = token_buffer {
        size: 0,
        buffer: 0 as *mut libc::c_char,
    };
    let mut is_stdin: bool = strcmp(file, b"-\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int;
    root = new_item(0 as *const libc::c_char);
    if !is_stdin
        && (freopen_safer(file, b"r\0" as *const u8 as *const libc::c_char, stdin))
            .is_null()
    {
        if ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    file,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    file,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    fadvise(stdin, FADVISE_SEQUENTIAL);
    init_tokenbuffer(&mut tokenbuffer);
    loop {
        let mut len: size_t = readtoken(
            stdin,
            b" \t\n\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            &mut tokenbuffer,
        );
        if len == -(1 as libc::c_int) as size_t {
            break;
        }
        if len != 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"len != 0\0" as *const u8 as *const libc::c_char,
                b"src/tsort.c\0" as *const u8 as *const libc::c_char,
                462 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"_Bool tsort(const char *)\0"))
                    .as_ptr(),
            );
        }
        'c_8491: {
            if len != 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"len != 0\0" as *const u8 as *const libc::c_char,
                    b"src/tsort.c\0" as *const u8 as *const libc::c_char,
                    462 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 26],
                        &[libc::c_char; 26],
                    >(b"_Bool tsort(const char *)\0"))
                        .as_ptr(),
                );
            }
        };
        k = search_item(root, tokenbuffer.buffer);
        if !j.is_null() {
            record_relation(j, k);
            k = 0 as *mut item;
        }
        j = k;
    }
    if !k.is_null() {
        if ::core::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"%s: input contains an odd number of tokens\0" as *const u8
                        as *const libc::c_char,
                ),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    file,
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
                    b"%s: input contains an odd number of tokens\0" as *const u8
                        as *const libc::c_char,
                ),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    file,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    walk_tree(root, Some(count_items as unsafe extern "C" fn(*mut item) -> bool));
    while n_strings > 0 as libc::c_int as libc::c_ulong {
        walk_tree(root, Some(scan_zeros as unsafe extern "C" fn(*mut item) -> bool));
        while !head.is_null() {
            let mut p: *mut successor = (*head).top;
            puts((*head).str_0);
            (*head).str_0 = 0 as *const libc::c_char;
            n_strings = n_strings.wrapping_sub(1);
            n_strings;
            while !p.is_null() {
                (*(*p).suc).count = ((*(*p).suc).count).wrapping_sub(1);
                (*(*p).suc).count;
                if (*(*p).suc).count == 0 as libc::c_int as libc::c_ulong {
                    (*zeros).qlink = (*p).suc;
                    zeros = (*p).suc;
                }
                p = (*p).next;
            }
            head = (*head).qlink;
        }
        if n_strings > 0 as libc::c_int as libc::c_ulong {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"%s: input contains a loop:\0" as *const u8 as *const libc::c_char,
                ),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    file,
                ),
            );
            ok = 0 as libc::c_int != 0;
            loop {
                walk_tree(
                    root,
                    Some(detect_loop as unsafe extern "C" fn(*mut item) -> bool),
                );
                if loop_0.is_null() {
                    break;
                }
            }
        }
    }
    if rpl_fclose(stdin) != 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                (if is_stdin as libc::c_int != 0 {
                    gettext(b"standard input\0" as *const u8 as *const libc::c_char)
                } else {
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        file,
                    )
                }),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                (if is_stdin as libc::c_int != 0 {
                    gettext(b"standard input\0" as *const u8 as *const libc::c_char)
                } else {
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        file,
                    )
                }),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    return ok;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut ok: bool = false;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    parse_gnu_standard_options_only(
        argc,
        argv,
        b"tsort\0" as *const u8 as *const libc::c_char,
        b"GNU coreutils\0" as *const u8 as *const libc::c_char,
        Version,
        1 as libc::c_int != 0,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> !>,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(Some(usage as unsafe extern "C" fn(libc::c_int) -> !)),
        b"Mark Kettenis\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    if (1 as libc::c_int) < argc - optind {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(b"extra operand %s\0" as *const u8 as *const libc::c_char),
            quote(*argv.offset((optind + 1 as libc::c_int) as isize)),
        );
        usage(1 as libc::c_int);
    }
    ok = tsort(
        if optind == argc {
            b"-\0" as *const u8 as *const libc::c_char
        } else {
            *argv.offset(optind as isize) as *const libc::c_char
        },
    );
    return if ok as libc::c_int != 0 { 0 as libc::c_int } else { 1 as libc::c_int };
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
