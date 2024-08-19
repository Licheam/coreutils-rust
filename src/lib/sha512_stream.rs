extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn feof_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn SHA384_Init(c: *mut SHA512_CTX) -> libc::c_int;
    fn SHA384_Final(md: *mut libc::c_uchar, c: *mut SHA512_CTX) -> libc::c_int;
    fn SHA384(
        d: *const libc::c_uchar,
        n: size_t,
        md: *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn SHA512_Init(c: *mut SHA512_CTX) -> libc::c_int;
    fn SHA512_Update(
        c: *mut SHA512_CTX,
        data: *const libc::c_void,
        len: size_t,
    ) -> libc::c_int;
    fn SHA512_Final(md: *mut libc::c_uchar, c: *mut SHA512_CTX) -> libc::c_int;
    fn SHA512(
        d: *const libc::c_uchar,
        n: size_t,
        md: *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA512state_st {
    pub h: [libc::c_ulonglong; 8],
    pub Nl: libc::c_ulonglong,
    pub Nh: libc::c_ulonglong,
    pub u: C2RustUnnamed,
    pub num: libc::c_uint,
    pub md_len: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub d: [libc::c_ulonglong; 16],
    pub p: [libc::c_uchar; 128],
}
pub type SHA512_CTX = SHA512state_st;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const SHA384_DIGEST_SIZE: C2RustUnnamed_0 = 48;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const SHA512_DIGEST_SIZE: C2RustUnnamed_1 = 64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha512_ctx {
    pub CTX: SHA512_CTX,
}
#[no_mangle]
pub unsafe extern "C" fn sha384_stream(
    mut stream: *mut FILE,
    mut resblock: *mut libc::c_void,
) -> libc::c_int {
    return shaxxx_stream(
        stream,
        b"sha384\0" as *const u8 as *const libc::c_char,
        resblock,
        SHA384_DIGEST_SIZE as libc::c_int as ssize_t,
        Some(sha384_init_ctx as unsafe extern "C" fn(*mut sha512_ctx) -> ()),
        Some(
            sha384_finish_ctx
                as unsafe extern "C" fn(
                    *mut sha512_ctx,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn sha512_stream(
    mut stream: *mut FILE,
    mut resblock: *mut libc::c_void,
) -> libc::c_int {
    return shaxxx_stream(
        stream,
        b"sha512\0" as *const u8 as *const libc::c_char,
        resblock,
        SHA512_DIGEST_SIZE as libc::c_int as ssize_t,
        Some(sha512_init_ctx as unsafe extern "C" fn(*mut sha512_ctx) -> ()),
        Some(
            sha512_finish_ctx
                as unsafe extern "C" fn(
                    *mut sha512_ctx,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
    );
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha384_init_ctx(mut ctx: *mut sha512_ctx) {
    SHA384_Init(ctx as *mut SHA512_CTX);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha384_finish_ctx(
    mut ctx: *mut sha512_ctx,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    SHA384_Final(res as *mut libc::c_uchar, ctx as *mut SHA512_CTX);
    return res;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha384_buffer(
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    return SHA384(buf as *const libc::c_uchar, len, res as *mut libc::c_uchar)
        as *mut libc::c_void;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha384_read_ctx(
    mut ctx: *const sha512_ctx,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut tmp_ctx: SHA512_CTX = *(ctx as *mut SHA512_CTX);
    SHA384_Final(res as *mut libc::c_uchar, &mut tmp_ctx);
    return res;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha512_init_ctx(mut ctx: *mut sha512_ctx) {
    SHA512_Init(ctx as *mut SHA512_CTX);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha512_process_bytes(
    mut buf: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut sha512_ctx,
) {
    SHA512_Update(ctx as *mut SHA512_CTX, buf, len);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha512_process_block(
    mut buf: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut sha512_ctx,
) {
    sha512_process_bytes(buf, len, ctx);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha512_finish_ctx(
    mut ctx: *mut sha512_ctx,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    SHA512_Final(res as *mut libc::c_uchar, ctx as *mut SHA512_CTX);
    return res;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha512_buffer(
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    return SHA512(buf as *const libc::c_uchar, len, res as *mut libc::c_uchar)
        as *mut libc::c_void;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha512_read_ctx(
    mut ctx: *const sha512_ctx,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut tmp_ctx: SHA512_CTX = *(ctx as *mut SHA512_CTX);
    SHA512_Final(res as *mut libc::c_uchar, &mut tmp_ctx);
    return res;
}
#[inline]
unsafe extern "C" fn afalg_stream(
    mut stream: *mut FILE,
    mut alg: *const libc::c_char,
    mut resblock: *mut libc::c_void,
    mut hashlen: ssize_t,
) -> libc::c_int {
    return -(97 as libc::c_int);
}
unsafe extern "C" fn shaxxx_stream(
    mut stream: *mut FILE,
    mut alg: *const libc::c_char,
    mut resblock: *mut libc::c_void,
    mut hashlen: ssize_t,
    mut init_ctx: Option::<unsafe extern "C" fn(*mut sha512_ctx) -> ()>,
    mut finish_ctx: Option::<
        unsafe extern "C" fn(*mut sha512_ctx, *mut libc::c_void) -> *mut libc::c_void,
    >,
) -> libc::c_int {
    match afalg_stream(stream, alg, resblock, hashlen) {
        0 => return 0 as libc::c_int,
        -5 => return 1 as libc::c_int,
        _ => {}
    }
    let mut buffer: *mut libc::c_char = malloc(
        (32768 as libc::c_int + 72 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_char;
    if buffer.is_null() {
        return 1 as libc::c_int;
    }
    let mut ctx: sha512_ctx = sha512_ctx {
        CTX: SHA512_CTX {
            h: [0; 8],
            Nl: 0,
            Nh: 0,
            u: C2RustUnnamed { d: [0; 16] },
            num: 0,
            md_len: 0,
        },
    };
    init_ctx.expect("non-null function pointer")(&mut ctx);
    let mut sum: size_t = 0;
    's_34: loop {
        let mut n: size_t = 0;
        sum = 0 as libc::c_int as size_t;
        loop {
            if feof_unlocked(stream) != 0 {
                break 's_34;
            }
            n = fread_unlocked(
                buffer.offset(sum as isize) as *mut libc::c_void,
                1 as libc::c_int as size_t,
                (32768 as libc::c_int as libc::c_ulong).wrapping_sub(sum),
                stream,
            );
            sum = (sum as libc::c_ulong).wrapping_add(n) as size_t as size_t;
            if sum == 32768 as libc::c_int as libc::c_ulong {
                break;
            }
            if !(n == 0 as libc::c_int as libc::c_ulong) {
                continue;
            }
            if ferror_unlocked(stream) != 0 {
                free(buffer as *mut libc::c_void);
                return 1 as libc::c_int;
            }
            break 's_34;
        }
        sha512_process_block(
            buffer as *const libc::c_void,
            32768 as libc::c_int as size_t,
            &mut ctx,
        );
    }
    if sum > 0 as libc::c_int as libc::c_ulong {
        sha512_process_bytes(buffer as *const libc::c_void, sum, &mut ctx);
    }
    finish_ctx.expect("non-null function pointer")(&mut ctx, resblock);
    free(buffer as *mut libc::c_void);
    return 0 as libc::c_int;
}
