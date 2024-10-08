extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn blake2b_init(S: *mut blake2b_state, outlen: size_t) -> libc::c_int;
    fn blake2b_update(
        S: *mut blake2b_state,
        in_0: *const libc::c_void,
        inlen: size_t,
    ) -> libc::c_int;
    fn blake2b_final(
        S: *mut blake2b_state,
        out: *mut libc::c_void,
        outlen: size_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
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
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blake2b_state__ {
    pub h: [uint64_t; 8],
    pub t: [uint64_t; 2],
    pub f: [uint64_t; 2],
    pub buf: [uint8_t; 128],
    pub buflen: size_t,
    pub outlen: size_t,
    pub last_node: uint8_t,
}
pub type blake2b_state = blake2b_state__;
#[no_mangle]
pub unsafe extern "C" fn blake2b_stream(
    mut stream: *mut FILE,
    mut resstream: *mut libc::c_void,
    mut outbytes: size_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut sum: size_t = 0;
    let mut n: size_t = 0;
    let mut S: [blake2b_state; 1] = [blake2b_state {
        h: [0; 8],
        t: [0; 2],
        f: [0; 2],
        buf: [0; 128],
        buflen: 0,
        outlen: 0,
        last_node: 0,
    }; 1];
    static mut buffer_length: size_t = 32768 as libc::c_int as size_t;
    let mut buffer: *mut uint8_t = malloc(buffer_length) as *mut uint8_t;
    if buffer.is_null() {
        return -(1 as libc::c_int);
    }
    blake2b_init(S.as_mut_ptr(), outbytes);
    's_24: loop {
        sum = 0 as libc::c_int as size_t;
        loop {
            n = fread(
                buffer.offset(sum as isize) as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                buffer_length.wrapping_sub(sum),
                stream,
            );
            sum = (sum as libc::c_ulong).wrapping_add(n) as size_t as size_t;
            if buffer_length == sum {
                break;
            }
            if 0 as libc::c_int as libc::c_ulong == n {
                if ferror(stream) != 0 {
                    current_block = 17486602637616263956;
                    break 's_24;
                } else {
                    current_block = 8236137900636309791;
                    break 's_24;
                }
            } else if feof(stream) != 0 {
                current_block = 8236137900636309791;
                break 's_24;
            }
        }
        blake2b_update(S.as_mut_ptr(), buffer as *const libc::c_void, buffer_length);
    }
    match current_block {
        8236137900636309791 => {
            if sum > 0 as libc::c_int as libc::c_ulong {
                blake2b_update(S.as_mut_ptr(), buffer as *const libc::c_void, sum);
            }
            blake2b_final(S.as_mut_ptr(), resstream, outbytes);
            ret = 0 as libc::c_int;
        }
        _ => {}
    }
    free(buffer as *mut libc::c_void);
    return ret;
}
