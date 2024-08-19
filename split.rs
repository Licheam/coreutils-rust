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
pub mod basename_lgpl;
pub mod c_strcasecmp;
pub mod close_stream;
pub mod closeout;
pub mod dup_safer;
pub mod exitfail;
pub mod fclose;
pub mod fcntl;
pub mod fd_reopen;
pub mod fd_safer;
pub mod fflush;
pub mod fseeko;
pub mod full_write;
pub mod hard_locale;
pub mod localcharset;
pub mod mbrtowc;
pub mod open_safer;
pub mod progname;
pub mod quotearg;
pub mod safe_read;
pub mod safe_write;
pub mod setlocale_null;
pub mod sig2str;
pub mod umaxtostr;
pub mod version_etc;
pub mod version_etc_fsf;
pub mod xalloc_die;
pub mod xdectoumax;
pub mod xmalloc;
pub mod xstrtoumax;
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
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn clearerr_unlocked(__stream: *mut FILE);
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execl(
        __path: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fork() -> __pid_t;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn getpagesize() -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn rawmemchr(__s: *const libc::c_void, __c: libc::c_int) -> *mut libc::c_void;
    fn memrchr(
        __s: *const libc::c_void,
        __c: libc::c_int,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn abort() -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
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
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn x2nrealloc(p: *mut libc::c_void, pn: *mut size_t, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
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
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn fd_reopen(
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: mode_t,
    ) -> libc::c_int;
    fn open_safer(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> size_t;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn sig2str(_: libc::c_int, _: *mut libc::c_char) -> libc::c_int;
    fn xdectoumax(
        n_str: *const libc::c_char,
        min: uintmax_t,
        max: uintmax_t,
        suffixes: *const libc::c_char,
        err: *const libc::c_char,
        err_exit: libc::c_int,
    ) -> uintmax_t;
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
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
pub type mode_t = __mode_t;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
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
pub type uintmax_t = __uintmax_t;
pub type C2RustUnnamed_10 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_10 = 8;
pub const _ISpunct: C2RustUnnamed_10 = 4;
pub const _IScntrl: C2RustUnnamed_10 = 2;
pub const _ISblank: C2RustUnnamed_10 = 1;
pub const _ISgraph: C2RustUnnamed_10 = 32768;
pub const _ISprint: C2RustUnnamed_10 = 16384;
pub const _ISspace: C2RustUnnamed_10 = 8192;
pub const _ISxdigit: C2RustUnnamed_10 = 4096;
pub const _ISdigit: C2RustUnnamed_10 = 2048;
pub const _ISalpha: C2RustUnnamed_10 = 1024;
pub const _ISlower: C2RustUnnamed_10 = 512;
pub const _ISupper: C2RustUnnamed_10 = 256;
pub type C2RustUnnamed_11 = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed_11 = -131;
pub const GETOPT_HELP_CHAR: C2RustUnnamed_11 = -130;
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
pub type C2RustUnnamed_12 = libc::c_uint;
pub const IO_BUFSIZE: C2RustUnnamed_12 = 131072;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
pub type Split_type = libc::c_uint;
pub const type_rr: Split_type = 7;
pub const type_chunk_lines: Split_type = 6;
pub const type_chunk_bytes: Split_type = 5;
pub const type_digits: Split_type = 4;
pub const type_lines: Split_type = 3;
pub const type_byteslines: Split_type = 2;
pub const type_bytes: Split_type = 1;
pub const type_undef: Split_type = 0;
pub type C2RustUnnamed_13 = libc::c_uint;
pub const ADDITIONAL_SUFFIX_OPTION: C2RustUnnamed_13 = 131;
pub const IO_BLKSIZE_OPTION: C2RustUnnamed_13 = 130;
pub const FILTER_OPTION: C2RustUnnamed_13 = 129;
pub const VERBOSE_OPTION: C2RustUnnamed_13 = 128;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_33 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_36 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_37 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_38 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_39 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_40 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_41 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_42 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct of_info {
    pub of_name: *mut libc::c_char,
    pub ofd: libc::c_int,
    pub ofile: *mut FILE,
    pub opid: libc::c_int,
}
pub type of_t = of_info;
pub type C2RustUnnamed_43 = libc::c_int;
pub const OFD_APPEND: C2RustUnnamed_43 = -2;
pub const OFD_NEW: C2RustUnnamed_43 = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_44 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_45 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_46 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_47 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_48 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_49 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_50 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_51 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_52 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_53 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_54 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_55 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_56 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_57 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_58 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_59 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_60 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_61 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_62 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_63 {
    pub _gl_dummy: libc::c_int,
}
#[inline]
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn ptr_align(
    mut ptr: *const libc::c_void,
    mut alignment: size_t,
) -> *mut libc::c_void {
    let mut p0: *const libc::c_char = ptr as *const libc::c_char;
    let mut p1: *const libc::c_char = p0
        .offset(alignment as isize)
        .offset(-(1 as libc::c_int as isize));
    return p1.offset(-((p1 as size_t).wrapping_rem(alignment) as isize))
        as *mut libc::c_void;
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
unsafe extern "C" fn bad_cast(mut s: *const libc::c_char) -> *mut libc::c_char {
    return s as *mut libc::c_char;
}
#[inline]
unsafe extern "C" fn usable_st_size(mut sb: *const stat) -> bool {
    return (*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
        || (*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
        || ((*sb).st_mode).wrapping_sub((*sb).st_mode) != 0 || 0 as libc::c_int != 0;
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
                b"Usage: %s [OPTION]... [FILE [PREFIX]]\n\0" as *const u8
                    as *const libc::c_char,
            ),
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"Output pieces of FILE to PREFIXaa, PREFIXab, ...;\ndefault size is 1000 lines, and default PREFIX is 'x'.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_stdin_note();
        emit_mandatory_arg_note();
        fprintf(
            stdout,
            gettext(
                b"  -a, --suffix-length=N   generate suffixes of length N (default %d)\n      --additional-suffix=SUFFIX  append an additional SUFFIX to file names\n  -b, --bytes=SIZE        put SIZE bytes per output file\n  -C, --line-bytes=SIZE   put at most SIZE bytes of records per output file\n  -d                      use numeric suffixes starting at 0, not alphabetic\n      --numeric-suffixes[=FROM]  same as -d, but allow setting the start value\n  -x                      use hex suffixes starting at 0, not alphabetic\n      --hex-suffixes[=FROM]  same as -x, but allow setting the start value\n  -e, --elide-empty-files  do not generate empty output files with '-n'\n      --filter=COMMAND    write to shell COMMAND; file name is $FILE\n  -l, --lines=NUMBER      put NUMBER lines/records per output file\n  -n, --number=CHUNKS     generate CHUNKS output files; see explanation below\n  -t, --separator=SEP     use SEP instead of newline as the record separator;\n                            '\\0' (zero) specifies the NUL character\n  -u, --unbuffered        immediately copy input to output with '-n r/...'\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            2 as libc::c_int,
        );
        fputs_unlocked(
            gettext(
                b"      --verbose           print a diagnostic just before each\n                            output file is opened\n\0"
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
                b"\nCHUNKS may be:\n  N       split into N files based on size of input\n  K/N     output Kth of N to stdout\n  l/N     split into N files without splitting lines/records\n  l/K/N   output Kth of N to stdout without splitting lines/records\n  r/N     like 'l' but use round robin distribution\n  r/K/N   likewise but only output Kth of N to stdout\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_ancillary_info(b"split\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
#[inline]
unsafe extern "C" fn io_blksize(mut sb: stat) -> size_t {
    return (if IO_BUFSIZE as libc::c_int as libc::c_long
        > (if (0 as libc::c_int as libc::c_long) < sb.st_blksize
            && sb.st_blksize as libc::c_ulong
                <= (-(1 as libc::c_int) as size_t)
                    .wrapping_div(8 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            sb.st_blksize
        } else {
            512 as libc::c_int as libc::c_long
        })
    {
        IO_BUFSIZE as libc::c_int as libc::c_long
    } else if (0 as libc::c_int as libc::c_long) < sb.st_blksize
        && sb.st_blksize as libc::c_ulong
            <= (-(1 as libc::c_int) as size_t)
                .wrapping_div(8 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        sb.st_blksize
    } else {
        512 as libc::c_int as libc::c_long
    }) as size_t;
}
#[inline]
unsafe extern "C" fn __gl_setmode(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn set_binary_mode(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    return __gl_setmode(fd, mode);
}
#[inline]
unsafe extern "C" fn xset_binary_mode_error() {}
#[inline]
unsafe extern "C" fn xset_binary_mode(mut fd: libc::c_int, mut mode: libc::c_int) {
    if set_binary_mode(fd, mode) < 0 as libc::c_int {
        xset_binary_mode_error();
    }
}
static mut filter_command: *const libc::c_char = 0 as *const libc::c_char;
static mut filter_pid: libc::c_int = 0;
static mut open_pipes: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut open_pipes_alloc: size_t = 0;
static mut n_open_pipes: size_t = 0;
static mut oldblocked: sigset_t = sigset_t { __val: [0; 16] };
static mut newblocked: sigset_t = sigset_t { __val: [0; 16] };
static mut outbase: *const libc::c_char = 0 as *const libc::c_char;
static mut outfile: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut outfile_mid: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut suffix_auto: bool = 1 as libc::c_int != 0;
static mut suffix_length: size_t = 0;
static mut suffix_alphabet: *const libc::c_char = b"abcdefghijklmnopqrstuvwxyz\0"
    as *const u8 as *const libc::c_char;
static mut numeric_suffix_start: *const libc::c_char = 0 as *const libc::c_char;
static mut additional_suffix: *const libc::c_char = 0 as *const libc::c_char;
static mut infile: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut in_stat_buf: stat = stat {
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
static mut output_desc: libc::c_int = -(1 as libc::c_int);
static mut verbose: bool = false;
static mut elide_empty_files: bool = false;
static mut unbuffered: bool = false;
static mut eolchar: libc::c_int = -(1 as libc::c_int);
static mut longopts: [option; 17] = [
    {
        let mut init = option {
            name: b"bytes\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"lines\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"line-bytes\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'C' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"number\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"elide-empty-files\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'e' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"unbuffered\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'u' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"suffix-length\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"additional-suffix\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ADDITIONAL_SUFFIX_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"numeric-suffixes\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"hex-suffixes\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'x' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"filter\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: FILTER_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: VERBOSE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"separator\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"-io-blksize\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: IO_BLKSIZE_OPTION as libc::c_int,
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
#[inline]
unsafe extern "C" fn ignorable(mut err: libc::c_int) -> bool {
    return !filter_command.is_null() && err == 32 as libc::c_int;
}
unsafe extern "C" fn set_suffix_length(
    mut n_units: uintmax_t,
    mut split_type: Split_type,
) {
    let mut suffix_length_needed: uintmax_t = 0 as libc::c_int as uintmax_t;
    if !numeric_suffix_start.is_null() {
        suffix_auto = 0 as libc::c_int != 0;
    }
    if split_type as libc::c_uint == type_chunk_bytes as libc::c_int as libc::c_uint
        || split_type as libc::c_uint == type_chunk_lines as libc::c_int as libc::c_uint
        || split_type as libc::c_uint == type_rr as libc::c_int as libc::c_uint
    {
        let mut n_units_end: uintmax_t = n_units
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        if !numeric_suffix_start.is_null() {
            let mut n_start: uintmax_t = 0;
            let mut e: strtol_error = xstrtoumax(
                numeric_suffix_start,
                0 as *mut *mut libc::c_char,
                10 as libc::c_int,
                &mut n_start,
                b"\0" as *const u8 as *const libc::c_char,
            );
            if e as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint
                && n_start
                    <= (18446744073709551615 as libc::c_ulong).wrapping_sub(n_units)
            {
                if n_start < n_units {
                    n_units_end = (n_units_end as libc::c_ulong).wrapping_add(n_start)
                        as uintmax_t as uintmax_t;
                }
            }
        }
        let mut alphabet_len: size_t = strlen(suffix_alphabet);
        loop {
            suffix_length_needed = suffix_length_needed.wrapping_add(1);
            suffix_length_needed;
            n_units_end = (n_units_end as libc::c_ulong).wrapping_div(alphabet_len)
                as uintmax_t as uintmax_t;
            if !(n_units_end != 0) {
                break;
            }
        }
        suffix_auto = 0 as libc::c_int != 0;
    }
    if suffix_length != 0 {
        if suffix_length < suffix_length_needed {
            if ::core::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"the suffix length needs to be at least %lu\0" as *const u8
                            as *const libc::c_char,
                    ),
                    suffix_length_needed,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"the suffix length needs to be at least %lu\0" as *const u8
                            as *const libc::c_char,
                    ),
                    suffix_length_needed,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        suffix_auto = 0 as libc::c_int != 0;
        return;
    } else {
        suffix_length = if 2 as libc::c_int as libc::c_ulong > suffix_length_needed {
            2 as libc::c_int as libc::c_ulong
        } else {
            suffix_length_needed
        };
    };
}
unsafe extern "C" fn input_file_size(
    mut fd: libc::c_int,
    mut st: *const stat,
    mut buf: *mut libc::c_char,
    mut bufsize: size_t,
) -> off_t {
    let mut cur: off_t = lseek(fd, 0 as libc::c_int as __off_t, 1 as libc::c_int);
    if cur < 0 as libc::c_int as libc::c_long {
        if *__errno_location() == 29 as libc::c_int {
            *__errno_location() = 0 as libc::c_int;
        }
        return -(1 as libc::c_int) as off_t;
    }
    let mut size: off_t = 0 as libc::c_int as off_t;
    loop {
        let mut n_read: size_t = safe_read(
            fd,
            buf.offset(size as isize) as *mut libc::c_void,
            bufsize.wrapping_sub(size as libc::c_ulong),
        );
        if n_read == 0 as libc::c_int as libc::c_ulong {
            return size;
        }
        if n_read == -(1 as libc::c_int) as size_t {
            return -(1 as libc::c_int) as off_t;
        }
        size = (size as libc::c_ulong).wrapping_add(n_read) as off_t as off_t;
        if !((size as libc::c_ulong) < bufsize) {
            break;
        }
    }
    if (*st).st_size == 0 as libc::c_int as libc::c_long {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int) as off_t;
    }
    cur += size;
    let mut end: off_t = 0;
    if usable_st_size(st) as libc::c_int != 0 && cur <= (*st).st_size {
        end = (*st).st_size;
    } else {
        end = lseek(fd, 0 as libc::c_int as __off_t, 2 as libc::c_int);
        if end < 0 as libc::c_int as libc::c_long {
            return -(1 as libc::c_int) as off_t;
        }
        if end != cur {
            if lseek(fd, cur, 0 as libc::c_int) < 0 as libc::c_int as libc::c_long {
                return -(1 as libc::c_int) as off_t;
            }
            if end < cur {
                end = cur;
            }
        }
    }
    size += end - cur;
    if size
        == (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
            -(1 as libc::c_int) as off_t
        } else {
            (((1 as libc::c_int as off_t)
                << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        })
    {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int) as off_t;
    }
    return size;
}
unsafe extern "C" fn next_file_name() {
    static mut sufindex: *mut size_t = 0 as *const size_t as *mut size_t;
    static mut outbase_length: size_t = 0;
    static mut outfile_length: size_t = 0;
    static mut addsuf_length: size_t = 0;
    let mut widen: bool = false;
    let mut current_block_36: u64;
    if outfile.is_null() {
        widen = false;
        current_block_36 = 3024234852907405291;
    } else {
        let mut i_0: size_t = suffix_length;
        loop {
            let fresh1 = i_0;
            i_0 = i_0.wrapping_sub(1);
            if !(fresh1 != 0 as libc::c_int as libc::c_ulong) {
                current_block_36 = 11913429853522160501;
                break;
            }
            let ref mut fresh2 = *sufindex.offset(i_0 as isize);
            *fresh2 = (*fresh2).wrapping_add(1);
            *fresh2;
            if suffix_auto as libc::c_int != 0
                && i_0 == 0 as libc::c_int as libc::c_ulong
                && *suffix_alphabet
                    .offset(
                        (*sufindex.offset(0 as libc::c_int as isize))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) == 0
            {
                current_block_36 = 3024234852907405291;
                break;
            }
            *outfile_mid
                .offset(
                    i_0 as isize,
                ) = *suffix_alphabet.offset(*sufindex.offset(i_0 as isize) as isize);
            if *outfile_mid.offset(i_0 as isize) != 0 {
                return;
            }
            *sufindex.offset(i_0 as isize) = 0 as libc::c_int as size_t;
            *outfile_mid
                .offset(
                    i_0 as isize,
                ) = *suffix_alphabet.offset(*sufindex.offset(i_0 as isize) as isize);
        }
        match current_block_36 {
            3024234852907405291 => {}
            _ => {
                if ::core::mem::size_of::<C2RustUnnamed_15>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"output file suffixes exhausted\0" as *const u8
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
                            b"output file suffixes exhausted\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
                current_block_36 = 18153031941552419006;
            }
        }
    }
    match current_block_36 {
        3024234852907405291 => {
            widen = outfile_length != 0;
            if !widen {
                outbase_length = strlen(outbase);
                addsuf_length = if !additional_suffix.is_null() {
                    strlen(additional_suffix)
                } else {
                    0 as libc::c_int as libc::c_ulong
                };
                outfile_length = outbase_length
                    .wrapping_add(suffix_length)
                    .wrapping_add(addsuf_length);
            } else {
                outfile_length = (outfile_length as libc::c_ulong)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                suffix_length = suffix_length.wrapping_add(1);
                suffix_length;
            }
            if outfile_length.wrapping_add(1 as libc::c_int as libc::c_ulong)
                < outbase_length
            {
                xalloc_die();
            }
            outfile = xrealloc(
                outfile as *mut libc::c_void,
                outfile_length.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            if !widen {
                memcpy(
                    outfile as *mut libc::c_void,
                    outbase as *const libc::c_void,
                    outbase_length,
                );
            } else {
                *outfile
                    .offset(
                        outbase_length as isize,
                    ) = *suffix_alphabet
                    .offset(*sufindex.offset(0 as libc::c_int as isize) as isize);
                outbase_length = outbase_length.wrapping_add(1);
                outbase_length;
            }
            outfile_mid = outfile.offset(outbase_length as isize);
            memset(
                outfile_mid as *mut libc::c_void,
                *suffix_alphabet.offset(0 as libc::c_int as isize) as libc::c_int,
                suffix_length,
            );
            if !additional_suffix.is_null() {
                memcpy(
                    outfile_mid.offset(suffix_length as isize) as *mut libc::c_void,
                    additional_suffix as *const libc::c_void,
                    addsuf_length,
                );
            }
            *outfile.offset(outfile_length as isize) = 0 as libc::c_int as libc::c_char;
            free(sufindex as *mut libc::c_void);
            sufindex = xcalloc(
                suffix_length,
                ::core::mem::size_of::<size_t>() as libc::c_ulong,
            ) as *mut size_t;
            if !numeric_suffix_start.is_null() {
                if !widen {} else {
                    __assert_fail(
                        b"! widen\0" as *const u8 as *const libc::c_char,
                        b"src/split.c\0" as *const u8 as *const libc::c_char,
                        408 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 26],
                            &[libc::c_char; 26],
                        >(b"void next_file_name(void)\0"))
                            .as_ptr(),
                    );
                }
                'c_8082: {
                    if !widen {} else {
                        __assert_fail(
                            b"! widen\0" as *const u8 as *const libc::c_char,
                            b"src/split.c\0" as *const u8 as *const libc::c_char,
                            408 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 26],
                                &[libc::c_char; 26],
                            >(b"void next_file_name(void)\0"))
                                .as_ptr(),
                        );
                    }
                };
                let mut i: size_t = strlen(numeric_suffix_start);
                memcpy(
                    outfile_mid.offset(suffix_length as isize).offset(-(i as isize))
                        as *mut libc::c_void,
                    numeric_suffix_start as *const libc::c_void,
                    i,
                );
                let mut sufindex_end: *mut size_t = sufindex
                    .offset(suffix_length as isize);
                loop {
                    let fresh0 = i;
                    i = i.wrapping_sub(1);
                    if !(fresh0 != 0 as libc::c_int as libc::c_ulong) {
                        break;
                    }
                    sufindex_end = sufindex_end.offset(-1);
                    *sufindex_end = (*numeric_suffix_start.offset(i as isize)
                        as libc::c_int - '0' as i32) as size_t;
                }
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn create(mut name: *const libc::c_char) -> libc::c_int {
    if filter_command.is_null() {
        if verbose {
            fprintf(
                stdout,
                gettext(b"creating file %s\n\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, name),
            );
        }
        let mut fd: libc::c_int = open_safer(
            name,
            0o1 as libc::c_int | 0o100 as libc::c_int | 0 as libc::c_int,
            0o400 as libc::c_int | 0o200 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int,
        );
        if fd < 0 as libc::c_int {
            return fd;
        }
        let mut out_stat_buf: stat = stat {
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
        if fstat(fd, &mut out_stat_buf) != 0 as libc::c_int {
            if ::core::mem::size_of::<C2RustUnnamed_27>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(b"failed to stat %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, name),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(b"failed to stat %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, name),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if in_stat_buf.st_ino == out_stat_buf.st_ino
            && in_stat_buf.st_dev == out_stat_buf.st_dev
        {
            if ::core::mem::size_of::<C2RustUnnamed_26>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"%s would overwrite input; aborting\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, name),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"%s would overwrite input; aborting\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, name),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if ftruncate(fd, 0 as libc::c_int as __off_t) != 0 as libc::c_int
            && (out_stat_buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint
                || (out_stat_buf.st_mode).wrapping_sub(out_stat_buf.st_mode) != 0)
        {
            if ::core::mem::size_of::<C2RustUnnamed_25>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"%s: error truncating\0" as *const u8 as *const libc::c_char,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        name,
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
                        b"%s: error truncating\0" as *const u8 as *const libc::c_char,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        name,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        return fd;
    } else {
        let mut fd_pair: [libc::c_int; 2] = [0; 2];
        let mut child_pid: pid_t = 0;
        let mut shell_prog: *const libc::c_char = getenv(
            b"SHELL\0" as *const u8 as *const libc::c_char,
        );
        if shell_prog.is_null() {
            shell_prog = b"/bin/sh\0" as *const u8 as *const libc::c_char;
        }
        if setenv(b"FILE\0" as *const u8 as *const libc::c_char, name, 1 as libc::c_int)
            != 0 as libc::c_int
        {
            if ::core::mem::size_of::<C2RustUnnamed_24>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"failed to set FILE environment variable\0" as *const u8
                            as *const libc::c_char,
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
                        b"failed to set FILE environment variable\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if verbose {
            fprintf(
                stdout,
                gettext(
                    b"executing with FILE=%s\n\0" as *const u8 as *const libc::c_char,
                ),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    name,
                ),
            );
        }
        if pipe(fd_pair.as_mut_ptr()) != 0 as libc::c_int {
            if ::core::mem::size_of::<C2RustUnnamed_23>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"failed to create pipe\0" as *const u8 as *const libc::c_char,
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
                        b"failed to create pipe\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        child_pid = fork();
        if child_pid == 0 as libc::c_int {
            let mut j: libc::c_int = 0;
            j = 0 as libc::c_int;
            while (j as libc::c_ulong) < n_open_pipes {
                if close(*open_pipes.offset(j as isize)) != 0 as libc::c_int {
                    if ::core::mem::size_of::<C2RustUnnamed_22>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"closing prior pipe\0" as *const u8 as *const libc::c_char,
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
                                b"closing prior pipe\0" as *const u8 as *const libc::c_char,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                j += 1;
                j;
            }
            if close(fd_pair[1 as libc::c_int as usize]) != 0 {
                if ::core::mem::size_of::<C2RustUnnamed_21>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        gettext(
                            b"closing output pipe\0" as *const u8 as *const libc::c_char,
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
                            b"closing output pipe\0" as *const u8 as *const libc::c_char,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            if fd_pair[0 as libc::c_int as usize] != 0 as libc::c_int {
                if dup2(fd_pair[0 as libc::c_int as usize], 0 as libc::c_int)
                    != 0 as libc::c_int
                {
                    if ::core::mem::size_of::<C2RustUnnamed_20>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"moving input pipe\0" as *const u8 as *const libc::c_char,
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
                                b"moving input pipe\0" as *const u8 as *const libc::c_char,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                if close(fd_pair[0 as libc::c_int as usize]) != 0 as libc::c_int {
                    if ::core::mem::size_of::<C2RustUnnamed_19>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"closing input pipe\0" as *const u8 as *const libc::c_char,
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
                                b"closing input pipe\0" as *const u8 as *const libc::c_char,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            }
            sigprocmask(2 as libc::c_int, &mut oldblocked, 0 as *mut sigset_t);
            execl(
                shell_prog,
                last_component(shell_prog),
                b"-c\0" as *const u8 as *const libc::c_char,
                filter_command,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
            if ::core::mem::size_of::<C2RustUnnamed_18>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"failed to run command: \"%s -c %s\"\0" as *const u8
                            as *const libc::c_char,
                    ),
                    shell_prog,
                    filter_command,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"failed to run command: \"%s -c %s\"\0" as *const u8
                            as *const libc::c_char,
                    ),
                    shell_prog,
                    filter_command,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if child_pid == -(1 as libc::c_int) {
            if ::core::mem::size_of::<C2RustUnnamed_17>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"fork system call failed\0" as *const u8 as *const libc::c_char,
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
                        b"fork system call failed\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if close(fd_pair[0 as libc::c_int as usize]) != 0 as libc::c_int {
            if ::core::mem::size_of::<C2RustUnnamed_16>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"failed to close input pipe\0" as *const u8
                            as *const libc::c_char,
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
                        b"failed to close input pipe\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        filter_pid = child_pid;
        if n_open_pipes == open_pipes_alloc {
            open_pipes = x2nrealloc(
                open_pipes as *mut libc::c_void,
                &mut open_pipes_alloc,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ) as *mut libc::c_int;
        }
        let fresh3 = n_open_pipes;
        n_open_pipes = n_open_pipes.wrapping_add(1);
        *open_pipes.offset(fresh3 as isize) = fd_pair[1 as libc::c_int as usize];
        return fd_pair[1 as libc::c_int as usize];
    };
}
unsafe extern "C" fn closeout(
    mut fp: *mut FILE,
    mut fd: libc::c_int,
    mut pid: pid_t,
    mut name: *const libc::c_char,
) {
    if !fp.is_null() && rpl_fclose(fp) != 0 as libc::c_int
        && !ignorable(*__errno_location())
    {
        if ::core::mem::size_of::<C2RustUnnamed_31>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    name,
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
                    name,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if fd >= 0 as libc::c_int {
        if fp.is_null() && close(fd) < 0 as libc::c_int {
            if ::core::mem::size_of::<C2RustUnnamed_30>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        name,
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
                        name,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while (j as libc::c_ulong) < n_open_pipes {
            if *open_pipes.offset(j as isize) == fd {
                n_open_pipes = n_open_pipes.wrapping_sub(1);
                *open_pipes
                    .offset(j as isize) = *open_pipes.offset(n_open_pipes as isize);
                break;
            } else {
                j += 1;
                j;
            }
        }
    }
    if pid > 0 as libc::c_int {
        let mut wstatus: libc::c_int = 0 as libc::c_int;
        if waitpid(pid, &mut wstatus, 0 as libc::c_int) == -(1 as libc::c_int)
            && *__errno_location() != 10 as libc::c_int
        {
            if ::core::mem::size_of::<C2RustUnnamed_29>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"waiting for child process\0" as *const u8
                            as *const libc::c_char,
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
                        b"waiting for child process\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if ((wstatus & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
            as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
        {
            let mut sig: libc::c_int = wstatus & 0x7f as libc::c_int;
            if sig != 13 as libc::c_int {
                let mut signame: [libc::c_char; 19] = [0; 19];
                if sig2str(sig, signame.as_mut_ptr()) != 0 as libc::c_int {
                    sprintf(
                        signame.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        sig,
                    );
                }
                error(
                    sig + 128 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"with FILE=%s, signal %s from command: %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        name,
                    ),
                    signame.as_mut_ptr(),
                    filter_command,
                );
            }
        } else if wstatus & 0x7f as libc::c_int == 0 as libc::c_int {
            let mut ex: libc::c_int = (wstatus & 0xff00 as libc::c_int)
                >> 8 as libc::c_int;
            if ex != 0 as libc::c_int {
                error(
                    ex,
                    0 as libc::c_int,
                    gettext(
                        b"with FILE=%s, exit %d from command: %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        name,
                    ),
                    ex,
                    filter_command,
                );
            }
        } else {
            if ::core::mem::size_of::<C2RustUnnamed_28>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"unknown status from command (0x%X)\0" as *const u8
                            as *const libc::c_char,
                    ),
                    (wstatus as libc::c_uint).wrapping_add(0 as libc::c_uint),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"unknown status from command (0x%X)\0" as *const u8
                            as *const libc::c_char,
                    ),
                    (wstatus as libc::c_uint).wrapping_add(0 as libc::c_uint),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
}
unsafe extern "C" fn cwrite(
    mut new_file_flag: bool,
    mut bp: *const libc::c_char,
    mut bytes: size_t,
) -> bool {
    if new_file_flag {
        if bp.is_null() && bytes == 0 as libc::c_int as libc::c_ulong
            && elide_empty_files as libc::c_int != 0
        {
            return 1 as libc::c_int != 0;
        }
        closeout(0 as *mut FILE, output_desc, filter_pid, outfile);
        next_file_name();
        output_desc = create(outfile);
        if output_desc < 0 as libc::c_int {
            if ::core::mem::size_of::<C2RustUnnamed_33>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        outfile,
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
                        outfile,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    if full_write(output_desc, bp as *const libc::c_void, bytes) == bytes {
        return 1 as libc::c_int != 0
    } else {
        if !ignorable(*__errno_location()) {
            if ::core::mem::size_of::<C2RustUnnamed_32>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        outfile,
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
                        outfile,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        return 0 as libc::c_int != 0;
    };
}
unsafe extern "C" fn bytes_split(
    mut n_bytes: uintmax_t,
    mut buf: *mut libc::c_char,
    mut bufsize: size_t,
    mut initial_read: size_t,
    mut max_files: uintmax_t,
) {
    let mut n_read: size_t = 0;
    let mut new_file_flag: bool = 1 as libc::c_int != 0;
    let mut filter_ok: bool = 1 as libc::c_int != 0;
    let mut to_write: uintmax_t = n_bytes;
    let mut opened: uintmax_t = 0 as libc::c_int as uintmax_t;
    let mut eof: bool = false;
    loop {
        if initial_read != 18446744073709551615 as libc::c_ulong {
            n_read = initial_read;
            initial_read = 18446744073709551615 as libc::c_ulong;
            eof = n_read < bufsize;
        } else {
            if !filter_ok
                && lseek(0 as libc::c_int, to_write as __off_t, 1 as libc::c_int)
                    != -(1 as libc::c_int) as libc::c_long
            {
                to_write = n_bytes;
                new_file_flag = 1 as libc::c_int != 0;
            }
            n_read = safe_read(0 as libc::c_int, buf as *mut libc::c_void, bufsize);
            if n_read == -(1 as libc::c_int) as size_t {
                if ::core::mem::size_of::<C2RustUnnamed_34>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            infile,
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
                            infile,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            eof = n_read == 0 as libc::c_int as libc::c_ulong;
        }
        let mut bp_out: *mut libc::c_char = buf;
        while to_write <= n_read {
            if filter_ok as libc::c_int != 0 || new_file_flag as libc::c_int != 0 {
                filter_ok = cwrite(new_file_flag, bp_out, to_write);
            }
            opened = (opened as libc::c_ulong)
                .wrapping_add(new_file_flag as libc::c_ulong) as uintmax_t as uintmax_t;
            new_file_flag = max_files == 0 || opened < max_files;
            if !filter_ok && !new_file_flag {
                n_read = 0 as libc::c_int as size_t;
                eof = 1 as libc::c_int != 0;
                break;
            } else {
                bp_out = bp_out.offset(to_write as isize);
                n_read = (n_read as libc::c_ulong).wrapping_sub(to_write) as size_t
                    as size_t;
                to_write = n_bytes;
            }
        }
        if n_read != 0 as libc::c_int as libc::c_ulong {
            if filter_ok as libc::c_int != 0 || new_file_flag as libc::c_int != 0 {
                filter_ok = cwrite(new_file_flag, bp_out, n_read);
            }
            opened = (opened as libc::c_ulong)
                .wrapping_add(new_file_flag as libc::c_ulong) as uintmax_t as uintmax_t;
            new_file_flag = 0 as libc::c_int != 0;
            if !filter_ok && opened == max_files {
                break;
            }
            to_write = (to_write as libc::c_ulong).wrapping_sub(n_read) as uintmax_t
                as uintmax_t;
        }
        if eof {
            break;
        }
    }
    loop {
        let fresh4 = opened;
        opened = opened.wrapping_add(1);
        if !(fresh4 < max_files) {
            break;
        }
        cwrite(
            1 as libc::c_int != 0,
            0 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
    };
}
unsafe extern "C" fn lines_split(
    mut n_lines: uintmax_t,
    mut buf: *mut libc::c_char,
    mut bufsize: size_t,
) {
    let mut n_read: size_t = 0;
    let mut bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bp_out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut eob: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_file_flag: bool = 1 as libc::c_int != 0;
    let mut n: uintmax_t = 0 as libc::c_int as uintmax_t;
    loop {
        n_read = safe_read(0 as libc::c_int, buf as *mut libc::c_void, bufsize);
        if n_read == -(1 as libc::c_int) as size_t {
            if ::core::mem::size_of::<C2RustUnnamed_35>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        infile,
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
                        infile,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        bp_out = buf;
        bp = bp_out;
        eob = bp.offset(n_read as isize);
        *eob = eolchar as libc::c_char;
        loop {
            bp = rawmemchr(bp as *const libc::c_void, eolchar) as *mut libc::c_char;
            if bp == eob {
                if eob != bp_out {
                    let mut len: size_t = eob.offset_from(bp_out) as libc::c_long
                        as size_t;
                    cwrite(new_file_flag, bp_out, len);
                    new_file_flag = 0 as libc::c_int != 0;
                }
                break;
            } else {
                bp = bp.offset(1);
                bp;
                n = n.wrapping_add(1);
                if n >= n_lines {
                    cwrite(
                        new_file_flag,
                        bp_out,
                        bp.offset_from(bp_out) as libc::c_long as size_t,
                    );
                    bp_out = bp;
                    new_file_flag = 1 as libc::c_int != 0;
                    n = 0 as libc::c_int as uintmax_t;
                }
            }
        }
        if !(n_read != 0) {
            break;
        }
    };
}
unsafe extern "C" fn line_bytes_split(
    mut n_bytes: uintmax_t,
    mut buf: *mut libc::c_char,
    mut bufsize: size_t,
) {
    let mut n_read: size_t = 0;
    let mut n_out: uintmax_t = 0 as libc::c_int as uintmax_t;
    let mut n_hold: size_t = 0 as libc::c_int as size_t;
    let mut hold: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hold_size: size_t = 0 as libc::c_int as size_t;
    let mut split_line: bool = 0 as libc::c_int != 0;
    loop {
        n_read = safe_read(0 as libc::c_int, buf as *mut libc::c_void, bufsize);
        if n_read == -(1 as libc::c_int) as size_t {
            if ::core::mem::size_of::<C2RustUnnamed_36>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        infile,
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
                        infile,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        let mut n_left: size_t = n_read;
        let mut sob: *mut libc::c_char = buf;
        while n_left != 0 {
            let mut split_rest: size_t = 0 as libc::c_int as size_t;
            let mut eoc: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut eol: *mut libc::c_char = 0 as *mut libc::c_char;
            if n_bytes.wrapping_sub(n_out).wrapping_sub(n_hold) <= n_left {
                split_rest = n_bytes.wrapping_sub(n_out).wrapping_sub(n_hold);
                eoc = sob
                    .offset(split_rest as isize)
                    .offset(-(1 as libc::c_int as isize));
                eol = memrchr(sob as *const libc::c_void, eolchar, split_rest)
                    as *mut libc::c_char;
            } else {
                eol = memrchr(sob as *const libc::c_void, eolchar, n_left)
                    as *mut libc::c_char;
            }
            if n_hold != 0 && !(eol.is_null() && n_out != 0) {
                cwrite(n_out == 0 as libc::c_int as libc::c_ulong, hold, n_hold);
                n_out = (n_out as libc::c_ulong).wrapping_add(n_hold) as uintmax_t
                    as uintmax_t;
                if n_hold > bufsize {
                    hold = xrealloc(hold as *mut libc::c_void, bufsize)
                        as *mut libc::c_char;
                }
                n_hold = 0 as libc::c_int as size_t;
                hold_size = bufsize;
            }
            if !eol.is_null() {
                split_line = 1 as libc::c_int != 0;
                let mut n_write: size_t = (eol.offset_from(sob) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as size_t;
                cwrite(n_out == 0 as libc::c_int as libc::c_ulong, sob, n_write);
                n_out = (n_out as libc::c_ulong).wrapping_add(n_write) as uintmax_t
                    as uintmax_t;
                n_left = (n_left as libc::c_ulong).wrapping_sub(n_write) as size_t
                    as size_t;
                sob = sob.offset(n_write as isize);
                if !eoc.is_null() {
                    split_rest = (split_rest as libc::c_ulong).wrapping_sub(n_write)
                        as size_t as size_t;
                }
            }
            if n_left != 0 && !split_line {
                let mut n_write_0: size_t = if !eoc.is_null() {
                    split_rest
                } else {
                    n_left
                };
                cwrite(n_out == 0 as libc::c_int as libc::c_ulong, sob, n_write_0);
                n_out = (n_out as libc::c_ulong).wrapping_add(n_write_0) as uintmax_t
                    as uintmax_t;
                n_left = (n_left as libc::c_ulong).wrapping_sub(n_write_0) as size_t
                    as size_t;
                sob = sob.offset(n_write_0 as isize);
                if !eoc.is_null() {
                    split_rest = (split_rest as libc::c_ulong).wrapping_sub(n_write_0)
                        as size_t as size_t;
                }
            }
            if !eoc.is_null() && split_rest != 0 || eoc.is_null() && n_left != 0 {
                let mut n_buf: size_t = if !eoc.is_null() { split_rest } else { n_left };
                if hold_size.wrapping_sub(n_hold) < n_buf {
                    if hold_size
                        <= (18446744073709551615 as libc::c_ulong).wrapping_sub(bufsize)
                    {
                        hold_size = (hold_size as libc::c_ulong).wrapping_add(bufsize)
                            as size_t as size_t;
                    } else {
                        xalloc_die();
                    }
                    hold = xrealloc(hold as *mut libc::c_void, hold_size)
                        as *mut libc::c_char;
                }
                memcpy(
                    hold.offset(n_hold as isize) as *mut libc::c_void,
                    sob as *const libc::c_void,
                    n_buf,
                );
                n_hold = (n_hold as libc::c_ulong).wrapping_add(n_buf) as size_t
                    as size_t;
                n_left = (n_left as libc::c_ulong).wrapping_sub(n_buf) as size_t
                    as size_t;
                sob = sob.offset(n_buf as isize);
            }
            if !eoc.is_null() {
                n_out = 0 as libc::c_int as uintmax_t;
                split_line = 0 as libc::c_int != 0;
            }
        }
        if !(n_read != 0) {
            break;
        }
    }
    if n_hold != 0 {
        cwrite(n_out == 0 as libc::c_int as libc::c_ulong, hold, n_hold);
    }
    free(hold as *mut libc::c_void);
}
unsafe extern "C" fn lines_chunk_split(
    mut k: uintmax_t,
    mut n: uintmax_t,
    mut buf: *mut libc::c_char,
    mut bufsize: size_t,
    mut initial_read: size_t,
    mut file_size: off_t,
) {
    if n != 0 && k <= n && n <= file_size as libc::c_ulong {} else {
        __assert_fail(
            b"n && k <= n && n <= file_size\0" as *const u8 as *const libc::c_char,
            b"src/split.c\0" as *const u8 as *const libc::c_char,
            866 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"void lines_chunk_split(uintmax_t, uintmax_t, char *, size_t, size_t, off_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_12277: {
        if n != 0 && k <= n && n <= file_size as libc::c_ulong {} else {
            __assert_fail(
                b"n && k <= n && n <= file_size\0" as *const u8 as *const libc::c_char,
                b"src/split.c\0" as *const u8 as *const libc::c_char,
                866 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 76],
                    &[libc::c_char; 76],
                >(
                    b"void lines_chunk_split(uintmax_t, uintmax_t, char *, size_t, size_t, off_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let chunk_size: off_t = (file_size as libc::c_ulong).wrapping_div(n) as off_t;
    let mut chunk_no: uintmax_t = 1 as libc::c_int as uintmax_t;
    let mut chunk_end: off_t = chunk_size - 1 as libc::c_int as libc::c_long;
    let mut n_written: off_t = 0 as libc::c_int as off_t;
    let mut new_file_flag: bool = 1 as libc::c_int != 0;
    let mut chunk_truncated: bool = 0 as libc::c_int != 0;
    if k > 1 as libc::c_int as libc::c_ulong {
        let mut start: off_t = k
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(chunk_size as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as off_t;
        if (start as libc::c_ulong) < initial_read {
            memmove(
                buf as *mut libc::c_void,
                buf.offset(start as isize) as *const libc::c_void,
                initial_read.wrapping_sub(start as libc::c_ulong),
            );
            initial_read = (initial_read as libc::c_ulong)
                .wrapping_sub(start as libc::c_ulong) as size_t as size_t;
        } else {
            if lseek(
                0 as libc::c_int,
                (start as libc::c_ulong).wrapping_sub(initial_read) as __off_t,
                1 as libc::c_int,
            ) < 0 as libc::c_int as libc::c_long
            {
                if ::core::mem::size_of::<C2RustUnnamed_39>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            infile,
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
                            infile,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            initial_read = 18446744073709551615 as libc::c_ulong;
        }
        n_written = start;
        chunk_no = k.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        chunk_end = chunk_no
            .wrapping_mul(chunk_size as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as off_t;
    }
    while n_written < file_size {
        let mut bp: *mut libc::c_char = buf;
        let mut eob: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut n_read: size_t = 0;
        if initial_read != 18446744073709551615 as libc::c_ulong {
            n_read = initial_read;
            initial_read = 18446744073709551615 as libc::c_ulong;
        } else {
            n_read = safe_read(0 as libc::c_int, buf as *mut libc::c_void, bufsize);
            if n_read == -(1 as libc::c_int) as size_t {
                if ::core::mem::size_of::<C2RustUnnamed_38>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            infile,
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
                            infile,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
        if n_read == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        n_read = if n_read < (file_size - n_written) as libc::c_ulong {
            n_read
        } else {
            (file_size - n_written) as libc::c_ulong
        };
        chunk_truncated = 0 as libc::c_int != 0;
        eob = buf.offset(n_read as isize);
        while bp != eob {
            let mut to_write: size_t = 0;
            let mut next: bool = 0 as libc::c_int != 0;
            let mut skip: off_t = (if n_read
                < (if 0 as libc::c_int as libc::c_long > chunk_end - n_written {
                    0 as libc::c_int as libc::c_long
                } else {
                    chunk_end - n_written
                }) as libc::c_ulong
            {
                n_read
            } else {
                (if 0 as libc::c_int as libc::c_long > chunk_end - n_written {
                    0 as libc::c_int as libc::c_long
                } else {
                    chunk_end - n_written
                }) as libc::c_ulong
            }) as off_t;
            let mut bp_out: *mut libc::c_char = memchr(
                bp.offset(skip as isize) as *const libc::c_void,
                eolchar,
                n_read.wrapping_sub(skip as libc::c_ulong),
            ) as *mut libc::c_char;
            if !bp_out.is_null() {
                bp_out = bp_out.offset(1);
                bp_out;
                next = 1 as libc::c_int != 0;
            } else {
                bp_out = eob;
            }
            to_write = bp_out.offset_from(bp) as libc::c_long as size_t;
            if k == chunk_no {
                if full_write(1 as libc::c_int, bp as *const libc::c_void, to_write)
                    != to_write
                {
                    if ::core::mem::size_of::<C2RustUnnamed_37>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            b"%s\0" as *const u8 as *const libc::c_char,
                            gettext(b"write error\0" as *const u8 as *const libc::c_char),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            b"%s\0" as *const u8 as *const libc::c_char,
                            gettext(b"write error\0" as *const u8 as *const libc::c_char),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            } else if k == 0 {
                cwrite(new_file_flag, bp, to_write);
            }
            n_written = (n_written as libc::c_ulong).wrapping_add(to_write) as off_t
                as off_t;
            bp = bp.offset(to_write as isize);
            n_read = (n_read as libc::c_ulong).wrapping_sub(to_write) as size_t
                as size_t;
            new_file_flag = next;
            while next as libc::c_int != 0
                || chunk_end <= n_written - 1 as libc::c_int as libc::c_long
            {
                if !next && bp == eob {
                    chunk_truncated = 1 as libc::c_int != 0;
                    break;
                } else {
                    chunk_no = chunk_no.wrapping_add(1);
                    chunk_no;
                    if k != 0 && chunk_no > k {
                        return;
                    }
                    if chunk_no == n {
                        chunk_end = file_size - 1 as libc::c_int as libc::c_long;
                    } else {
                        chunk_end += chunk_size;
                    }
                    if chunk_end <= n_written - 1 as libc::c_int as libc::c_long {
                        if k == 0 {
                            cwrite(
                                1 as libc::c_int != 0,
                                0 as *const libc::c_char,
                                0 as libc::c_int as size_t,
                            );
                        }
                    } else {
                        next = 0 as libc::c_int != 0;
                    }
                }
            }
        }
    }
    if chunk_truncated {
        chunk_no = chunk_no.wrapping_add(1);
        chunk_no;
    }
    while k == 0
        && {
            let fresh5 = chunk_no;
            chunk_no = chunk_no.wrapping_add(1);
            fresh5 <= n
        }
    {
        cwrite(
            1 as libc::c_int != 0,
            0 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
    }
}
unsafe extern "C" fn bytes_chunk_extract(
    mut k: uintmax_t,
    mut n: uintmax_t,
    mut buf: *mut libc::c_char,
    mut bufsize: size_t,
    mut initial_read: size_t,
    mut file_size: off_t,
) {
    let mut start: off_t = 0;
    let mut end: off_t = 0;
    if k != 0 && n != 0 && k <= n && n <= file_size as libc::c_ulong {} else {
        __assert_fail(
            b"k && n && k <= n && n <= file_size\0" as *const u8 as *const libc::c_char,
            b"src/split.c\0" as *const u8 as *const libc::c_char,
            995 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 78],
                &[libc::c_char; 78],
            >(
                b"void bytes_chunk_extract(uintmax_t, uintmax_t, char *, size_t, size_t, off_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_12786: {
        if k != 0 && n != 0 && k <= n && n <= file_size as libc::c_ulong {} else {
            __assert_fail(
                b"k && n && k <= n && n <= file_size\0" as *const u8
                    as *const libc::c_char,
                b"src/split.c\0" as *const u8 as *const libc::c_char,
                995 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 78],
                    &[libc::c_char; 78],
                >(
                    b"void bytes_chunk_extract(uintmax_t, uintmax_t, char *, size_t, size_t, off_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    start = k
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul((file_size as libc::c_ulong).wrapping_div(n)) as off_t;
    end = (if k == n {
        file_size as libc::c_ulong
    } else {
        k.wrapping_mul((file_size as libc::c_ulong).wrapping_div(n))
    }) as off_t;
    if (start as libc::c_ulong) < initial_read {
        memmove(
            buf as *mut libc::c_void,
            buf.offset(start as isize) as *const libc::c_void,
            initial_read.wrapping_sub(start as libc::c_ulong),
        );
        initial_read = (initial_read as libc::c_ulong)
            .wrapping_sub(start as libc::c_ulong) as size_t as size_t;
    } else {
        if lseek(
            0 as libc::c_int,
            (start as libc::c_ulong).wrapping_sub(initial_read) as __off_t,
            1 as libc::c_int,
        ) < 0 as libc::c_int as libc::c_long
        {
            if ::core::mem::size_of::<C2RustUnnamed_42>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        infile,
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
                        infile,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        initial_read = 18446744073709551615 as libc::c_ulong;
    }
    while start < end {
        let mut n_read: size_t = 0;
        if initial_read != 18446744073709551615 as libc::c_ulong {
            n_read = initial_read;
            initial_read = 18446744073709551615 as libc::c_ulong;
        } else {
            n_read = safe_read(0 as libc::c_int, buf as *mut libc::c_void, bufsize);
            if n_read == -(1 as libc::c_int) as size_t {
                if ::core::mem::size_of::<C2RustUnnamed_41>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            infile,
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
                            infile,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
        if n_read == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        n_read = if n_read < (end - start) as libc::c_ulong {
            n_read
        } else {
            (end - start) as libc::c_ulong
        };
        if full_write(1 as libc::c_int, buf as *const libc::c_void, n_read) != n_read
            && !ignorable(*__errno_location())
        {
            if ::core::mem::size_of::<C2RustUnnamed_40>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        b"-\0" as *const u8 as *const libc::c_char,
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
                        b"-\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        start = (start as libc::c_ulong).wrapping_add(n_read) as off_t as off_t;
    }
}
unsafe extern "C" fn ofile_open(
    mut files: *mut of_t,
    mut i_check: size_t,
    mut nfiles: size_t,
) -> bool {
    let mut file_limit: bool = 0 as libc::c_int != 0;
    if (*files.offset(i_check as isize)).ofd <= OFD_NEW as libc::c_int {
        let mut fd: libc::c_int = 0;
        let mut i_reopen: size_t = if i_check != 0 {
            i_check.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        } else {
            nfiles.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        };
        loop {
            if (*files.offset(i_check as isize)).ofd == OFD_NEW as libc::c_int {
                fd = create((*files.offset(i_check as isize)).of_name);
            } else {
                fd = open_safer(
                    (*files.offset(i_check as isize)).of_name,
                    0o1 as libc::c_int | 0 as libc::c_int | 0o2000 as libc::c_int
                        | 0o4000 as libc::c_int,
                );
            }
            if -(1 as libc::c_int) < fd {
                break;
            }
            if !(*__errno_location() == 24 as libc::c_int
                || *__errno_location() == 23 as libc::c_int)
            {
                if ::core::mem::size_of::<C2RustUnnamed_47>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            (*files.offset(i_check as isize)).of_name,
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
                            (*files.offset(i_check as isize)).of_name,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            file_limit = 1 as libc::c_int != 0;
            while (*files.offset(i_reopen as isize)).ofd < 0 as libc::c_int {
                i_reopen = if i_reopen != 0 {
                    i_reopen.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                } else {
                    nfiles.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                };
                if i_reopen == i_check {
                    if ::core::mem::size_of::<C2RustUnnamed_46>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            b"%s\0" as *const u8 as *const libc::c_char,
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                (*files.offset(i_check as isize)).of_name,
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
                                (*files.offset(i_check as isize)).of_name,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            }
            if rpl_fclose((*files.offset(i_reopen as isize)).ofile) != 0 as libc::c_int {
                if ::core::mem::size_of::<C2RustUnnamed_45>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            (*files.offset(i_reopen as isize)).of_name,
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
                            (*files.offset(i_reopen as isize)).of_name,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            let ref mut fresh6 = (*files.offset(i_reopen as isize)).ofile;
            *fresh6 = 0 as *mut FILE;
            (*files.offset(i_reopen as isize)).ofd = OFD_APPEND as libc::c_int;
        }
        (*files.offset(i_check as isize)).ofd = fd;
        let ref mut fresh7 = (*files.offset(i_check as isize)).ofile;
        *fresh7 = fdopen(fd, b"a\0" as *const u8 as *const libc::c_char);
        if (*fresh7).is_null() {
            if ::core::mem::size_of::<C2RustUnnamed_44>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        (*files.offset(i_check as isize)).of_name,
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
                        (*files.offset(i_check as isize)).of_name,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        (*files.offset(i_check as isize)).opid = filter_pid;
        filter_pid = 0 as libc::c_int;
    }
    return file_limit;
}
unsafe extern "C" fn lines_rr(
    mut k: uintmax_t,
    mut n: uintmax_t,
    mut buf: *mut libc::c_char,
    mut bufsize: size_t,
) {
    let mut wrapped: bool = 0 as libc::c_int != 0;
    let mut wrote: bool = 0 as libc::c_int != 0;
    let mut file_limit: bool = false;
    let mut i_file: size_t = 0;
    let mut files: *mut of_t = 0 as *mut of_t;
    let mut line_no: uintmax_t = 0;
    if k != 0 {
        line_no = 1 as libc::c_int as uintmax_t;
    } else {
        if (18446744073709551615 as libc::c_ulong) < n {
            xalloc_die();
        }
        files = xnmalloc(n, ::core::mem::size_of::<of_t>() as libc::c_ulong)
            as *mut of_t;
        i_file = 0 as libc::c_int as size_t;
        while i_file < n {
            next_file_name();
            let ref mut fresh8 = (*files.offset(i_file as isize)).of_name;
            *fresh8 = xstrdup(outfile);
            (*files.offset(i_file as isize)).ofd = OFD_NEW as libc::c_int;
            let ref mut fresh9 = (*files.offset(i_file as isize)).ofile;
            *fresh9 = 0 as *mut FILE;
            (*files.offset(i_file as isize)).opid = 0 as libc::c_int;
            i_file = i_file.wrapping_add(1);
            i_file;
        }
        i_file = 0 as libc::c_int as size_t;
        file_limit = 0 as libc::c_int != 0;
    }
    's_76: loop {
        let mut bp: *mut libc::c_char = buf;
        let mut eob: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut n_read: size_t = safe_read(
            0 as libc::c_int,
            buf as *mut libc::c_void,
            bufsize,
        );
        if n_read == -(1 as libc::c_int) as size_t {
            if ::core::mem::size_of::<C2RustUnnamed_53>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        infile,
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
                        infile,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        } else if n_read == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        eob = buf.offset(n_read as isize);
        while bp != eob {
            let mut to_write: size_t = 0;
            let mut next: bool = 0 as libc::c_int != 0;
            let mut bp_out: *mut libc::c_char = memchr(
                bp as *const libc::c_void,
                eolchar,
                eob.offset_from(bp) as libc::c_long as libc::c_ulong,
            ) as *mut libc::c_char;
            if !bp_out.is_null() {
                bp_out = bp_out.offset(1);
                bp_out;
                next = 1 as libc::c_int != 0;
            } else {
                bp_out = eob;
            }
            to_write = bp_out.offset_from(bp) as libc::c_long as size_t;
            if k != 0 {
                if line_no == k && unbuffered as libc::c_int != 0 {
                    if full_write(1 as libc::c_int, bp as *const libc::c_void, to_write)
                        != to_write
                    {
                        if ::core::mem::size_of::<C2RustUnnamed_52>() as libc::c_ulong
                            != 0
                        {
                            error(
                                1 as libc::c_int,
                                *__errno_location(),
                                b"%s\0" as *const u8 as *const libc::c_char,
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
                                b"%s\0" as *const u8 as *const libc::c_char,
                                gettext(
                                    b"write error\0" as *const u8 as *const libc::c_char,
                                ),
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                } else if line_no == k
                    && fwrite_unlocked(
                        bp as *const libc::c_void,
                        to_write,
                        1 as libc::c_int as size_t,
                        stdout,
                    ) != 1 as libc::c_int as libc::c_ulong
                {
                    clearerr_unlocked(stdout);
                    if ::core::mem::size_of::<C2RustUnnamed_51>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            b"%s\0" as *const u8 as *const libc::c_char,
                            gettext(b"write error\0" as *const u8 as *const libc::c_char),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            b"%s\0" as *const u8 as *const libc::c_char,
                            gettext(b"write error\0" as *const u8 as *const libc::c_char),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                if next {
                    line_no = if line_no == n {
                        1 as libc::c_int as libc::c_ulong
                    } else {
                        line_no.wrapping_add(1 as libc::c_int as libc::c_ulong)
                    };
                }
            } else {
                file_limit = (file_limit as libc::c_int
                    | ofile_open(files, i_file, n) as libc::c_int) != 0;
                if unbuffered {
                    if full_write(
                        (*files.offset(i_file as isize)).ofd,
                        bp as *const libc::c_void,
                        to_write,
                    ) != to_write && !ignorable(*__errno_location())
                    {
                        if ::core::mem::size_of::<C2RustUnnamed_50>() as libc::c_ulong
                            != 0
                        {
                            error(
                                1 as libc::c_int,
                                *__errno_location(),
                                b"%s\0" as *const u8 as *const libc::c_char,
                                quotearg_n_style_colon(
                                    0 as libc::c_int,
                                    shell_escape_quoting_style,
                                    (*files.offset(i_file as isize)).of_name,
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
                                    (*files.offset(i_file as isize)).of_name,
                                ),
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                } else if fwrite_unlocked(
                    bp as *const libc::c_void,
                    to_write,
                    1 as libc::c_int as size_t,
                    (*files.offset(i_file as isize)).ofile,
                ) != 1 as libc::c_int as libc::c_ulong && !ignorable(*__errno_location())
                {
                    if ::core::mem::size_of::<C2RustUnnamed_49>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            b"%s\0" as *const u8 as *const libc::c_char,
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                (*files.offset(i_file as isize)).of_name,
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
                                (*files.offset(i_file as isize)).of_name,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                if !ignorable(*__errno_location()) {
                    wrote = 1 as libc::c_int != 0;
                }
                if file_limit {
                    if rpl_fclose((*files.offset(i_file as isize)).ofile)
                        != 0 as libc::c_int
                    {
                        if ::core::mem::size_of::<C2RustUnnamed_48>() as libc::c_ulong
                            != 0
                        {
                            error(
                                1 as libc::c_int,
                                *__errno_location(),
                                b"%s\0" as *const u8 as *const libc::c_char,
                                quotearg_n_style_colon(
                                    0 as libc::c_int,
                                    shell_escape_quoting_style,
                                    (*files.offset(i_file as isize)).of_name,
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
                                    (*files.offset(i_file as isize)).of_name,
                                ),
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                    let ref mut fresh10 = (*files.offset(i_file as isize)).ofile;
                    *fresh10 = 0 as *mut FILE;
                    (*files.offset(i_file as isize)).ofd = OFD_APPEND as libc::c_int;
                }
                if next as libc::c_int != 0
                    && {
                        i_file = i_file.wrapping_add(1);
                        i_file == n
                    }
                {
                    wrapped = 1 as libc::c_int != 0;
                    if !wrote {
                        break 's_76;
                    }
                    wrote = 0 as libc::c_int != 0;
                    i_file = 0 as libc::c_int as size_t;
                }
            }
            bp = bp_out;
        }
    }
    if k == 0 {
        let mut ceiling: libc::c_int = (if wrapped as libc::c_int != 0 {
            n
        } else {
            i_file
        }) as libc::c_int;
        i_file = 0 as libc::c_int as size_t;
        while i_file < n {
            if i_file >= ceiling as libc::c_ulong && !elide_empty_files {
                file_limit = (file_limit as libc::c_int
                    | ofile_open(files, i_file, n) as libc::c_int) != 0;
            }
            if (*files.offset(i_file as isize)).ofd >= 0 as libc::c_int {
                closeout(
                    (*files.offset(i_file as isize)).ofile,
                    (*files.offset(i_file as isize)).ofd,
                    (*files.offset(i_file as isize)).opid,
                    (*files.offset(i_file as isize)).of_name,
                );
            }
            (*files.offset(i_file as isize)).ofd = OFD_APPEND as libc::c_int;
            i_file = i_file.wrapping_add(1);
            i_file;
        }
    }
}
unsafe extern "C" fn parse_chunk(
    mut k_units: *mut uintmax_t,
    mut n_units: *mut uintmax_t,
    mut slash: *mut libc::c_char,
) {
    *n_units = xdectoumax(
        slash.offset(1 as libc::c_int as isize),
        1 as libc::c_int as uintmax_t,
        18446744073709551615 as libc::c_ulong,
        b"\0" as *const u8 as *const libc::c_char,
        gettext(b"invalid number of chunks\0" as *const u8 as *const libc::c_char),
        0 as libc::c_int,
    );
    if slash != optarg {
        *slash = '\0' as i32 as libc::c_char;
        *k_units = xdectoumax(
            optarg,
            1 as libc::c_int as uintmax_t,
            *n_units,
            b"\0" as *const u8 as *const libc::c_char,
            gettext(b"invalid chunk number\0" as *const u8 as *const libc::c_char),
            0 as libc::c_int,
        );
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut split_type: Split_type = type_undef;
    let mut in_blk_size: size_t = 0 as libc::c_int as size_t;
    let mut page_size: size_t = getpagesize() as size_t;
    let mut k_units: uintmax_t = 0 as libc::c_int as uintmax_t;
    let mut n_units: uintmax_t = 0 as libc::c_int as uintmax_t;
    static mut multipliers: [libc::c_char; 13] = unsafe {
        *::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"bEGKkMmPTYZ0\0")
    };
    let mut c: libc::c_int = 0;
    let mut digits_optind: libc::c_int = 0 as libc::c_int;
    let mut file_size: off_t = if (0 as libc::c_int as off_t)
        < -(1 as libc::c_int) as off_t
    {
        -(1 as libc::c_int) as off_t
    } else {
        (((1 as libc::c_int as off_t)
            << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
            - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long
    };
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    infile = bad_cast(b"-\0" as *const u8 as *const libc::c_char);
    outbase = bad_cast(b"x\0" as *const u8 as *const libc::c_char);
    loop {
        let mut this_optind: libc::c_int = if optind != 0 {
            optind
        } else {
            1 as libc::c_int
        };
        let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
        c = getopt_long(
            argc,
            argv,
            b"0123456789C:a:b:del:n:t:ux\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if c == -(1 as libc::c_int) {
            break;
        }
        match c {
            97 => {
                suffix_length = xdectoumax(
                    optarg,
                    0 as libc::c_int as uintmax_t,
                    (18446744073709551615 as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<size_t>() as libc::c_ulong),
                    b"\0" as *const u8 as *const libc::c_char,
                    gettext(
                        b"invalid suffix length\0" as *const u8 as *const libc::c_char,
                    ),
                    0 as libc::c_int,
                );
            }
            131 => {
                if last_component(optarg) != optarg {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"invalid suffix %s, contains directory separator\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quote(optarg),
                    );
                    usage(1 as libc::c_int);
                }
                additional_suffix = optarg;
            }
            98 => {
                if split_type as libc::c_uint
                    != type_undef as libc::c_int as libc::c_uint
                {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"cannot split in more than one way\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    usage(1 as libc::c_int);
                }
                split_type = type_bytes;
                n_units = xdectoumax(
                    optarg,
                    1 as libc::c_int as uintmax_t,
                    (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                        -(1 as libc::c_int) as off_t
                    } else {
                        (((1 as libc::c_int as off_t)
                            << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    }) as uintmax_t,
                    multipliers.as_ptr(),
                    gettext(
                        b"invalid number of bytes\0" as *const u8 as *const libc::c_char,
                    ),
                    0 as libc::c_int,
                );
            }
            108 => {
                if split_type as libc::c_uint
                    != type_undef as libc::c_int as libc::c_uint
                {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"cannot split in more than one way\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    usage(1 as libc::c_int);
                }
                split_type = type_lines;
                n_units = xdectoumax(
                    optarg,
                    1 as libc::c_int as uintmax_t,
                    18446744073709551615 as libc::c_ulong,
                    b"\0" as *const u8 as *const libc::c_char,
                    gettext(
                        b"invalid number of lines\0" as *const u8 as *const libc::c_char,
                    ),
                    0 as libc::c_int,
                );
            }
            67 => {
                if split_type as libc::c_uint
                    != type_undef as libc::c_int as libc::c_uint
                {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"cannot split in more than one way\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    usage(1 as libc::c_int);
                }
                split_type = type_byteslines;
                n_units = xdectoumax(
                    optarg,
                    1 as libc::c_int as uintmax_t,
                    if (18446744073709551615 as libc::c_ulong)
                        < (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t
                        {
                            -(1 as libc::c_int) as off_t
                        } else {
                            (((1 as libc::c_int as off_t)
                                << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        }) as libc::c_ulong
                    {
                        18446744073709551615 as libc::c_ulong
                    } else {
                        (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                            -(1 as libc::c_int) as off_t
                        } else {
                            (((1 as libc::c_int as off_t)
                                << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        }) as libc::c_ulong
                    },
                    multipliers.as_ptr(),
                    gettext(
                        b"invalid number of bytes\0" as *const u8 as *const libc::c_char,
                    ),
                    0 as libc::c_int,
                );
            }
            110 => {
                if split_type as libc::c_uint
                    != type_undef as libc::c_int as libc::c_uint
                {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"cannot split in more than one way\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    usage(1 as libc::c_int);
                }
                while *(*__ctype_b_loc())
                    .offset(to_uchar(*optarg) as libc::c_int as isize) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    optarg = optarg.offset(1);
                    optarg;
                }
                if strncmp(
                    optarg,
                    b"r/\0" as *const u8 as *const libc::c_char,
                    (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0 as libc::c_int
                {
                    split_type = type_rr;
                    optarg = optarg.offset(2 as libc::c_int as isize);
                } else if strncmp(
                    optarg,
                    b"l/\0" as *const u8 as *const libc::c_char,
                    (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0 as libc::c_int
                {
                    split_type = type_chunk_lines;
                    optarg = optarg.offset(2 as libc::c_int as isize);
                } else {
                    split_type = type_chunk_bytes;
                }
                slash = strchr(optarg, '/' as i32);
                if !slash.is_null() {
                    parse_chunk(&mut k_units, &mut n_units, slash);
                } else {
                    n_units = xdectoumax(
                        optarg,
                        1 as libc::c_int as uintmax_t,
                        18446744073709551615 as libc::c_ulong,
                        b"\0" as *const u8 as *const libc::c_char,
                        gettext(
                            b"invalid number of chunks\0" as *const u8
                                as *const libc::c_char,
                        ),
                        0 as libc::c_int,
                    );
                }
            }
            117 => {
                unbuffered = 1 as libc::c_int != 0;
            }
            116 => {
                let mut neweol: libc::c_char = *optarg.offset(0 as libc::c_int as isize);
                if neweol == 0 {
                    if ::core::mem::size_of::<C2RustUnnamed_63>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"empty record separator\0" as *const u8
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
                                b"empty record separator\0" as *const u8
                                    as *const libc::c_char,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                if *optarg.offset(1 as libc::c_int as isize) != 0 {
                    if strcmp(optarg, b"\\0\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        neweol = '\0' as i32 as libc::c_char;
                    } else {
                        if ::core::mem::size_of::<C2RustUnnamed_62>() as libc::c_ulong
                            != 0
                        {
                            error(
                                1 as libc::c_int,
                                0 as libc::c_int,
                                gettext(
                                    b"multi-character separator %s\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                quote(optarg),
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        } else {
                            error(
                                1 as libc::c_int,
                                0 as libc::c_int,
                                gettext(
                                    b"multi-character separator %s\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                quote(optarg),
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                }
                if 0 as libc::c_int <= eolchar && neweol as libc::c_int != eolchar {
                    if ::core::mem::size_of::<C2RustUnnamed_61>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"multiple separator characters specified\0" as *const u8
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
                                b"multiple separator characters specified\0" as *const u8
                                    as *const libc::c_char,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                eolchar = neweol as libc::c_int;
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                if split_type as libc::c_uint
                    == type_undef as libc::c_int as libc::c_uint
                {
                    split_type = type_digits;
                    n_units = 0 as libc::c_int as uintmax_t;
                }
                if split_type as libc::c_uint
                    != type_undef as libc::c_int as libc::c_uint
                    && split_type as libc::c_uint
                        != type_digits as libc::c_int as libc::c_uint
                {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"cannot split in more than one way\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    usage(1 as libc::c_int);
                }
                if digits_optind != 0 as libc::c_int && digits_optind != this_optind {
                    n_units = 0 as libc::c_int as uintmax_t;
                }
                digits_optind = this_optind;
                &mut n_units as *mut uintmax_t;
                if (if ::core::mem::size_of::<C2RustUnnamed_60>() as libc::c_ulong != 0 {
                    if (-(1 as libc::c_int) as uintmax_t)
                        .wrapping_div(10 as libc::c_int as libc::c_ulong) < n_units
                        || n_units
                            .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                            .wrapping_add((c - '0' as i32) as libc::c_ulong) < n_units
                    {
                        0 as libc::c_int
                    } else {
                        n_units = n_units
                            .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                            .wrapping_add((c - '0' as i32) as libc::c_ulong);
                        1 as libc::c_int
                    }
                } else {
                    if (-(1 as libc::c_int) as uintmax_t)
                        .wrapping_div(10 as libc::c_int as libc::c_ulong) < n_units
                        || n_units
                            .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                            .wrapping_add((c - '0' as i32) as libc::c_ulong) < n_units
                    {
                        0 as libc::c_int
                    } else {
                        n_units = n_units
                            .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                            .wrapping_add((c - '0' as i32) as libc::c_ulong);
                        1 as libc::c_int
                    }
                }) == 0
                {
                    let mut buffer: [libc::c_char; 21] = [0; 21];
                    if ::core::mem::size_of::<C2RustUnnamed_59>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"line count option -%s%c... is too large\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            umaxtostr(n_units, buffer.as_mut_ptr()),
                            c,
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"line count option -%s%c... is too large\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            umaxtostr(n_units, buffer.as_mut_ptr()),
                            c,
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            }
            100 | 120 => {
                if c == 'd' as i32 {
                    suffix_alphabet = b"0123456789\0" as *const u8
                        as *const libc::c_char;
                } else {
                    suffix_alphabet = b"0123456789abcdef\0" as *const u8
                        as *const libc::c_char;
                }
                if !optarg.is_null() {
                    if strlen(optarg) != strspn(optarg, suffix_alphabet) {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            if c == 'd' as i32 {
                                gettext(
                                    b"%s: invalid start value for numerical suffix\0"
                                        as *const u8 as *const libc::c_char,
                                )
                            } else {
                                gettext(
                                    b"%s: invalid start value for hexadecimal suffix\0"
                                        as *const u8 as *const libc::c_char,
                                )
                            },
                            quote(optarg),
                        );
                        usage(1 as libc::c_int);
                    } else {
                        while *optarg as libc::c_int == '0' as i32
                            && *optarg.offset(1 as libc::c_int as isize) as libc::c_int
                                != '\0' as i32
                        {
                            optarg = optarg.offset(1);
                            optarg;
                        }
                        numeric_suffix_start = optarg;
                    }
                }
            }
            101 => {
                elide_empty_files = 1 as libc::c_int != 0;
            }
            129 => {
                filter_command = optarg;
            }
            130 => {
                in_blk_size = xdectoumax(
                    optarg,
                    1 as libc::c_int as uintmax_t,
                    (18446744073709551615 as libc::c_ulong).wrapping_sub(page_size),
                    multipliers.as_ptr(),
                    gettext(
                        b"invalid IO block size\0" as *const u8 as *const libc::c_char,
                    ),
                    0 as libc::c_int,
                );
            }
            128 => {
                verbose = 1 as libc::c_int != 0;
            }
            -130 => {
                usage(0 as libc::c_int);
            }
            -131 => {
                version_etc(
                    stdout,
                    b"split\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Torbjorn Granlund\0" as *const u8 as *const libc::c_char,
                    b"Richard M. Stallman\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    if k_units != 0 as libc::c_int as libc::c_ulong && !filter_command.is_null() {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"--filter does not process a chunk extracted to stdout\0" as *const u8
                    as *const libc::c_char,
            ),
        );
        usage(1 as libc::c_int);
    }
    if split_type as libc::c_uint == type_undef as libc::c_int as libc::c_uint {
        split_type = type_lines;
        n_units = 1000 as libc::c_int as uintmax_t;
    }
    if n_units == 0 as libc::c_int as libc::c_ulong {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            gettext(b"invalid number of lines\0" as *const u8 as *const libc::c_char),
            quote(b"0\0" as *const u8 as *const libc::c_char),
        );
        usage(1 as libc::c_int);
    }
    if eolchar < 0 as libc::c_int {
        eolchar = '\n' as i32;
    }
    set_suffix_length(n_units, split_type);
    if optind < argc {
        let fresh11 = optind;
        optind = optind + 1;
        infile = *argv.offset(fresh11 as isize);
    }
    if optind < argc {
        let fresh12 = optind;
        optind = optind + 1;
        outbase = *argv.offset(fresh12 as isize);
    }
    if optind < argc {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(b"extra operand %s\0" as *const u8 as *const libc::c_char),
            quote(*argv.offset(optind as isize)),
        );
        usage(1 as libc::c_int);
    }
    if !numeric_suffix_start.is_null() && strlen(numeric_suffix_start) > suffix_length {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"numerical suffix start value is too large for the suffix length\0"
                    as *const u8 as *const libc::c_char,
            ),
        );
        usage(1 as libc::c_int);
    }
    if !(strcmp(infile, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int)
        && fd_reopen(
            0 as libc::c_int,
            infile,
            0 as libc::c_int,
            0 as libc::c_int as mode_t,
        ) < 0 as libc::c_int
    {
        if ::core::mem::size_of::<C2RustUnnamed_58>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                gettext(
                    b"cannot open %s for reading\0" as *const u8 as *const libc::c_char,
                ),
                quotearg_style(shell_escape_always_quoting_style, infile),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                *__errno_location(),
                gettext(
                    b"cannot open %s for reading\0" as *const u8 as *const libc::c_char,
                ),
                quotearg_style(shell_escape_always_quoting_style, infile),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    xset_binary_mode(0 as libc::c_int, 0 as libc::c_int);
    if fstat(0 as libc::c_int, &mut in_stat_buf) != 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_57>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    infile,
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
                    infile,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    let mut specified_buf_size: bool = in_blk_size != 0;
    if !specified_buf_size {
        in_blk_size = io_blksize(in_stat_buf);
    }
    let mut b: *mut libc::c_void = xmalloc(
        in_blk_size
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(page_size)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    let mut buf: *mut libc::c_char = ptr_align(b, page_size) as *mut libc::c_char;
    let mut initial_read: size_t = 18446744073709551615 as libc::c_ulong;
    if split_type as libc::c_uint == type_chunk_bytes as libc::c_int as libc::c_uint
        || split_type as libc::c_uint == type_chunk_lines as libc::c_int as libc::c_uint
    {
        file_size = input_file_size(
            0 as libc::c_int,
            &mut in_stat_buf,
            buf,
            in_blk_size,
        );
        if file_size < 0 as libc::c_int as libc::c_long {
            if ::core::mem::size_of::<C2RustUnnamed_56>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"%s: cannot determine file size\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        infile,
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
                        b"%s: cannot determine file size\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        infile,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        initial_read = if (file_size as libc::c_ulong) < in_blk_size {
            file_size as libc::c_ulong
        } else {
            in_blk_size
        };
        if ((if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
            -(1 as libc::c_int) as off_t
        } else {
            (((1 as libc::c_int as off_t)
                << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        }) as libc::c_ulong) < n_units
        {
            let mut buffer_0: [libc::c_char; 21] = [0; 21];
            if ::core::mem::size_of::<C2RustUnnamed_55>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    75 as libc::c_int,
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    gettext(
                        b"invalid number of chunks\0" as *const u8 as *const libc::c_char,
                    ),
                    quote(umaxtostr(n_units, buffer_0.as_mut_ptr())),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    75 as libc::c_int,
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    gettext(
                        b"invalid number of chunks\0" as *const u8 as *const libc::c_char,
                    ),
                    quote(umaxtostr(n_units, buffer_0.as_mut_ptr())),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        file_size = (if file_size as libc::c_ulong > n_units {
            file_size as libc::c_ulong
        } else {
            n_units
        }) as off_t;
    }
    if !filter_command.is_null() {
        let mut act: sigaction = sigaction {
            __sigaction_handler: C2RustUnnamed_9 {
                sa_handler: None,
            },
            sa_mask: sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        sigemptyset(&mut newblocked);
        sigaction(13 as libc::c_int, 0 as *const sigaction, &mut act);
        if act.__sigaction_handler.sa_handler
            != ::core::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t)
        {
            sigaddset(&mut newblocked, 13 as libc::c_int);
        }
        sigprocmask(0 as libc::c_int, &mut newblocked, &mut oldblocked);
    }
    match split_type as libc::c_uint {
        4 | 3 => {
            lines_split(n_units, buf, in_blk_size);
        }
        1 => {
            bytes_split(
                n_units,
                buf,
                in_blk_size,
                18446744073709551615 as libc::c_ulong,
                0 as libc::c_int as uintmax_t,
            );
        }
        2 => {
            line_bytes_split(n_units, buf, in_blk_size);
        }
        5 => {
            if k_units == 0 as libc::c_int as libc::c_ulong {
                bytes_split(
                    (file_size as libc::c_ulong).wrapping_div(n_units),
                    buf,
                    in_blk_size,
                    initial_read,
                    n_units,
                );
            } else {
                bytes_chunk_extract(
                    k_units,
                    n_units,
                    buf,
                    in_blk_size,
                    initial_read,
                    file_size,
                );
            }
        }
        6 => {
            lines_chunk_split(
                k_units,
                n_units,
                buf,
                in_blk_size,
                initial_read,
                file_size,
            );
        }
        7 => {
            lines_rr(k_units, n_units, buf, in_blk_size);
        }
        _ => {
            abort();
        }
    }
    if close(0 as libc::c_int) != 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_54>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    infile,
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
                    infile,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    closeout(0 as *mut FILE, output_desc, filter_pid, outfile);
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
