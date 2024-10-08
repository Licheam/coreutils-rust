extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type blake2b_constant = libc::c_uint;
pub const BLAKE2B_PERSONALBYTES: blake2b_constant = 16;
pub const BLAKE2B_SALTBYTES: blake2b_constant = 16;
pub const BLAKE2B_KEYBYTES: blake2b_constant = 64;
pub const BLAKE2B_OUTBYTES: blake2b_constant = 64;
pub const BLAKE2B_BLOCKBYTES: blake2b_constant = 128;
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct blake2b_param__ {
    pub digest_length: uint8_t,
    pub key_length: uint8_t,
    pub fanout: uint8_t,
    pub depth: uint8_t,
    pub leaf_length: uint32_t,
    pub node_offset: uint32_t,
    pub xof_length: uint32_t,
    pub node_depth: uint8_t,
    pub inner_length: uint8_t,
    pub reserved: [uint8_t; 14],
    pub salt: [uint8_t; 16],
    pub personal: [uint8_t; 16],
}
pub type blake2b_param = blake2b_param__;
#[no_mangle]
pub unsafe extern "C" fn blake2b_init(
    mut S: *mut blake2b_state,
    mut outlen: size_t,
) -> libc::c_int {
    let mut P: [blake2b_param; 1] = [blake2b_param {
        digest_length: 0,
        key_length: 0,
        fanout: 0,
        depth: 0,
        leaf_length: 0,
        node_offset: 0,
        xof_length: 0,
        node_depth: 0,
        inner_length: 0,
        reserved: [0; 14],
        salt: [0; 16],
        personal: [0; 16],
    }; 1];
    if outlen == 0 || outlen > BLAKE2B_OUTBYTES as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    (*P.as_mut_ptr()).digest_length = outlen as uint8_t;
    (*P.as_mut_ptr()).key_length = 0 as libc::c_int as uint8_t;
    (*P.as_mut_ptr()).fanout = 1 as libc::c_int as uint8_t;
    (*P.as_mut_ptr()).depth = 1 as libc::c_int as uint8_t;
    store32(
        &mut (*P.as_mut_ptr()).leaf_length as *mut uint32_t as *mut libc::c_void,
        0 as libc::c_int as uint32_t,
    );
    store32(
        &mut (*P.as_mut_ptr()).node_offset as *mut uint32_t as *mut libc::c_void,
        0 as libc::c_int as uint32_t,
    );
    store32(
        &mut (*P.as_mut_ptr()).xof_length as *mut uint32_t as *mut libc::c_void,
        0 as libc::c_int as uint32_t,
    );
    (*P.as_mut_ptr()).node_depth = 0 as libc::c_int as uint8_t;
    (*P.as_mut_ptr()).inner_length = 0 as libc::c_int as uint8_t;
    memset(
        ((*P.as_mut_ptr()).reserved).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[uint8_t; 14]>() as libc::c_ulong,
    );
    memset(
        ((*P.as_mut_ptr()).salt).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
    );
    memset(
        ((*P.as_mut_ptr()).personal).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
    );
    return blake2b_init_param(S, P.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn blake2b_init_key(
    mut S: *mut blake2b_state,
    mut outlen: size_t,
    mut key: *const libc::c_void,
    mut keylen: size_t,
) -> libc::c_int {
    let mut P: [blake2b_param; 1] = [blake2b_param {
        digest_length: 0,
        key_length: 0,
        fanout: 0,
        depth: 0,
        leaf_length: 0,
        node_offset: 0,
        xof_length: 0,
        node_depth: 0,
        inner_length: 0,
        reserved: [0; 14],
        salt: [0; 16],
        personal: [0; 16],
    }; 1];
    if outlen == 0 || outlen > BLAKE2B_OUTBYTES as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if key.is_null() || keylen == 0
        || keylen > BLAKE2B_KEYBYTES as libc::c_int as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    (*P.as_mut_ptr()).digest_length = outlen as uint8_t;
    (*P.as_mut_ptr()).key_length = keylen as uint8_t;
    (*P.as_mut_ptr()).fanout = 1 as libc::c_int as uint8_t;
    (*P.as_mut_ptr()).depth = 1 as libc::c_int as uint8_t;
    store32(
        &mut (*P.as_mut_ptr()).leaf_length as *mut uint32_t as *mut libc::c_void,
        0 as libc::c_int as uint32_t,
    );
    store32(
        &mut (*P.as_mut_ptr()).node_offset as *mut uint32_t as *mut libc::c_void,
        0 as libc::c_int as uint32_t,
    );
    store32(
        &mut (*P.as_mut_ptr()).xof_length as *mut uint32_t as *mut libc::c_void,
        0 as libc::c_int as uint32_t,
    );
    (*P.as_mut_ptr()).node_depth = 0 as libc::c_int as uint8_t;
    (*P.as_mut_ptr()).inner_length = 0 as libc::c_int as uint8_t;
    memset(
        ((*P.as_mut_ptr()).reserved).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[uint8_t; 14]>() as libc::c_ulong,
    );
    memset(
        ((*P.as_mut_ptr()).salt).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
    );
    memset(
        ((*P.as_mut_ptr()).personal).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
    );
    if blake2b_init_param(S, P.as_mut_ptr()) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let mut block: [uint8_t; 128] = [0; 128];
    memset(
        block.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        BLAKE2B_BLOCKBYTES as libc::c_int as libc::c_ulong,
    );
    memcpy(block.as_mut_ptr() as *mut libc::c_void, key, keylen);
    blake2b_update(
        S,
        block.as_mut_ptr() as *const libc::c_void,
        BLAKE2B_BLOCKBYTES as libc::c_int as size_t,
    );
    secure_zero_memory(
        block.as_mut_ptr() as *mut libc::c_void,
        BLAKE2B_BLOCKBYTES as libc::c_int as size_t,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn blake2b_init_param(
    mut S: *mut blake2b_state,
    mut P: *const blake2b_param,
) -> libc::c_int {
    let mut p: *const uint8_t = P as *const uint8_t;
    let mut i: size_t = 0;
    blake2b_init0(S);
    i = 0 as libc::c_int as size_t;
    while i < 8 as libc::c_int as libc::c_ulong {
        (*S).h[i as usize]
            ^= load64(
                p
                    .offset(
                        (::core::mem::size_of::<uint64_t>() as libc::c_ulong)
                            .wrapping_mul(i) as isize,
                    ) as *const libc::c_void,
            );
        i = i.wrapping_add(1);
        i;
    }
    (*S).outlen = (*P).digest_length as size_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn blake2b_update(
    mut S: *mut blake2b_state,
    mut pin: *const libc::c_void,
    mut inlen: size_t,
) -> libc::c_int {
    let mut in_0: *const libc::c_uchar = pin as *const libc::c_uchar;
    if inlen > 0 as libc::c_int as libc::c_ulong {
        let mut left: size_t = (*S).buflen;
        let mut fill: size_t = (BLAKE2B_BLOCKBYTES as libc::c_int as libc::c_ulong)
            .wrapping_sub(left);
        if inlen > fill {
            (*S).buflen = 0 as libc::c_int as size_t;
            memcpy(
                ((*S).buf).as_mut_ptr().offset(left as isize) as *mut libc::c_void,
                in_0 as *const libc::c_void,
                fill,
            );
            blake2b_increment_counter(S, BLAKE2B_BLOCKBYTES as libc::c_int as uint64_t);
            blake2b_compress(S, ((*S).buf).as_mut_ptr() as *const uint8_t);
            in_0 = in_0.offset(fill as isize);
            inlen = (inlen as libc::c_ulong).wrapping_sub(fill) as size_t as size_t;
            while inlen > BLAKE2B_BLOCKBYTES as libc::c_int as libc::c_ulong {
                blake2b_increment_counter(
                    S,
                    BLAKE2B_BLOCKBYTES as libc::c_int as uint64_t,
                );
                blake2b_compress(S, in_0);
                in_0 = in_0.offset(BLAKE2B_BLOCKBYTES as libc::c_int as isize);
                inlen = (inlen as libc::c_ulong)
                    .wrapping_sub(BLAKE2B_BLOCKBYTES as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
            }
        }
        memcpy(
            ((*S).buf).as_mut_ptr().offset((*S).buflen as isize) as *mut libc::c_void,
            in_0 as *const libc::c_void,
            inlen,
        );
        (*S)
            .buflen = ((*S).buflen as libc::c_ulong).wrapping_add(inlen) as size_t
            as size_t;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn blake2b_final(
    mut S: *mut blake2b_state,
    mut out: *mut libc::c_void,
    mut outlen: size_t,
) -> libc::c_int {
    let mut buffer: [uint8_t; 64] = [
        0 as libc::c_int as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut i: size_t = 0;
    if out.is_null() || outlen < (*S).outlen {
        return -(1 as libc::c_int);
    }
    if blake2b_is_lastblock(S) != 0 {
        return -(1 as libc::c_int);
    }
    blake2b_increment_counter(S, (*S).buflen);
    blake2b_set_lastblock(S);
    memset(
        ((*S).buf).as_mut_ptr().offset((*S).buflen as isize) as *mut libc::c_void,
        0 as libc::c_int,
        (BLAKE2B_BLOCKBYTES as libc::c_int as libc::c_ulong).wrapping_sub((*S).buflen),
    );
    blake2b_compress(S, ((*S).buf).as_mut_ptr() as *const uint8_t);
    i = 0 as libc::c_int as size_t;
    while i < 8 as libc::c_int as libc::c_ulong {
        store64(
            buffer
                .as_mut_ptr()
                .offset(
                    (::core::mem::size_of::<uint64_t>() as libc::c_ulong).wrapping_mul(i)
                        as isize,
                ) as *mut libc::c_void,
            (*S).h[i as usize],
        );
        i = i.wrapping_add(1);
        i;
    }
    memcpy(out, buffer.as_mut_ptr() as *const libc::c_void, (*S).outlen);
    secure_zero_memory(
        buffer.as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn blake2b(
    mut out: *mut libc::c_void,
    mut outlen: size_t,
    mut in_0: *const libc::c_void,
    mut inlen: size_t,
    mut key: *const libc::c_void,
    mut keylen: size_t,
) -> libc::c_int {
    let mut S: [blake2b_state; 1] = [blake2b_state {
        h: [0; 8],
        t: [0; 2],
        f: [0; 2],
        buf: [0; 128],
        buflen: 0,
        outlen: 0,
        last_node: 0,
    }; 1];
    if in_0.is_null() && inlen > 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if out.is_null() {
        return -(1 as libc::c_int);
    }
    if key.is_null() && keylen > 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if outlen == 0 || outlen > BLAKE2B_OUTBYTES as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if keylen > BLAKE2B_KEYBYTES as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if keylen > 0 as libc::c_int as libc::c_ulong {
        if blake2b_init_key(S.as_mut_ptr(), outlen, key, keylen) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    } else if blake2b_init(S.as_mut_ptr(), outlen) < 0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    blake2b_update(S.as_mut_ptr(), in_0 as *const uint8_t as *const libc::c_void, inlen);
    blake2b_final(S.as_mut_ptr(), out, outlen);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn blake2(
    mut out: *mut libc::c_void,
    mut outlen: size_t,
    mut in_0: *const libc::c_void,
    mut inlen: size_t,
    mut key: *const libc::c_void,
    mut keylen: size_t,
) -> libc::c_int {
    return blake2b(out, outlen, in_0, inlen, key, keylen);
}
#[inline]
unsafe extern "C" fn load64(mut src: *const libc::c_void) -> uint64_t {
    let mut w: uint64_t = 0;
    memcpy(
        &mut w as *mut uint64_t as *mut libc::c_void,
        src,
        ::core::mem::size_of::<uint64_t>() as libc::c_ulong,
    );
    return w;
}
#[inline]
unsafe extern "C" fn store32(mut dst: *mut libc::c_void, mut w: uint32_t) {
    memcpy(
        dst,
        &mut w as *mut uint32_t as *const libc::c_void,
        ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
    );
}
#[inline]
unsafe extern "C" fn store64(mut dst: *mut libc::c_void, mut w: uint64_t) {
    memcpy(
        dst,
        &mut w as *mut uint64_t as *const libc::c_void,
        ::core::mem::size_of::<uint64_t>() as libc::c_ulong,
    );
}
#[inline]
unsafe extern "C" fn rotr64(w: uint64_t, c: libc::c_uint) -> uint64_t {
    return w >> c | w << (64 as libc::c_int as libc::c_uint).wrapping_sub(c);
}
#[inline]
unsafe extern "C" fn secure_zero_memory(mut v: *mut libc::c_void, mut n: size_t) {
    static mut memset_v: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int, size_t) -> *mut libc::c_void,
    > = Some(
        memset
            as unsafe extern "C" fn(
                *mut libc::c_void,
                libc::c_int,
                libc::c_ulong,
            ) -> *mut libc::c_void,
    );
    memset_v.expect("non-null function pointer")(v, 0 as libc::c_int, n);
}
static mut blake2b_IV: [uint64_t; 8] = [
    0x6a09e667f3bcc908 as libc::c_ulonglong as uint64_t,
    0xbb67ae8584caa73b as libc::c_ulonglong as uint64_t,
    0x3c6ef372fe94f82b as libc::c_ulonglong as uint64_t,
    0xa54ff53a5f1d36f1 as libc::c_ulonglong as uint64_t,
    0x510e527fade682d1 as libc::c_ulonglong as uint64_t,
    0x9b05688c2b3e6c1f as libc::c_ulonglong as uint64_t,
    0x1f83d9abfb41bd6b as libc::c_ulonglong as uint64_t,
    0x5be0cd19137e2179 as libc::c_ulonglong as uint64_t,
];
static mut blake2b_sigma: [[uint8_t; 16]; 12] = [
    [
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
    ],
    [
        14 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
    ],
    [
        11 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
    ],
    [
        7 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
    ],
    [
        9 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
    ],
    [
        2 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
    ],
    [
        12 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
    ],
    [
        13 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
    ],
    [
        6 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
    ],
    [
        10 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    ],
    [
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
    ],
    [
        14 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
    ],
];
unsafe extern "C" fn blake2b_set_lastnode(mut S: *mut blake2b_state) {
    (*S).f[1 as libc::c_int as usize] = -(1 as libc::c_int) as uint64_t;
}
unsafe extern "C" fn blake2b_is_lastblock(mut S: *const blake2b_state) -> libc::c_int {
    return ((*S).f[0 as libc::c_int as usize] != 0 as libc::c_int as libc::c_ulong)
        as libc::c_int;
}
unsafe extern "C" fn blake2b_set_lastblock(mut S: *mut blake2b_state) {
    if (*S).last_node != 0 {
        blake2b_set_lastnode(S);
    }
    (*S).f[0 as libc::c_int as usize] = -(1 as libc::c_int) as uint64_t;
}
unsafe extern "C" fn blake2b_increment_counter(
    mut S: *mut blake2b_state,
    inc: uint64_t,
) {
    (*S)
        .t[0 as libc::c_int
        as usize] = ((*S).t[0 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_add(inc) as uint64_t as uint64_t;
    (*S)
        .t[1 as libc::c_int
        as usize] = ((*S).t[1 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_add(
            ((*S).t[0 as libc::c_int as usize] < inc) as libc::c_int as libc::c_ulong,
        ) as uint64_t as uint64_t;
}
unsafe extern "C" fn blake2b_init0(mut S: *mut blake2b_state) {
    let mut i: size_t = 0;
    memset(
        S as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<blake2b_state>() as libc::c_ulong,
    );
    i = 0 as libc::c_int as size_t;
    while i < 8 as libc::c_int as libc::c_ulong {
        (*S).h[i as usize] = blake2b_IV[i as usize];
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn blake2b_compress(
    mut S: *mut blake2b_state,
    mut block: *const uint8_t,
) {
    let mut m: [uint64_t; 16] = [0; 16];
    let mut v: [uint64_t; 16] = [0; 16];
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < 16 as libc::c_int as libc::c_ulong {
        m[i
            as usize] = load64(
            block
                .offset(
                    i.wrapping_mul(::core::mem::size_of::<uint64_t>() as libc::c_ulong)
                        as isize,
                ) as *const libc::c_void,
        );
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < 8 as libc::c_int as libc::c_ulong {
        v[i as usize] = (*S).h[i as usize];
        i = i.wrapping_add(1);
        i;
    }
    v[8 as libc::c_int as usize] = blake2b_IV[0 as libc::c_int as usize];
    v[9 as libc::c_int as usize] = blake2b_IV[1 as libc::c_int as usize];
    v[10 as libc::c_int as usize] = blake2b_IV[2 as libc::c_int as usize];
    v[11 as libc::c_int as usize] = blake2b_IV[3 as libc::c_int as usize];
    v[12 as libc::c_int
        as usize] = blake2b_IV[4 as libc::c_int as usize]
        ^ (*S).t[0 as libc::c_int as usize];
    v[13 as libc::c_int
        as usize] = blake2b_IV[5 as libc::c_int as usize]
        ^ (*S).t[1 as libc::c_int as usize];
    v[14 as libc::c_int
        as usize] = blake2b_IV[6 as libc::c_int as usize]
        ^ (*S).f[0 as libc::c_int as usize];
    v[15 as libc::c_int
        as usize] = blake2b_IV[7 as libc::c_int as usize]
        ^ (*S).f[1 as libc::c_int as usize];
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[0 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[0 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[0 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[0 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[0 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[0 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[0 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[0 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[0 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[0 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[0 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[0 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[0 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[0 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[0 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[0 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[1 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[1 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[1 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[1 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[1 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[1 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[1 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[1 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[1 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[1 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[1 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[1 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[1 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[1 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[1 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[1 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[2 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[2 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[2 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[2 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[2 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[2 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[2 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[2 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[2 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[2 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[2 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[2 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[2 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[2 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[2 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[2 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[3 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[3 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[3 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[3 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[3 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[3 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[3 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[3 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[3 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[3 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[3 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[3 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[3 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[3 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[3 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[3 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[4 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[4 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[4 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[4 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[4 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[4 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[4 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[4 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[4 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[4 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[4 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[4 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[4 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[4 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[4 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[4 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[5 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[5 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[5 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[5 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[5 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[5 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[5 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[5 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[5 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[5 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[5 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[5 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[5 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[5 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[5 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[5 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[6 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[6 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[6 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[6 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[6 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[6 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[6 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[6 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[6 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[6 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[6 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[6 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[6 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[6 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[6 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[6 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[7 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[7 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[7 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[7 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[7 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[7 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[7 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[7 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[7 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[7 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[7 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[7 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[7 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[7 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[7 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[7 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[8 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[8 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[8 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[8 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[8 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[8 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[8 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[8 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[8 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[8 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[8 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[8 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[8 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[8 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[8 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[8 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[9 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[9 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[9 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[9 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[9 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[9 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[9 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[9 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[9 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[9 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[9 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[9 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[9 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[9 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[9 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[9 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[10 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[10 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[10 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[10 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[10 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[10 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[10 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[10 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[10 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[10 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[10 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[10 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[10 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[10 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[10 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[10 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[11 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[11 as libc::c_int
                as usize][(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[11 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[11 as libc::c_int
                as usize][(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[11 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[11 as libc::c_int
                as usize][(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[11 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[11 as libc::c_int
                as usize][(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[11 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int
        as usize] = (v[0 as libc::c_int as usize])
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[11 as libc::c_int
                as usize][(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[15 as libc::c_int
        as usize] = rotr64(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int
        as usize] = (v[10 as libc::c_int as usize])
        .wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int
        as usize] = rotr64(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[11 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int
        as usize] = (v[1 as libc::c_int as usize])
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[11 as libc::c_int
                as usize][(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[12 as libc::c_int
        as usize] = rotr64(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int
        as usize] = (v[11 as libc::c_int as usize])
        .wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int
        as usize] = rotr64(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[11 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize])
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[11 as libc::c_int
                as usize][(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[13 as libc::c_int
        as usize] = rotr64(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int
        as usize] = (v[8 as libc::c_int as usize])
        .wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int
        as usize] = rotr64(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[11 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        32 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        24 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int
        as usize] = (v[3 as libc::c_int as usize])
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2b_sigma[11 as libc::c_int
                as usize][(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int)
                as usize] as usize],
        );
    v[14 as libc::c_int
        as usize] = rotr64(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int
        as usize] = (v[9 as libc::c_int as usize])
        .wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int
        as usize] = rotr64(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        63 as libc::c_int as libc::c_uint,
    );
    i = 0 as libc::c_int as size_t;
    while i < 8 as libc::c_int as libc::c_ulong {
        (*S)
            .h[i
            as usize] = (*S).h[i as usize] ^ v[i as usize]
            ^ v[i.wrapping_add(8 as libc::c_int as libc::c_ulong) as usize];
        i = i.wrapping_add(1);
        i;
    }
}
