#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]


extern crate selinux_sys;
extern crate libc;
pub mod src {
pub mod lib {
pub mod basename;
pub mod basename_lgpl;
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
pub mod stripslash;
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
    static mut optarg: *mut libc::c_char;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optind: libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn set_program_name(argv0: *const libc::c_char);
    static mut program_name: *const libc::c_char;
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
    fn base_name(file: *const libc::c_char) -> *mut libc::c_char;
    fn strip_trailing_slashes(file: *mut libc::c_char) -> bool;
    fn close_stdout();
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type size_t = libc::c_ulong;
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
pub type C2RustUnnamed = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed = -131;
pub const GETOPT_HELP_CHAR: C2RustUnnamed = -130;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct infomap {
    pub program: *const libc::c_char,
    pub node: *const libc::c_char,
}
#[no_mangle]
pub extern "C" fn usage(mut status: libc::c_int) -> ! {
    if status != 0 as libc::c_int {
        unsafe {
            fprintf(
                stderr,
                gettext(
                    b"Try '%s --help' for more information.\n\0" as *const u8
                        as *const libc::c_char,
                ),
                program_name,
            );
        }
    } else {
        unsafe {
            printf(
                gettext(
                    b"Usage: %s NAME [SUFFIX]\n  or:  %s OPTION... NAME...\n\0" as *const u8
                        as *const libc::c_char,
                ),
                program_name,
                program_name,
            );
            fputs_unlocked(
                gettext(
                    b"Print NAME with any leading directory components removed.\nIf specified, also remove a trailing SUFFIX.\n\0"
                        as *const u8 as *const libc::c_char,
                ),
                stdout,
            );
            emit_mandatory_arg_note();
            fputs_unlocked(
                gettext(
                    b"  -a, --multiple       support multiple arguments and treat each as a NAME\n  -s, --suffix=SUFFIX  remove a trailing SUFFIX; implies -a\n  -z, --zero           end each output line with NUL, not newline\n\0"
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
            printf(
                gettext(
                    b"\nExamples:\n  %s /usr/bin/sort          -> \"sort\"\n  %s include/stdio.h .h     -> \"stdio\"\n  %s -s .h include/stdio.h  -> \"stdio\"\n  %s -a any/str1 any/str2   -> \"str1\" followed by \"str2\"\n\0"
                        as *const u8 as *const libc::c_char,
                ),
                program_name,
                program_name,
                program_name,
                program_name,
            );
            emit_ancillary_info(b"basename\0" as *const u8 as *const libc::c_char);
        }
    }
    unsafe {
        exit(status);
    }
    
}
#[inline]
extern "C" fn emit_ancillary_info(mut program: *const libc::c_char) {
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
    unsafe {
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
            name: b"multiple\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"suffix\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"zero\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'z' as i32,
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
extern "C" fn remove_suffix(
    mut name: &mut libc::c_char,
    mut suffix: & libc::c_char,
) {
    let mut np: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp: *const libc::c_char = 0 as *const libc::c_char;
    let name_ptr: *mut libc::c_char = name;
    let suffix_ptr: *const libc::c_char = suffix;
    unsafe {
        np = name_ptr.offset(strlen(name_ptr) as isize);
        sp = suffix_ptr.offset(strlen(suffix) as isize);
        while np > name && sp > suffix {

            np = np.offset(-1);
            sp = sp.offset(-1);
            
            if *np as libc::c_int != *sp as libc::c_int {
                return;
            }
        }
        if np > name {
            *np = '\0' as i32 as libc::c_char;
        }
    }
    
}
unsafe extern "C" fn perform_basename(
    mut string: *const libc::c_char,
    mut suffix: *const libc::c_char,
    mut use_nuls: bool,
) {
    let mut name: *mut libc::c_char = base_name(string);
    strip_trailing_slashes(name);
    if !suffix.is_null()
        && !(*name.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32)
        && 0 as libc::c_int == 0
    {
        let name_ref: &mut i8 = &mut *name;
        let suffix_ref: & i8 = &*suffix;
        remove_suffix(name_ref, suffix_ref);
    }
    fputs_unlocked(name, stdout);
    putchar_unlocked(
        if use_nuls as libc::c_int != 0 { '\0' as i32 } else { '\n' as i32 },
    );
    free(name as *mut libc::c_void);
}
fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut multiple_names: bool = 0 as libc::c_int != 0;
    let mut use_nuls: bool = 0 as libc::c_int != 0;
    let mut suffix: *const libc::c_char = 0 as *const libc::c_char;
    unsafe {
        set_program_name(*argv.offset(0 as libc::c_int as isize));
        setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
        bindtextdomain(
            b"coreutils\0" as *const u8 as *const libc::c_char,
            b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
        );
        textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
        atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    }
    loop {
        let mut c: libc::c_int;
        unsafe {
            let mut c_unsafe: libc::c_int = getopt_long(
                argc,
                argv,
                b"+as:z\0" as *const u8 as *const libc::c_char,
                longopts.as_ptr(),
                0 as *mut libc::c_int,
            );
            c = c_unsafe;
        }
        if c == -(1 as libc::c_int) {
            break;
        }
        let mut current_block_15: u64;
        match c {
            115 => {
                unsafe {
                    suffix = optarg;
                }
                current_block_15 = 1839871050172057018;
            }
            97 => {
                current_block_15 = 1839871050172057018;
            }
            122 => {
                use_nuls = 1 as libc::c_int != 0;
                current_block_15 = 4956146061682418353;
            }
            -130 => {
                usage(0 as libc::c_int);
            }
            -131 => {
                unsafe {
                    version_etc(
                        stdout,
                        b"basename\0" as *const u8 as *const libc::c_char,
                        b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                        Version,
                        b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                        0 as *mut libc::c_void as *mut libc::c_char,
                    );
                    exit(0 as libc::c_int);
                }
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
        match current_block_15 {
            1839871050172057018 => {
                multiple_names = 1 as libc::c_int != 0;
            }
            _ => {}
        }
    }
    unsafe {
        if argc < optind + 1 as libc::c_int {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(b"missing operand\0" as *const u8 as *const libc::c_char),
            );
            usage(1 as libc::c_int);
        }
        if !multiple_names && (optind + 2 as libc::c_int) < argc {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(b"extra operand %s\0" as *const u8 as *const libc::c_char),
                quote(*argv.offset((optind + 2 as libc::c_int) as isize)),
            );
            usage(1 as libc::c_int);
        }
        if multiple_names {
            while optind < argc {
                perform_basename(*argv.offset(optind as isize), suffix, use_nuls);
                optind += 1;
                optind;
            }
        } else {
            perform_basename(
                *argv.offset(optind as isize),
                if optind + 2 as libc::c_int == argc {
                    *argv.offset((optind + 1 as libc::c_int) as isize)
                } else {
                    0 as *mut libc::c_char
                },
                use_nuls,
            );
        }
    }
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
