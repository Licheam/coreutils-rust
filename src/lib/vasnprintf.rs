use core::arch::asm;
use num_traits::{Float, ToPrimitive};
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn nl_langinfo(__item: nl_item) -> *mut libc::c_char;
    fn printf_fetchargs(args: ::core::ffi::VaList, a: *mut arguments) -> libc::c_int;
    fn printf_parse(
        format: *const libc::c_char,
        d: *mut char_directives,
        a: *mut arguments,
    ) -> libc::c_int;
    fn frexpl(_: f128::f128, _: *mut libc::c_int) -> f128::f128;
    fn frexp(_: libc::c_double, _: *mut libc::c_int) -> libc::c_double;
    fn printf_frexp(x: libc::c_double, expptr: *mut libc::c_int) -> libc::c_double;
    fn printf_frexpl(x: f128::f128, expptr: *mut libc::c_int) -> f128::f128;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argument {
    pub type_0: arg_type,
    pub a: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub a_schar: libc::c_schar,
    pub a_uchar: libc::c_uchar,
    pub a_short: libc::c_short,
    pub a_ushort: libc::c_ushort,
    pub a_int: libc::c_int,
    pub a_uint: libc::c_uint,
    pub a_longint: libc::c_long,
    pub a_ulongint: libc::c_ulong,
    pub a_longlongint: libc::c_longlong,
    pub a_ulonglongint: libc::c_ulonglong,
    pub a_float: libc::c_float,
    pub a_double: libc::c_double,
    pub a_longdouble: f128::f128,
    pub a_char: libc::c_int,
    pub a_wide_char: wint_t,
    pub a_string: *const libc::c_char,
    pub a_wide_string: *const wchar_t,
    pub a_pointer: *mut libc::c_void,
    pub a_count_schar_pointer: *mut libc::c_schar,
    pub a_count_short_pointer: *mut libc::c_short,
    pub a_count_int_pointer: *mut libc::c_int,
    pub a_count_longint_pointer: *mut libc::c_long,
    pub a_count_longlongint_pointer: *mut libc::c_longlong,
}
pub type wint_t = libc::c_uint;
pub type arg_type = libc::c_uint;
pub const TYPE_COUNT_LONGLONGINT_POINTER: arg_type = 22;
pub const TYPE_COUNT_LONGINT_POINTER: arg_type = 21;
pub const TYPE_COUNT_INT_POINTER: arg_type = 20;
pub const TYPE_COUNT_SHORT_POINTER: arg_type = 19;
pub const TYPE_COUNT_SCHAR_POINTER: arg_type = 18;
pub const TYPE_POINTER: arg_type = 17;
pub const TYPE_WIDE_STRING: arg_type = 16;
pub const TYPE_STRING: arg_type = 15;
pub const TYPE_WIDE_CHAR: arg_type = 14;
pub const TYPE_CHAR: arg_type = 13;
pub const TYPE_LONGDOUBLE: arg_type = 12;
pub const TYPE_DOUBLE: arg_type = 11;
pub const TYPE_ULONGLONGINT: arg_type = 10;
pub const TYPE_LONGLONGINT: arg_type = 9;
pub const TYPE_ULONGINT: arg_type = 8;
pub const TYPE_LONGINT: arg_type = 7;
pub const TYPE_UINT: arg_type = 6;
pub const TYPE_INT: arg_type = 5;
pub const TYPE_USHORT: arg_type = 4;
pub const TYPE_SHORT: arg_type = 3;
pub const TYPE_UCHAR: arg_type = 2;
pub const TYPE_SCHAR: arg_type = 1;
pub const TYPE_NONE: arg_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arguments {
    pub count: size_t,
    pub arg: *mut argument,
    pub direct_alloc_arg: [argument; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct char_directive {
    pub dir_start: *const libc::c_char,
    pub dir_end: *const libc::c_char,
    pub flags: libc::c_int,
    pub width_start: *const libc::c_char,
    pub width_end: *const libc::c_char,
    pub width_arg_index: size_t,
    pub precision_start: *const libc::c_char,
    pub precision_end: *const libc::c_char,
    pub precision_arg_index: size_t,
    pub conversion: libc::c_char,
    pub arg_index: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct char_directives {
    pub count: size_t,
    pub dir: *mut char_directive,
    pub max_width_length: size_t,
    pub max_precision_length: size_t,
    pub direct_alloc_dir: [char_directive; 7],
}
pub const RADIXCHAR: C2RustUnnamed_0 = 65536;
pub type nl_item = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpn_t {
    pub nlimbs: size_t,
    pub limbs: *mut mp_limb_t,
}
pub type mp_limb_t = libc::c_uint;
pub type mp_twolimb_t = libc::c_ulonglong;
pub type fpucw_t = libc::c_ushort;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _NL_NUM: C2RustUnnamed_0 = 786449;
pub const _NL_NUM_LC_IDENTIFICATION: C2RustUnnamed_0 = 786448;
pub const _NL_IDENTIFICATION_CODESET: C2RustUnnamed_0 = 786447;
pub const _NL_IDENTIFICATION_CATEGORY: C2RustUnnamed_0 = 786446;
pub const _NL_IDENTIFICATION_DATE: C2RustUnnamed_0 = 786445;
pub const _NL_IDENTIFICATION_REVISION: C2RustUnnamed_0 = 786444;
pub const _NL_IDENTIFICATION_ABBREVIATION: C2RustUnnamed_0 = 786443;
pub const _NL_IDENTIFICATION_APPLICATION: C2RustUnnamed_0 = 786442;
pub const _NL_IDENTIFICATION_AUDIENCE: C2RustUnnamed_0 = 786441;
pub const _NL_IDENTIFICATION_TERRITORY: C2RustUnnamed_0 = 786440;
pub const _NL_IDENTIFICATION_LANGUAGE: C2RustUnnamed_0 = 786439;
pub const _NL_IDENTIFICATION_FAX: C2RustUnnamed_0 = 786438;
pub const _NL_IDENTIFICATION_TEL: C2RustUnnamed_0 = 786437;
pub const _NL_IDENTIFICATION_EMAIL: C2RustUnnamed_0 = 786436;
pub const _NL_IDENTIFICATION_CONTACT: C2RustUnnamed_0 = 786435;
pub const _NL_IDENTIFICATION_ADDRESS: C2RustUnnamed_0 = 786434;
pub const _NL_IDENTIFICATION_SOURCE: C2RustUnnamed_0 = 786433;
pub const _NL_IDENTIFICATION_TITLE: C2RustUnnamed_0 = 786432;
pub const _NL_NUM_LC_MEASUREMENT: C2RustUnnamed_0 = 720898;
pub const _NL_MEASUREMENT_CODESET: C2RustUnnamed_0 = 720897;
pub const _NL_MEASUREMENT_MEASUREMENT: C2RustUnnamed_0 = 720896;
pub const _NL_NUM_LC_TELEPHONE: C2RustUnnamed_0 = 655365;
pub const _NL_TELEPHONE_CODESET: C2RustUnnamed_0 = 655364;
pub const _NL_TELEPHONE_INT_PREFIX: C2RustUnnamed_0 = 655363;
pub const _NL_TELEPHONE_INT_SELECT: C2RustUnnamed_0 = 655362;
pub const _NL_TELEPHONE_TEL_DOM_FMT: C2RustUnnamed_0 = 655361;
pub const _NL_TELEPHONE_TEL_INT_FMT: C2RustUnnamed_0 = 655360;
pub const _NL_NUM_LC_ADDRESS: C2RustUnnamed_0 = 589837;
pub const _NL_ADDRESS_CODESET: C2RustUnnamed_0 = 589836;
pub const _NL_ADDRESS_LANG_LIB: C2RustUnnamed_0 = 589835;
pub const _NL_ADDRESS_LANG_TERM: C2RustUnnamed_0 = 589834;
pub const _NL_ADDRESS_LANG_AB: C2RustUnnamed_0 = 589833;
pub const _NL_ADDRESS_LANG_NAME: C2RustUnnamed_0 = 589832;
pub const _NL_ADDRESS_COUNTRY_ISBN: C2RustUnnamed_0 = 589831;
pub const _NL_ADDRESS_COUNTRY_NUM: C2RustUnnamed_0 = 589830;
pub const _NL_ADDRESS_COUNTRY_CAR: C2RustUnnamed_0 = 589829;
pub const _NL_ADDRESS_COUNTRY_AB3: C2RustUnnamed_0 = 589828;
pub const _NL_ADDRESS_COUNTRY_AB2: C2RustUnnamed_0 = 589827;
pub const _NL_ADDRESS_COUNTRY_POST: C2RustUnnamed_0 = 589826;
pub const _NL_ADDRESS_COUNTRY_NAME: C2RustUnnamed_0 = 589825;
pub const _NL_ADDRESS_POSTAL_FMT: C2RustUnnamed_0 = 589824;
pub const _NL_NUM_LC_NAME: C2RustUnnamed_0 = 524295;
pub const _NL_NAME_CODESET: C2RustUnnamed_0 = 524294;
pub const _NL_NAME_NAME_MS: C2RustUnnamed_0 = 524293;
pub const _NL_NAME_NAME_MISS: C2RustUnnamed_0 = 524292;
pub const _NL_NAME_NAME_MRS: C2RustUnnamed_0 = 524291;
pub const _NL_NAME_NAME_MR: C2RustUnnamed_0 = 524290;
pub const _NL_NAME_NAME_GEN: C2RustUnnamed_0 = 524289;
pub const _NL_NAME_NAME_FMT: C2RustUnnamed_0 = 524288;
pub const _NL_NUM_LC_PAPER: C2RustUnnamed_0 = 458755;
pub const _NL_PAPER_CODESET: C2RustUnnamed_0 = 458754;
pub const _NL_PAPER_WIDTH: C2RustUnnamed_0 = 458753;
pub const _NL_PAPER_HEIGHT: C2RustUnnamed_0 = 458752;
pub const _NL_NUM_LC_MESSAGES: C2RustUnnamed_0 = 327685;
pub const _NL_MESSAGES_CODESET: C2RustUnnamed_0 = 327684;
pub const __NOSTR: C2RustUnnamed_0 = 327683;
pub const __YESSTR: C2RustUnnamed_0 = 327682;
pub const __NOEXPR: C2RustUnnamed_0 = 327681;
pub const __YESEXPR: C2RustUnnamed_0 = 327680;
pub const _NL_NUM_LC_NUMERIC: C2RustUnnamed_0 = 65542;
pub const _NL_NUMERIC_CODESET: C2RustUnnamed_0 = 65541;
pub const _NL_NUMERIC_THOUSANDS_SEP_WC: C2RustUnnamed_0 = 65540;
pub const _NL_NUMERIC_DECIMAL_POINT_WC: C2RustUnnamed_0 = 65539;
pub const __GROUPING: C2RustUnnamed_0 = 65538;
pub const THOUSEP: C2RustUnnamed_0 = 65537;
pub const __THOUSANDS_SEP: C2RustUnnamed_0 = 65537;
pub const __DECIMAL_POINT: C2RustUnnamed_0 = 65536;
pub const _NL_NUM_LC_MONETARY: C2RustUnnamed_0 = 262190;
pub const _NL_MONETARY_CODESET: C2RustUnnamed_0 = 262189;
pub const _NL_MONETARY_THOUSANDS_SEP_WC: C2RustUnnamed_0 = 262188;
pub const _NL_MONETARY_DECIMAL_POINT_WC: C2RustUnnamed_0 = 262187;
pub const _NL_MONETARY_CONVERSION_RATE: C2RustUnnamed_0 = 262186;
pub const _NL_MONETARY_DUO_VALID_TO: C2RustUnnamed_0 = 262185;
pub const _NL_MONETARY_DUO_VALID_FROM: C2RustUnnamed_0 = 262184;
pub const _NL_MONETARY_UNO_VALID_TO: C2RustUnnamed_0 = 262183;
pub const _NL_MONETARY_UNO_VALID_FROM: C2RustUnnamed_0 = 262182;
pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: C2RustUnnamed_0 = 262181;
pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: C2RustUnnamed_0 = 262180;
pub const _NL_MONETARY_DUO_N_SIGN_POSN: C2RustUnnamed_0 = 262179;
pub const _NL_MONETARY_DUO_P_SIGN_POSN: C2RustUnnamed_0 = 262178;
pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: C2RustUnnamed_0 = 262177;
pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: C2RustUnnamed_0 = 262176;
pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: C2RustUnnamed_0 = 262175;
pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: C2RustUnnamed_0 = 262174;
pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: C2RustUnnamed_0 = 262173;
pub const _NL_MONETARY_DUO_N_CS_PRECEDES: C2RustUnnamed_0 = 262172;
pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: C2RustUnnamed_0 = 262171;
pub const _NL_MONETARY_DUO_P_CS_PRECEDES: C2RustUnnamed_0 = 262170;
pub const _NL_MONETARY_DUO_FRAC_DIGITS: C2RustUnnamed_0 = 262169;
pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: C2RustUnnamed_0 = 262168;
pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: C2RustUnnamed_0 = 262167;
pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: C2RustUnnamed_0 = 262166;
pub const __INT_N_SIGN_POSN: C2RustUnnamed_0 = 262165;
pub const __INT_P_SIGN_POSN: C2RustUnnamed_0 = 262164;
pub const __INT_N_SEP_BY_SPACE: C2RustUnnamed_0 = 262163;
pub const __INT_N_CS_PRECEDES: C2RustUnnamed_0 = 262162;
pub const __INT_P_SEP_BY_SPACE: C2RustUnnamed_0 = 262161;
pub const __INT_P_CS_PRECEDES: C2RustUnnamed_0 = 262160;
pub const _NL_MONETARY_CRNCYSTR: C2RustUnnamed_0 = 262159;
pub const __N_SIGN_POSN: C2RustUnnamed_0 = 262158;
pub const __P_SIGN_POSN: C2RustUnnamed_0 = 262157;
pub const __N_SEP_BY_SPACE: C2RustUnnamed_0 = 262156;
pub const __N_CS_PRECEDES: C2RustUnnamed_0 = 262155;
pub const __P_SEP_BY_SPACE: C2RustUnnamed_0 = 262154;
pub const __P_CS_PRECEDES: C2RustUnnamed_0 = 262153;
pub const __FRAC_DIGITS: C2RustUnnamed_0 = 262152;
pub const __INT_FRAC_DIGITS: C2RustUnnamed_0 = 262151;
pub const __NEGATIVE_SIGN: C2RustUnnamed_0 = 262150;
pub const __POSITIVE_SIGN: C2RustUnnamed_0 = 262149;
pub const __MON_GROUPING: C2RustUnnamed_0 = 262148;
pub const __MON_THOUSANDS_SEP: C2RustUnnamed_0 = 262147;
pub const __MON_DECIMAL_POINT: C2RustUnnamed_0 = 262146;
pub const __CURRENCY_SYMBOL: C2RustUnnamed_0 = 262145;
pub const __INT_CURR_SYMBOL: C2RustUnnamed_0 = 262144;
pub const _NL_NUM_LC_CTYPE: C2RustUnnamed_0 = 86;
pub const _NL_CTYPE_EXTRA_MAP_14: C2RustUnnamed_0 = 85;
pub const _NL_CTYPE_EXTRA_MAP_13: C2RustUnnamed_0 = 84;
pub const _NL_CTYPE_EXTRA_MAP_12: C2RustUnnamed_0 = 83;
pub const _NL_CTYPE_EXTRA_MAP_11: C2RustUnnamed_0 = 82;
pub const _NL_CTYPE_EXTRA_MAP_10: C2RustUnnamed_0 = 81;
pub const _NL_CTYPE_EXTRA_MAP_9: C2RustUnnamed_0 = 80;
pub const _NL_CTYPE_EXTRA_MAP_8: C2RustUnnamed_0 = 79;
pub const _NL_CTYPE_EXTRA_MAP_7: C2RustUnnamed_0 = 78;
pub const _NL_CTYPE_EXTRA_MAP_6: C2RustUnnamed_0 = 77;
pub const _NL_CTYPE_EXTRA_MAP_5: C2RustUnnamed_0 = 76;
pub const _NL_CTYPE_EXTRA_MAP_4: C2RustUnnamed_0 = 75;
pub const _NL_CTYPE_EXTRA_MAP_3: C2RustUnnamed_0 = 74;
pub const _NL_CTYPE_EXTRA_MAP_2: C2RustUnnamed_0 = 73;
pub const _NL_CTYPE_EXTRA_MAP_1: C2RustUnnamed_0 = 72;
pub const _NL_CTYPE_NONASCII_CASE: C2RustUnnamed_0 = 71;
pub const _NL_CTYPE_MAP_TO_NONASCII: C2RustUnnamed_0 = 70;
pub const _NL_CTYPE_TRANSLIT_IGNORE: C2RustUnnamed_0 = 69;
pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: C2RustUnnamed_0 = 68;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: C2RustUnnamed_0 = 67;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: C2RustUnnamed_0 = 66;
pub const _NL_CTYPE_TRANSLIT_TO_TBL: C2RustUnnamed_0 = 65;
pub const _NL_CTYPE_TRANSLIT_TO_IDX: C2RustUnnamed_0 = 64;
pub const _NL_CTYPE_TRANSLIT_FROM_TBL: C2RustUnnamed_0 = 63;
pub const _NL_CTYPE_TRANSLIT_FROM_IDX: C2RustUnnamed_0 = 62;
pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: C2RustUnnamed_0 = 61;
pub const _NL_CTYPE_OUTDIGIT9_WC: C2RustUnnamed_0 = 60;
pub const _NL_CTYPE_OUTDIGIT8_WC: C2RustUnnamed_0 = 59;
pub const _NL_CTYPE_OUTDIGIT7_WC: C2RustUnnamed_0 = 58;
pub const _NL_CTYPE_OUTDIGIT6_WC: C2RustUnnamed_0 = 57;
pub const _NL_CTYPE_OUTDIGIT5_WC: C2RustUnnamed_0 = 56;
pub const _NL_CTYPE_OUTDIGIT4_WC: C2RustUnnamed_0 = 55;
pub const _NL_CTYPE_OUTDIGIT3_WC: C2RustUnnamed_0 = 54;
pub const _NL_CTYPE_OUTDIGIT2_WC: C2RustUnnamed_0 = 53;
pub const _NL_CTYPE_OUTDIGIT1_WC: C2RustUnnamed_0 = 52;
pub const _NL_CTYPE_OUTDIGIT0_WC: C2RustUnnamed_0 = 51;
pub const _NL_CTYPE_OUTDIGIT9_MB: C2RustUnnamed_0 = 50;
pub const _NL_CTYPE_OUTDIGIT8_MB: C2RustUnnamed_0 = 49;
pub const _NL_CTYPE_OUTDIGIT7_MB: C2RustUnnamed_0 = 48;
pub const _NL_CTYPE_OUTDIGIT6_MB: C2RustUnnamed_0 = 47;
pub const _NL_CTYPE_OUTDIGIT5_MB: C2RustUnnamed_0 = 46;
pub const _NL_CTYPE_OUTDIGIT4_MB: C2RustUnnamed_0 = 45;
pub const _NL_CTYPE_OUTDIGIT3_MB: C2RustUnnamed_0 = 44;
pub const _NL_CTYPE_OUTDIGIT2_MB: C2RustUnnamed_0 = 43;
pub const _NL_CTYPE_OUTDIGIT1_MB: C2RustUnnamed_0 = 42;
pub const _NL_CTYPE_OUTDIGIT0_MB: C2RustUnnamed_0 = 41;
pub const _NL_CTYPE_INDIGITS9_WC: C2RustUnnamed_0 = 40;
pub const _NL_CTYPE_INDIGITS8_WC: C2RustUnnamed_0 = 39;
pub const _NL_CTYPE_INDIGITS7_WC: C2RustUnnamed_0 = 38;
pub const _NL_CTYPE_INDIGITS6_WC: C2RustUnnamed_0 = 37;
pub const _NL_CTYPE_INDIGITS5_WC: C2RustUnnamed_0 = 36;
pub const _NL_CTYPE_INDIGITS4_WC: C2RustUnnamed_0 = 35;
pub const _NL_CTYPE_INDIGITS3_WC: C2RustUnnamed_0 = 34;
pub const _NL_CTYPE_INDIGITS2_WC: C2RustUnnamed_0 = 33;
pub const _NL_CTYPE_INDIGITS1_WC: C2RustUnnamed_0 = 32;
pub const _NL_CTYPE_INDIGITS0_WC: C2RustUnnamed_0 = 31;
pub const _NL_CTYPE_INDIGITS_WC_LEN: C2RustUnnamed_0 = 30;
pub const _NL_CTYPE_INDIGITS9_MB: C2RustUnnamed_0 = 29;
pub const _NL_CTYPE_INDIGITS8_MB: C2RustUnnamed_0 = 28;
pub const _NL_CTYPE_INDIGITS7_MB: C2RustUnnamed_0 = 27;
pub const _NL_CTYPE_INDIGITS6_MB: C2RustUnnamed_0 = 26;
pub const _NL_CTYPE_INDIGITS5_MB: C2RustUnnamed_0 = 25;
pub const _NL_CTYPE_INDIGITS4_MB: C2RustUnnamed_0 = 24;
pub const _NL_CTYPE_INDIGITS3_MB: C2RustUnnamed_0 = 23;
pub const _NL_CTYPE_INDIGITS2_MB: C2RustUnnamed_0 = 22;
pub const _NL_CTYPE_INDIGITS1_MB: C2RustUnnamed_0 = 21;
pub const _NL_CTYPE_INDIGITS0_MB: C2RustUnnamed_0 = 20;
pub const _NL_CTYPE_INDIGITS_MB_LEN: C2RustUnnamed_0 = 19;
pub const _NL_CTYPE_MAP_OFFSET: C2RustUnnamed_0 = 18;
pub const _NL_CTYPE_CLASS_OFFSET: C2RustUnnamed_0 = 17;
pub const _NL_CTYPE_TOLOWER32: C2RustUnnamed_0 = 16;
pub const _NL_CTYPE_TOUPPER32: C2RustUnnamed_0 = 15;
pub const CODESET: C2RustUnnamed_0 = 14;
pub const _NL_CTYPE_CODESET_NAME: C2RustUnnamed_0 = 14;
pub const _NL_CTYPE_MB_CUR_MAX: C2RustUnnamed_0 = 13;
pub const _NL_CTYPE_WIDTH: C2RustUnnamed_0 = 12;
pub const _NL_CTYPE_MAP_NAMES: C2RustUnnamed_0 = 11;
pub const _NL_CTYPE_CLASS_NAMES: C2RustUnnamed_0 = 10;
pub const _NL_CTYPE_GAP6: C2RustUnnamed_0 = 9;
pub const _NL_CTYPE_GAP5: C2RustUnnamed_0 = 8;
pub const _NL_CTYPE_GAP4: C2RustUnnamed_0 = 7;
pub const _NL_CTYPE_GAP3: C2RustUnnamed_0 = 6;
pub const _NL_CTYPE_CLASS32: C2RustUnnamed_0 = 5;
pub const _NL_CTYPE_GAP2: C2RustUnnamed_0 = 4;
pub const _NL_CTYPE_TOLOWER: C2RustUnnamed_0 = 3;
pub const _NL_CTYPE_GAP1: C2RustUnnamed_0 = 2;
pub const _NL_CTYPE_TOUPPER: C2RustUnnamed_0 = 1;
pub const _NL_CTYPE_CLASS: C2RustUnnamed_0 = 0;
pub const _NL_NUM_LC_COLLATE: C2RustUnnamed_0 = 196627;
pub const _NL_COLLATE_CODESET: C2RustUnnamed_0 = 196626;
pub const _NL_COLLATE_COLLSEQWC: C2RustUnnamed_0 = 196625;
pub const _NL_COLLATE_COLLSEQMB: C2RustUnnamed_0 = 196624;
pub const _NL_COLLATE_SYMB_EXTRAMB: C2RustUnnamed_0 = 196623;
pub const _NL_COLLATE_SYMB_TABLEMB: C2RustUnnamed_0 = 196622;
pub const _NL_COLLATE_SYMB_HASH_SIZEMB: C2RustUnnamed_0 = 196621;
pub const _NL_COLLATE_INDIRECTWC: C2RustUnnamed_0 = 196620;
pub const _NL_COLLATE_EXTRAWC: C2RustUnnamed_0 = 196619;
pub const _NL_COLLATE_WEIGHTWC: C2RustUnnamed_0 = 196618;
pub const _NL_COLLATE_TABLEWC: C2RustUnnamed_0 = 196617;
pub const _NL_COLLATE_GAP3: C2RustUnnamed_0 = 196616;
pub const _NL_COLLATE_GAP2: C2RustUnnamed_0 = 196615;
pub const _NL_COLLATE_GAP1: C2RustUnnamed_0 = 196614;
pub const _NL_COLLATE_INDIRECTMB: C2RustUnnamed_0 = 196613;
pub const _NL_COLLATE_EXTRAMB: C2RustUnnamed_0 = 196612;
pub const _NL_COLLATE_WEIGHTMB: C2RustUnnamed_0 = 196611;
pub const _NL_COLLATE_TABLEMB: C2RustUnnamed_0 = 196610;
pub const _NL_COLLATE_RULESETS: C2RustUnnamed_0 = 196609;
pub const _NL_COLLATE_NRULES: C2RustUnnamed_0 = 196608;
pub const _NL_NUM_LC_TIME: C2RustUnnamed_0 = 131231;
pub const _NL_WABALTMON_12: C2RustUnnamed_0 = 131230;
pub const _NL_WABALTMON_11: C2RustUnnamed_0 = 131229;
pub const _NL_WABALTMON_10: C2RustUnnamed_0 = 131228;
pub const _NL_WABALTMON_9: C2RustUnnamed_0 = 131227;
pub const _NL_WABALTMON_8: C2RustUnnamed_0 = 131226;
pub const _NL_WABALTMON_7: C2RustUnnamed_0 = 131225;
pub const _NL_WABALTMON_6: C2RustUnnamed_0 = 131224;
pub const _NL_WABALTMON_5: C2RustUnnamed_0 = 131223;
pub const _NL_WABALTMON_4: C2RustUnnamed_0 = 131222;
pub const _NL_WABALTMON_3: C2RustUnnamed_0 = 131221;
pub const _NL_WABALTMON_2: C2RustUnnamed_0 = 131220;
pub const _NL_WABALTMON_1: C2RustUnnamed_0 = 131219;
pub const _NL_ABALTMON_12: C2RustUnnamed_0 = 131218;
pub const _NL_ABALTMON_11: C2RustUnnamed_0 = 131217;
pub const _NL_ABALTMON_10: C2RustUnnamed_0 = 131216;
pub const _NL_ABALTMON_9: C2RustUnnamed_0 = 131215;
pub const _NL_ABALTMON_8: C2RustUnnamed_0 = 131214;
pub const _NL_ABALTMON_7: C2RustUnnamed_0 = 131213;
pub const _NL_ABALTMON_6: C2RustUnnamed_0 = 131212;
pub const _NL_ABALTMON_5: C2RustUnnamed_0 = 131211;
pub const _NL_ABALTMON_4: C2RustUnnamed_0 = 131210;
pub const _NL_ABALTMON_3: C2RustUnnamed_0 = 131209;
pub const _NL_ABALTMON_2: C2RustUnnamed_0 = 131208;
pub const _NL_ABALTMON_1: C2RustUnnamed_0 = 131207;
pub const _NL_WALTMON_12: C2RustUnnamed_0 = 131206;
pub const _NL_WALTMON_11: C2RustUnnamed_0 = 131205;
pub const _NL_WALTMON_10: C2RustUnnamed_0 = 131204;
pub const _NL_WALTMON_9: C2RustUnnamed_0 = 131203;
pub const _NL_WALTMON_8: C2RustUnnamed_0 = 131202;
pub const _NL_WALTMON_7: C2RustUnnamed_0 = 131201;
pub const _NL_WALTMON_6: C2RustUnnamed_0 = 131200;
pub const _NL_WALTMON_5: C2RustUnnamed_0 = 131199;
pub const _NL_WALTMON_4: C2RustUnnamed_0 = 131198;
pub const _NL_WALTMON_3: C2RustUnnamed_0 = 131197;
pub const _NL_WALTMON_2: C2RustUnnamed_0 = 131196;
pub const _NL_WALTMON_1: C2RustUnnamed_0 = 131195;
pub const __ALTMON_12: C2RustUnnamed_0 = 131194;
pub const __ALTMON_11: C2RustUnnamed_0 = 131193;
pub const __ALTMON_10: C2RustUnnamed_0 = 131192;
pub const __ALTMON_9: C2RustUnnamed_0 = 131191;
pub const __ALTMON_8: C2RustUnnamed_0 = 131190;
pub const __ALTMON_7: C2RustUnnamed_0 = 131189;
pub const __ALTMON_6: C2RustUnnamed_0 = 131188;
pub const __ALTMON_5: C2RustUnnamed_0 = 131187;
pub const __ALTMON_4: C2RustUnnamed_0 = 131186;
pub const __ALTMON_3: C2RustUnnamed_0 = 131185;
pub const __ALTMON_2: C2RustUnnamed_0 = 131184;
pub const __ALTMON_1: C2RustUnnamed_0 = 131183;
pub const _NL_TIME_CODESET: C2RustUnnamed_0 = 131182;
pub const _NL_W_DATE_FMT: C2RustUnnamed_0 = 131181;
pub const _DATE_FMT: C2RustUnnamed_0 = 131180;
pub const _NL_TIME_TIMEZONE: C2RustUnnamed_0 = 131179;
pub const _NL_TIME_CAL_DIRECTION: C2RustUnnamed_0 = 131178;
pub const _NL_TIME_FIRST_WORKDAY: C2RustUnnamed_0 = 131177;
pub const _NL_TIME_FIRST_WEEKDAY: C2RustUnnamed_0 = 131176;
pub const _NL_TIME_WEEK_1STWEEK: C2RustUnnamed_0 = 131175;
pub const _NL_TIME_WEEK_1STDAY: C2RustUnnamed_0 = 131174;
pub const _NL_TIME_WEEK_NDAYS: C2RustUnnamed_0 = 131173;
pub const _NL_WERA_T_FMT: C2RustUnnamed_0 = 131172;
pub const _NL_WERA_D_T_FMT: C2RustUnnamed_0 = 131171;
pub const _NL_WALT_DIGITS: C2RustUnnamed_0 = 131170;
pub const _NL_WERA_D_FMT: C2RustUnnamed_0 = 131169;
pub const _NL_WERA_YEAR: C2RustUnnamed_0 = 131168;
pub const _NL_WT_FMT_AMPM: C2RustUnnamed_0 = 131167;
pub const _NL_WT_FMT: C2RustUnnamed_0 = 131166;
pub const _NL_WD_FMT: C2RustUnnamed_0 = 131165;
pub const _NL_WD_T_FMT: C2RustUnnamed_0 = 131164;
pub const _NL_WPM_STR: C2RustUnnamed_0 = 131163;
pub const _NL_WAM_STR: C2RustUnnamed_0 = 131162;
pub const _NL_WMON_12: C2RustUnnamed_0 = 131161;
pub const _NL_WMON_11: C2RustUnnamed_0 = 131160;
pub const _NL_WMON_10: C2RustUnnamed_0 = 131159;
pub const _NL_WMON_9: C2RustUnnamed_0 = 131158;
pub const _NL_WMON_8: C2RustUnnamed_0 = 131157;
pub const _NL_WMON_7: C2RustUnnamed_0 = 131156;
pub const _NL_WMON_6: C2RustUnnamed_0 = 131155;
pub const _NL_WMON_5: C2RustUnnamed_0 = 131154;
pub const _NL_WMON_4: C2RustUnnamed_0 = 131153;
pub const _NL_WMON_3: C2RustUnnamed_0 = 131152;
pub const _NL_WMON_2: C2RustUnnamed_0 = 131151;
pub const _NL_WMON_1: C2RustUnnamed_0 = 131150;
pub const _NL_WABMON_12: C2RustUnnamed_0 = 131149;
pub const _NL_WABMON_11: C2RustUnnamed_0 = 131148;
pub const _NL_WABMON_10: C2RustUnnamed_0 = 131147;
pub const _NL_WABMON_9: C2RustUnnamed_0 = 131146;
pub const _NL_WABMON_8: C2RustUnnamed_0 = 131145;
pub const _NL_WABMON_7: C2RustUnnamed_0 = 131144;
pub const _NL_WABMON_6: C2RustUnnamed_0 = 131143;
pub const _NL_WABMON_5: C2RustUnnamed_0 = 131142;
pub const _NL_WABMON_4: C2RustUnnamed_0 = 131141;
pub const _NL_WABMON_3: C2RustUnnamed_0 = 131140;
pub const _NL_WABMON_2: C2RustUnnamed_0 = 131139;
pub const _NL_WABMON_1: C2RustUnnamed_0 = 131138;
pub const _NL_WDAY_7: C2RustUnnamed_0 = 131137;
pub const _NL_WDAY_6: C2RustUnnamed_0 = 131136;
pub const _NL_WDAY_5: C2RustUnnamed_0 = 131135;
pub const _NL_WDAY_4: C2RustUnnamed_0 = 131134;
pub const _NL_WDAY_3: C2RustUnnamed_0 = 131133;
pub const _NL_WDAY_2: C2RustUnnamed_0 = 131132;
pub const _NL_WDAY_1: C2RustUnnamed_0 = 131131;
pub const _NL_WABDAY_7: C2RustUnnamed_0 = 131130;
pub const _NL_WABDAY_6: C2RustUnnamed_0 = 131129;
pub const _NL_WABDAY_5: C2RustUnnamed_0 = 131128;
pub const _NL_WABDAY_4: C2RustUnnamed_0 = 131127;
pub const _NL_WABDAY_3: C2RustUnnamed_0 = 131126;
pub const _NL_WABDAY_2: C2RustUnnamed_0 = 131125;
pub const _NL_WABDAY_1: C2RustUnnamed_0 = 131124;
pub const _NL_TIME_ERA_ENTRIES: C2RustUnnamed_0 = 131123;
pub const _NL_TIME_ERA_NUM_ENTRIES: C2RustUnnamed_0 = 131122;
pub const ERA_T_FMT: C2RustUnnamed_0 = 131121;
pub const ERA_D_T_FMT: C2RustUnnamed_0 = 131120;
pub const ALT_DIGITS: C2RustUnnamed_0 = 131119;
pub const ERA_D_FMT: C2RustUnnamed_0 = 131118;
pub const __ERA_YEAR: C2RustUnnamed_0 = 131117;
pub const ERA: C2RustUnnamed_0 = 131116;
pub const T_FMT_AMPM: C2RustUnnamed_0 = 131115;
pub const T_FMT: C2RustUnnamed_0 = 131114;
pub const D_FMT: C2RustUnnamed_0 = 131113;
pub const D_T_FMT: C2RustUnnamed_0 = 131112;
pub const PM_STR: C2RustUnnamed_0 = 131111;
pub const AM_STR: C2RustUnnamed_0 = 131110;
pub const MON_12: C2RustUnnamed_0 = 131109;
pub const MON_11: C2RustUnnamed_0 = 131108;
pub const MON_10: C2RustUnnamed_0 = 131107;
pub const MON_9: C2RustUnnamed_0 = 131106;
pub const MON_8: C2RustUnnamed_0 = 131105;
pub const MON_7: C2RustUnnamed_0 = 131104;
pub const MON_6: C2RustUnnamed_0 = 131103;
pub const MON_5: C2RustUnnamed_0 = 131102;
pub const MON_4: C2RustUnnamed_0 = 131101;
pub const MON_3: C2RustUnnamed_0 = 131100;
pub const MON_2: C2RustUnnamed_0 = 131099;
pub const MON_1: C2RustUnnamed_0 = 131098;
pub const ABMON_12: C2RustUnnamed_0 = 131097;
pub const ABMON_11: C2RustUnnamed_0 = 131096;
pub const ABMON_10: C2RustUnnamed_0 = 131095;
pub const ABMON_9: C2RustUnnamed_0 = 131094;
pub const ABMON_8: C2RustUnnamed_0 = 131093;
pub const ABMON_7: C2RustUnnamed_0 = 131092;
pub const ABMON_6: C2RustUnnamed_0 = 131091;
pub const ABMON_5: C2RustUnnamed_0 = 131090;
pub const ABMON_4: C2RustUnnamed_0 = 131089;
pub const ABMON_3: C2RustUnnamed_0 = 131088;
pub const ABMON_2: C2RustUnnamed_0 = 131087;
pub const ABMON_1: C2RustUnnamed_0 = 131086;
pub const DAY_7: C2RustUnnamed_0 = 131085;
pub const DAY_6: C2RustUnnamed_0 = 131084;
pub const DAY_5: C2RustUnnamed_0 = 131083;
pub const DAY_4: C2RustUnnamed_0 = 131082;
pub const DAY_3: C2RustUnnamed_0 = 131081;
pub const DAY_2: C2RustUnnamed_0 = 131080;
pub const DAY_1: C2RustUnnamed_0 = 131079;
pub const ABDAY_7: C2RustUnnamed_0 = 131078;
pub const ABDAY_6: C2RustUnnamed_0 = 131077;
pub const ABDAY_5: C2RustUnnamed_0 = 131076;
pub const ABDAY_4: C2RustUnnamed_0 = 131075;
pub const ABDAY_3: C2RustUnnamed_0 = 131074;
pub const ABDAY_2: C2RustUnnamed_0 = 131073;
pub const ABDAY_1: C2RustUnnamed_0 = 131072;
#[no_mangle]
pub unsafe extern "C" fn vasnprintf(
    mut resultbuf: *mut libc::c_char,
    mut lengthp: *mut size_t,
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut d: char_directives = char_directives {
        count: 0,
        dir: 0 as *mut char_directive,
        max_width_length: 0,
        max_precision_length: 0,
        direct_alloc_dir: [char_directive {
            dir_start: 0 as *const libc::c_char,
            dir_end: 0 as *const libc::c_char,
            flags: 0,
            width_start: 0 as *const libc::c_char,
            width_end: 0 as *const libc::c_char,
            width_arg_index: 0,
            precision_start: 0 as *const libc::c_char,
            precision_end: 0 as *const libc::c_char,
            precision_arg_index: 0,
            conversion: 0,
            arg_index: 0,
        }; 7],
    };
    let mut a: arguments = arguments {
        count: 0,
        arg: 0 as *mut argument,
        direct_alloc_arg: [argument {
            type_0: TYPE_NONE,
            a: C2RustUnnamed { a_schar: 0 },
        }; 7],
    };
    if printf_parse(format, &mut d, &mut a) < 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    if printf_fetchargs(args.as_va_list(), &mut a) < 0 as libc::c_int {
        if d.dir != (d.direct_alloc_dir).as_mut_ptr() {
            free(d.dir as *mut libc::c_void);
        }
        if a.arg != (a.direct_alloc_arg).as_mut_ptr() {
            free(a.arg as *mut libc::c_void);
        }
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_char;
    }
    let mut buf_neededlength: size_t = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf_malloced: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: size_t = 0;
    let mut dp: *mut char_directive = 0 as *mut char_directive;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut allocated: size_t = 0;
    let mut length: size_t = 0;
    buf_neededlength = xsum4(
        7 as libc::c_int as size_t,
        d.max_width_length,
        d.max_precision_length,
        6 as libc::c_int as size_t,
    );
    if buf_neededlength
        < (4000 as libc::c_int as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
    {
        let mut fresh0 = ::std::vec::from_elem(
            0,
            buf_neededlength
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                as usize,
        );
        buf = fresh0.as_mut_ptr() as *mut libc::c_char;
        buf_malloced = 0 as *mut libc::c_char;
        current_block = 13797916685926291137;
    } else {
        let mut buf_memsize: size_t = if buf_neededlength
            <= (18446744073709551615 as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
        {
            buf_neededlength
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
        } else {
            18446744073709551615 as libc::c_ulong
        };
        if buf_memsize == 18446744073709551615 as libc::c_ulong {
            current_block = 2829922118317177800;
        } else {
            buf = malloc(buf_memsize) as *mut libc::c_char;
            if buf.is_null() {
                current_block = 2829922118317177800;
            } else {
                buf_malloced = buf;
                current_block = 13797916685926291137;
            }
        }
    }
    match current_block {
        13797916685926291137 => {
            if !resultbuf.is_null() {
                result = resultbuf;
                allocated = *lengthp;
            } else {
                result = 0 as *mut libc::c_char;
                allocated = 0 as libc::c_int as size_t;
            }
            length = 0 as libc::c_int as size_t;
            cp = format;
            i = 0 as libc::c_int as size_t;
            dp = &mut *(d.dir).offset(0 as libc::c_int as isize) as *mut char_directive;
            's_139: loop {
                if cp != (*dp).dir_start {
                    let mut n: size_t = ((*dp).dir_start).offset_from(cp) as libc::c_long
                        as size_t;
                    let mut augmented_length: size_t = xsum(length, n);
                    if augmented_length > allocated {
                        let mut memory_size: size_t = 0;
                        let mut memory: *mut libc::c_char = 0 as *mut libc::c_char;
                        allocated = if allocated > 0 as libc::c_int as libc::c_ulong {
                            if allocated
                                <= (18446744073709551615 as libc::c_ulong)
                                    .wrapping_div(2 as libc::c_int as libc::c_ulong)
                            {
                                allocated.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            } else {
                                18446744073709551615 as libc::c_ulong
                            }
                        } else {
                            12 as libc::c_int as libc::c_ulong
                        };
                        if augmented_length > allocated {
                            allocated = augmented_length;
                        }
                        memory_size = if allocated
                            <= (18446744073709551615 as libc::c_ulong)
                                .wrapping_div(
                                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                )
                        {
                            allocated
                                .wrapping_mul(
                                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                )
                        } else {
                            18446744073709551615 as libc::c_ulong
                        };
                        if memory_size == 18446744073709551615 as libc::c_ulong {
                            current_block = 8340077360075150893;
                            break;
                        }
                        if result == resultbuf || result.is_null() {
                            memory = malloc(memory_size) as *mut libc::c_char;
                        } else {
                            memory = realloc(result as *mut libc::c_void, memory_size)
                                as *mut libc::c_char;
                        }
                        if memory.is_null() {
                            current_block = 8340077360075150893;
                            break;
                        }
                        if result == resultbuf
                            && length > 0 as libc::c_int as libc::c_ulong
                        {
                            memcpy(
                                memory as *mut libc::c_void,
                                result as *const libc::c_void,
                                length,
                            );
                        }
                        result = memory;
                    }
                    if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                        == ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                    {
                        memcpy(
                            result.offset(length as isize) as *mut libc::c_void,
                            cp as *const libc::c_void,
                            n,
                        );
                        length = augmented_length;
                    } else {
                        loop {
                            let fresh1 = cp;
                            cp = cp.offset(1);
                            let fresh2 = length;
                            length = length.wrapping_add(1);
                            *result.offset(fresh2 as isize) = *fresh1;
                            n = n.wrapping_sub(1);
                            if !(n > 0 as libc::c_int as libc::c_ulong) {
                                break;
                            }
                        }
                    }
                }
                if i == d.count {
                    current_block = 701926054148271290;
                    break;
                }
                if (*dp).conversion as libc::c_int == '%' as i32 {
                    let mut augmented_length_0: size_t = 0;
                    if !((*dp).arg_index == !(0 as libc::c_int as size_t)) {
                        abort();
                    }
                    augmented_length_0 = xsum(length, 1 as libc::c_int as size_t);
                    if augmented_length_0 > allocated {
                        let mut memory_size_0: size_t = 0;
                        let mut memory_0: *mut libc::c_char = 0 as *mut libc::c_char;
                        allocated = if allocated > 0 as libc::c_int as libc::c_ulong {
                            if allocated
                                <= (18446744073709551615 as libc::c_ulong)
                                    .wrapping_div(2 as libc::c_int as libc::c_ulong)
                            {
                                allocated.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            } else {
                                18446744073709551615 as libc::c_ulong
                            }
                        } else {
                            12 as libc::c_int as libc::c_ulong
                        };
                        if augmented_length_0 > allocated {
                            allocated = augmented_length_0;
                        }
                        memory_size_0 = if allocated
                            <= (18446744073709551615 as libc::c_ulong)
                                .wrapping_div(
                                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                )
                        {
                            allocated
                                .wrapping_mul(
                                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                )
                        } else {
                            18446744073709551615 as libc::c_ulong
                        };
                        if memory_size_0 == 18446744073709551615 as libc::c_ulong {
                            current_block = 8340077360075150893;
                            break;
                        }
                        if result == resultbuf || result.is_null() {
                            memory_0 = malloc(memory_size_0) as *mut libc::c_char;
                        } else {
                            memory_0 = realloc(
                                result as *mut libc::c_void,
                                memory_size_0,
                            ) as *mut libc::c_char;
                        }
                        if memory_0.is_null() {
                            current_block = 8340077360075150893;
                            break;
                        }
                        if result == resultbuf
                            && length > 0 as libc::c_int as libc::c_ulong
                        {
                            memcpy(
                                memory_0 as *mut libc::c_void,
                                result as *const libc::c_void,
                                length,
                            );
                        }
                        result = memory_0;
                    }
                    *result.offset(length as isize) = '%' as i32 as libc::c_char;
                    length = augmented_length_0;
                } else {
                    if !((*dp).arg_index != !(0 as libc::c_int as size_t)) {
                        abort();
                    }
                    if (*dp).conversion as libc::c_int == 'n' as i32 {
                        match (*(a.arg).offset((*dp).arg_index as isize)).type_0
                            as libc::c_uint
                        {
                            18 => {
                                *(*(a.arg).offset((*dp).arg_index as isize))
                                    .a
                                    .a_count_schar_pointer = length as libc::c_schar;
                            }
                            19 => {
                                *(*(a.arg).offset((*dp).arg_index as isize))
                                    .a
                                    .a_count_short_pointer = length as libc::c_short;
                            }
                            20 => {
                                *(*(a.arg).offset((*dp).arg_index as isize))
                                    .a
                                    .a_count_int_pointer = length as libc::c_int;
                            }
                            21 => {
                                *(*(a.arg).offset((*dp).arg_index as isize))
                                    .a
                                    .a_count_longint_pointer = length as libc::c_long;
                            }
                            22 => {
                                *(*(a.arg).offset((*dp).arg_index as isize))
                                    .a
                                    .a_count_longlongint_pointer = length as libc::c_longlong;
                            }
                            _ => {
                                abort();
                            }
                        }
                    } else if (*dp).conversion as libc::c_int == 'a' as i32
                        || (*dp).conversion as libc::c_int == 'A' as i32
                    {
                        let mut type_0: arg_type = (*(a.arg)
                            .offset((*dp).arg_index as isize))
                            .type_0;
                        let mut flags: libc::c_int = (*dp).flags;
                        let mut width: size_t = 0;
                        let mut has_precision: libc::c_int = 0;
                        let mut precision: size_t = 0;
                        let mut tmp_length: size_t = 0;
                        let mut count: size_t = 0;
                        let mut tmpbuf: [libc::c_char; 700] = [0; 700];
                        let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut pad_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
                        width = 0 as libc::c_int as size_t;
                        if (*dp).width_start != (*dp).width_end {
                            if (*dp).width_arg_index != !(0 as libc::c_int as size_t) {
                                let mut arg: libc::c_int = 0;
                                if !((*(a.arg).offset((*dp).width_arg_index as isize))
                                    .type_0 as libc::c_uint
                                    == TYPE_INT as libc::c_int as libc::c_uint)
                                {
                                    abort();
                                }
                                arg = (*(a.arg).offset((*dp).width_arg_index as isize))
                                    .a
                                    .a_int;
                                width = arg as size_t;
                                if arg < 0 as libc::c_int {
                                    flags |= 2 as libc::c_int;
                                    width = width.wrapping_neg();
                                }
                            } else {
                                let mut digitp: *const libc::c_char = (*dp).width_start;
                                loop {
                                    let fresh3 = digitp;
                                    digitp = digitp.offset(1);
                                    width = xsum(
                                        if width
                                            <= (18446744073709551615 as libc::c_ulong)
                                                .wrapping_div(10 as libc::c_int as libc::c_ulong)
                                        {
                                            width.wrapping_mul(10 as libc::c_int as libc::c_ulong)
                                        } else {
                                            18446744073709551615 as libc::c_ulong
                                        },
                                        (*fresh3 as libc::c_int - '0' as i32) as size_t,
                                    );
                                    if !(digitp != (*dp).width_end) {
                                        break;
                                    }
                                }
                            }
                        }
                        has_precision = 0 as libc::c_int;
                        precision = 0 as libc::c_int as size_t;
                        if (*dp).precision_start != (*dp).precision_end {
                            if (*dp).precision_arg_index != !(0 as libc::c_int as size_t)
                            {
                                let mut arg_0: libc::c_int = 0;
                                if !((*(a.arg).offset((*dp).precision_arg_index as isize))
                                    .type_0 as libc::c_uint
                                    == TYPE_INT as libc::c_int as libc::c_uint)
                                {
                                    abort();
                                }
                                arg_0 = (*(a.arg)
                                    .offset((*dp).precision_arg_index as isize))
                                    .a
                                    .a_int;
                                if arg_0 >= 0 as libc::c_int {
                                    precision = arg_0 as size_t;
                                    has_precision = 1 as libc::c_int;
                                }
                            } else {
                                let mut digitp_0: *const libc::c_char = ((*dp)
                                    .precision_start)
                                    .offset(1 as libc::c_int as isize);
                                precision = 0 as libc::c_int as size_t;
                                while digitp_0 != (*dp).precision_end {
                                    let fresh4 = digitp_0;
                                    digitp_0 = digitp_0.offset(1);
                                    precision = xsum(
                                        if precision
                                            <= (18446744073709551615 as libc::c_ulong)
                                                .wrapping_div(10 as libc::c_int as libc::c_ulong)
                                        {
                                            precision.wrapping_mul(10 as libc::c_int as libc::c_ulong)
                                        } else {
                                            18446744073709551615 as libc::c_ulong
                                        },
                                        (*fresh4 as libc::c_int - '0' as i32) as size_t,
                                    );
                                }
                                has_precision = 1 as libc::c_int;
                            }
                        }
                        if type_0 as libc::c_uint
                            == TYPE_LONGDOUBLE as libc::c_int as libc::c_uint
                        {
                            tmp_length = (((18 as libc::c_int + 1 as libc::c_int)
                                as libc::c_double * 0.831f64) as libc::c_uint)
                                .wrapping_add(1 as libc::c_int as libc::c_uint) as size_t;
                        } else {
                            tmp_length = (((15 as libc::c_int + 1 as libc::c_int)
                                as libc::c_double * 0.831f64) as libc::c_uint)
                                .wrapping_add(1 as libc::c_int as libc::c_uint) as size_t;
                        }
                        if tmp_length < precision {
                            tmp_length = precision;
                        }
                        tmp_length = xsum(tmp_length, 12 as libc::c_int as size_t);
                        if tmp_length < width {
                            tmp_length = width;
                        }
                        tmp_length = xsum(tmp_length, 1 as libc::c_int as size_t);
                        if tmp_length
                            <= (::core::mem::size_of::<[libc::c_char; 700]>()
                                as libc::c_ulong)
                                .wrapping_div(
                                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                )
                        {
                            tmp = tmpbuf.as_mut_ptr();
                        } else {
                            let mut tmp_memsize: size_t = if tmp_length
                                <= (18446744073709551615 as libc::c_ulong)
                                    .wrapping_div(
                                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                    )
                            {
                                tmp_length
                                    .wrapping_mul(
                                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                    )
                            } else {
                                18446744073709551615 as libc::c_ulong
                            };
                            if tmp_memsize == 18446744073709551615 as libc::c_ulong {
                                current_block = 8340077360075150893;
                                break;
                            } else {
                                tmp = malloc(tmp_memsize) as *mut libc::c_char;
                                if tmp.is_null() {
                                    current_block = 8340077360075150893;
                                    break;
                                }
                            }
                        }
                        pad_ptr = 0 as *mut libc::c_char;
                        p = tmp;
                        if type_0 as libc::c_uint
                            == TYPE_LONGDOUBLE as libc::c_int as libc::c_uint
                        {
                            let mut arg_1: f128::f128 = (*(a.arg)
                                .offset((*dp).arg_index as isize))
                                .a
                                .a_longdouble;
                            if arg_1.is_nan() as i32 != 0 {
                                if (*dp).conversion as libc::c_int == 'A' as i32 {
                                    let fresh5 = p;
                                    p = p.offset(1);
                                    *fresh5 = 'N' as i32 as libc::c_char;
                                    let fresh6 = p;
                                    p = p.offset(1);
                                    *fresh6 = 'A' as i32 as libc::c_char;
                                    let fresh7 = p;
                                    p = p.offset(1);
                                    *fresh7 = 'N' as i32 as libc::c_char;
                                } else {
                                    let fresh8 = p;
                                    p = p.offset(1);
                                    *fresh8 = 'n' as i32 as libc::c_char;
                                    let fresh9 = p;
                                    p = p.offset(1);
                                    *fresh9 = 'a' as i32 as libc::c_char;
                                    let fresh10 = p;
                                    p = p.offset(1);
                                    *fresh10 = 'n' as i32 as libc::c_char;
                                }
                            } else {
                                let mut sign: libc::c_int = 0 as libc::c_int;
                                let mut oldcw: fpucw_t = 0;
                                oldcw = {
                                    let mut _cw: fpucw_t = 0;
                                    asm!(
                                        "fnstcw [{0}]", in (reg) & mut _cw, options(preserves_flags)
                                    );
                                    _cw
                                };
                                let mut _ncw: fpucw_t = (oldcw as libc::c_int
                                    & !(0x300 as libc::c_int) | 0x300 as libc::c_int)
                                    as fpucw_t;
                                asm!(
                                    "fldcw [{0}]", in (reg) & _ncw, options(preserves_flags)
                                );
                                'c_15335: {
                                    let mut _ncw: fpucw_t = (oldcw as libc::c_int
                                        & !(0x300 as libc::c_int) | 0x300 as libc::c_int)
                                        as fpucw_t;
                                    asm!(
                                        "fldcw [{0}]", in (reg) & _ncw, options(preserves_flags)
                                    );
                                };
                                if if ::core::mem::size_of::<f128::f128>() as libc::c_ulong
                                    == ::core::mem::size_of::<f128::f128>() as libc::c_ulong
                                {
                                    arg_1.is_sign_negative() as libc::c_int
                                } else if ::core::mem::size_of::<f128::f128>()
                                    as libc::c_ulong
                                    == ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
                                {
                                    arg_1.is_sign_negative() as libc::c_int
                                } else {
                                    arg_1.to_f32().unwrap().is_sign_negative() as libc::c_int
                                } != 0
                                {
                                    sign = -(1 as libc::c_int);
                                    arg_1 = -arg_1;
                                }
                                if sign < 0 as libc::c_int {
                                    let fresh11 = p;
                                    p = p.offset(1);
                                    *fresh11 = '-' as i32 as libc::c_char;
                                } else if flags & 4 as libc::c_int != 0 {
                                    let fresh12 = p;
                                    p = p.offset(1);
                                    *fresh12 = '+' as i32 as libc::c_char;
                                } else if flags & 8 as libc::c_int != 0 {
                                    let fresh13 = p;
                                    p = p.offset(1);
                                    *fresh13 = ' ' as i32 as libc::c_char;
                                }
                                if arg_1 > f128::f128::new(0.0) && arg_1 + arg_1 == arg_1 {
                                    if (*dp).conversion as libc::c_int == 'A' as i32 {
                                        let fresh14 = p;
                                        p = p.offset(1);
                                        *fresh14 = 'I' as i32 as libc::c_char;
                                        let fresh15 = p;
                                        p = p.offset(1);
                                        *fresh15 = 'N' as i32 as libc::c_char;
                                        let fresh16 = p;
                                        p = p.offset(1);
                                        *fresh16 = 'F' as i32 as libc::c_char;
                                    } else {
                                        let fresh17 = p;
                                        p = p.offset(1);
                                        *fresh17 = 'i' as i32 as libc::c_char;
                                        let fresh18 = p;
                                        p = p.offset(1);
                                        *fresh18 = 'n' as i32 as libc::c_char;
                                        let fresh19 = p;
                                        p = p.offset(1);
                                        *fresh19 = 'f' as i32 as libc::c_char;
                                    }
                                } else {
                                    let mut exponent: libc::c_int = 0;
                                    let mut mantissa: f128::f128 = f128::f128::ZERO;
                                    if arg_1 > f128::f128::new(0.0) {
                                        mantissa = printf_frexpl(arg_1, &mut exponent);
                                    } else {
                                        exponent = 0 as libc::c_int;
                                        mantissa = f128::f128::new(0.0);
                                    }
                                    if has_precision != 0
                                        && precision
                                            < (((18 as libc::c_int + 1 as libc::c_int) as libc::c_double
                                                * 0.831f64) as libc::c_uint)
                                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                                                as libc::c_ulong
                                    {
                                        let mut tail: f128::f128 = mantissa;
                                        let mut q: size_t = 0;
                                        q = precision;
                                        loop {
                                            let mut digit: libc::c_int = tail.to_i32().unwrap();
                                            tail -= f128::f128::new(digit);
                                            if q == 0 as libc::c_int as libc::c_ulong {
                                                if if digit & 1 as libc::c_int != 0 {
                                                    (tail >= f128::f128::new(0.5)) as libc::c_int
                                                } else {
                                                    (tail > f128::f128::new(0.5)) as libc::c_int
                                                } != 0
                                                {
                                                    tail = f128::f128::new(1 as libc::c_int) - tail;
                                                } else {
                                                    tail = -tail;
                                                }
                                                break;
                                            } else {
                                                tail *= f128::f128::new(16.0);
                                                q = q.wrapping_sub(1);
                                                q;
                                            }
                                        }
                                        if tail != f128::f128::new(0.0) {
                                            q = precision;
                                            while q > 0 as libc::c_int as libc::c_ulong {
                                                tail *= f128::f128::new(0.0625);
                                                q = q.wrapping_sub(1);
                                                q;
                                            }
                                        }
                                        mantissa += tail;
                                    }
                                    let fresh20 = p;
                                    p = p.offset(1);
                                    *fresh20 = '0' as i32 as libc::c_char;
                                    let fresh21 = p;
                                    p = p.offset(1);
                                    *fresh21 = ((*dp).conversion as libc::c_int - 'A' as i32
                                        + 'X' as i32) as libc::c_char;
                                    pad_ptr = p;
                                    let mut digit_0: libc::c_int = 0;
                                    digit_0 = mantissa.to_i32().unwrap();
                                    mantissa -= f128::f128::new(digit_0);
                                    let fresh22 = p;
                                    p = p.offset(1);
                                    *fresh22 = ('0' as i32 + digit_0) as libc::c_char;
                                    if flags & 16 as libc::c_int != 0
                                        || mantissa > f128::f128::new(0.0)
                                        || precision > 0 as libc::c_int as libc::c_ulong
                                    {
                                        let fresh23 = p;
                                        p = p.offset(1);
                                        *fresh23 = decimal_point_char();
                                        while mantissa > f128::f128::new(0.0) {
                                            mantissa *= f128::f128::new(16.0);
                                            digit_0 = mantissa.to_i32().unwrap();
                                            mantissa -= f128::f128::new(digit_0);
                                            let fresh24 = p;
                                            p = p.offset(1);
                                            *fresh24 = (digit_0
                                                + (if digit_0 < 10 as libc::c_int {
                                                    '0' as i32
                                                } else {
                                                    (*dp).conversion as libc::c_int - 10 as libc::c_int
                                                })) as libc::c_char;
                                            if precision > 0 as libc::c_int as libc::c_ulong {
                                                precision = precision.wrapping_sub(1);
                                                precision;
                                            }
                                        }
                                        while precision > 0 as libc::c_int as libc::c_ulong {
                                            let fresh25 = p;
                                            p = p.offset(1);
                                            *fresh25 = '0' as i32 as libc::c_char;
                                            precision = precision.wrapping_sub(1);
                                            precision;
                                        }
                                    }
                                    let fresh26 = p;
                                    p = p.offset(1);
                                    *fresh26 = ((*dp).conversion as libc::c_int - 'A' as i32
                                        + 'P' as i32) as libc::c_char;
                                    if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        == 1 as libc::c_int as libc::c_ulong
                                    {
                                        sprintf(
                                            p,
                                            b"%+d\0" as *const u8 as *const libc::c_char,
                                            exponent,
                                        );
                                        while *p as libc::c_int != '\0' as i32 {
                                            p = p.offset(1);
                                            p;
                                        }
                                    } else {
                                        let mut expbuf: [libc::c_char; 7] = [0; 7];
                                        let mut ep: *const libc::c_char = 0 as *const libc::c_char;
                                        sprintf(
                                            expbuf.as_mut_ptr(),
                                            b"%+d\0" as *const u8 as *const libc::c_char,
                                            exponent,
                                        );
                                        ep = expbuf.as_mut_ptr();
                                        loop {
                                            *p = *ep;
                                            if !(*p as libc::c_int != '\0' as i32) {
                                                break;
                                            }
                                            p = p.offset(1);
                                            p;
                                            ep = ep.offset(1);
                                            ep;
                                        }
                                    }
                                }
                                let mut _ncw: fpucw_t = oldcw;
                                asm!(
                                    "fldcw [{0}]", in (reg) & _ncw, options(preserves_flags)
                                );
                                'c_14784: {
                                    let mut _ncw: fpucw_t = oldcw;
                                    asm!(
                                        "fldcw [{0}]", in (reg) & _ncw, options(preserves_flags)
                                    );
                                };
                            }
                        } else {
                            let mut arg_2: libc::c_double = (*(a.arg)
                                .offset((*dp).arg_index as isize))
                                .a
                                .a_double;
                            if arg_2.is_nan() as i32 != 0 {
                                if (*dp).conversion as libc::c_int == 'A' as i32 {
                                    let fresh27 = p;
                                    p = p.offset(1);
                                    *fresh27 = 'N' as i32 as libc::c_char;
                                    let fresh28 = p;
                                    p = p.offset(1);
                                    *fresh28 = 'A' as i32 as libc::c_char;
                                    let fresh29 = p;
                                    p = p.offset(1);
                                    *fresh29 = 'N' as i32 as libc::c_char;
                                } else {
                                    let fresh30 = p;
                                    p = p.offset(1);
                                    *fresh30 = 'n' as i32 as libc::c_char;
                                    let fresh31 = p;
                                    p = p.offset(1);
                                    *fresh31 = 'a' as i32 as libc::c_char;
                                    let fresh32 = p;
                                    p = p.offset(1);
                                    *fresh32 = 'n' as i32 as libc::c_char;
                                }
                            } else {
                                let mut sign_0: libc::c_int = 0 as libc::c_int;
                                if if ::core::mem::size_of::<libc::c_double>()
                                    as libc::c_ulong
                                    == ::core::mem::size_of::<f128::f128>() as libc::c_ulong
                                {
                                    (f128::f128::new(arg_2)).is_sign_negative() as libc::c_int
                                } else if ::core::mem::size_of::<libc::c_double>()
                                    as libc::c_ulong
                                    == ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
                                {
                                    arg_2.is_sign_negative() as libc::c_int
                                } else {
                                    (arg_2 as libc::c_float).is_sign_negative() as libc::c_int
                                } != 0
                                {
                                    sign_0 = -(1 as libc::c_int);
                                    arg_2 = -arg_2;
                                }
                                if sign_0 < 0 as libc::c_int {
                                    let fresh33 = p;
                                    p = p.offset(1);
                                    *fresh33 = '-' as i32 as libc::c_char;
                                } else if flags & 4 as libc::c_int != 0 {
                                    let fresh34 = p;
                                    p = p.offset(1);
                                    *fresh34 = '+' as i32 as libc::c_char;
                                } else if flags & 8 as libc::c_int != 0 {
                                    let fresh35 = p;
                                    p = p.offset(1);
                                    *fresh35 = ' ' as i32 as libc::c_char;
                                }
                                if arg_2 > 0.0f64 && arg_2 + arg_2 == arg_2 {
                                    if (*dp).conversion as libc::c_int == 'A' as i32 {
                                        let fresh36 = p;
                                        p = p.offset(1);
                                        *fresh36 = 'I' as i32 as libc::c_char;
                                        let fresh37 = p;
                                        p = p.offset(1);
                                        *fresh37 = 'N' as i32 as libc::c_char;
                                        let fresh38 = p;
                                        p = p.offset(1);
                                        *fresh38 = 'F' as i32 as libc::c_char;
                                    } else {
                                        let fresh39 = p;
                                        p = p.offset(1);
                                        *fresh39 = 'i' as i32 as libc::c_char;
                                        let fresh40 = p;
                                        p = p.offset(1);
                                        *fresh40 = 'n' as i32 as libc::c_char;
                                        let fresh41 = p;
                                        p = p.offset(1);
                                        *fresh41 = 'f' as i32 as libc::c_char;
                                    }
                                } else {
                                    let mut exponent_0: libc::c_int = 0;
                                    let mut mantissa_0: libc::c_double = 0.;
                                    if arg_2 > 0.0f64 {
                                        mantissa_0 = printf_frexp(arg_2, &mut exponent_0);
                                    } else {
                                        exponent_0 = 0 as libc::c_int;
                                        mantissa_0 = 0.0f64;
                                    }
                                    if has_precision != 0
                                        && precision
                                            < (((15 as libc::c_int + 1 as libc::c_int) as libc::c_double
                                                * 0.831f64) as libc::c_uint)
                                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                                                as libc::c_ulong
                                    {
                                        let mut tail_0: libc::c_double = mantissa_0;
                                        let mut q_0: size_t = 0;
                                        q_0 = precision;
                                        loop {
                                            let mut digit_1: libc::c_int = tail_0 as libc::c_int;
                                            tail_0 -= digit_1 as libc::c_double;
                                            if q_0 == 0 as libc::c_int as libc::c_ulong {
                                                if if digit_1 & 1 as libc::c_int != 0 {
                                                    (tail_0 >= 0.5f64) as libc::c_int
                                                } else {
                                                    (tail_0 > 0.5f64) as libc::c_int
                                                } != 0
                                                {
                                                    tail_0 = 1 as libc::c_int as libc::c_double - tail_0;
                                                } else {
                                                    tail_0 = -tail_0;
                                                }
                                                break;
                                            } else {
                                                tail_0 *= 16.0f64;
                                                q_0 = q_0.wrapping_sub(1);
                                                q_0;
                                            }
                                        }
                                        if tail_0 != 0.0f64 {
                                            q_0 = precision;
                                            while q_0 > 0 as libc::c_int as libc::c_ulong {
                                                tail_0 *= 0.0625f64;
                                                q_0 = q_0.wrapping_sub(1);
                                                q_0;
                                            }
                                        }
                                        mantissa_0 += tail_0;
                                    }
                                    let fresh42 = p;
                                    p = p.offset(1);
                                    *fresh42 = '0' as i32 as libc::c_char;
                                    let fresh43 = p;
                                    p = p.offset(1);
                                    *fresh43 = ((*dp).conversion as libc::c_int - 'A' as i32
                                        + 'X' as i32) as libc::c_char;
                                    pad_ptr = p;
                                    let mut digit_2: libc::c_int = 0;
                                    digit_2 = mantissa_0 as libc::c_int;
                                    mantissa_0 -= digit_2 as libc::c_double;
                                    let fresh44 = p;
                                    p = p.offset(1);
                                    *fresh44 = ('0' as i32 + digit_2) as libc::c_char;
                                    if flags & 16 as libc::c_int != 0 || mantissa_0 > 0.0f64
                                        || precision > 0 as libc::c_int as libc::c_ulong
                                    {
                                        let fresh45 = p;
                                        p = p.offset(1);
                                        *fresh45 = decimal_point_char();
                                        while mantissa_0 > 0.0f64 {
                                            mantissa_0 *= 16.0f64;
                                            digit_2 = mantissa_0 as libc::c_int;
                                            mantissa_0 -= digit_2 as libc::c_double;
                                            let fresh46 = p;
                                            p = p.offset(1);
                                            *fresh46 = (digit_2
                                                + (if digit_2 < 10 as libc::c_int {
                                                    '0' as i32
                                                } else {
                                                    (*dp).conversion as libc::c_int - 10 as libc::c_int
                                                })) as libc::c_char;
                                            if precision > 0 as libc::c_int as libc::c_ulong {
                                                precision = precision.wrapping_sub(1);
                                                precision;
                                            }
                                        }
                                        while precision > 0 as libc::c_int as libc::c_ulong {
                                            let fresh47 = p;
                                            p = p.offset(1);
                                            *fresh47 = '0' as i32 as libc::c_char;
                                            precision = precision.wrapping_sub(1);
                                            precision;
                                        }
                                    }
                                    let fresh48 = p;
                                    p = p.offset(1);
                                    *fresh48 = ((*dp).conversion as libc::c_int - 'A' as i32
                                        + 'P' as i32) as libc::c_char;
                                    if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        == 1 as libc::c_int as libc::c_ulong
                                    {
                                        sprintf(
                                            p,
                                            b"%+d\0" as *const u8 as *const libc::c_char,
                                            exponent_0,
                                        );
                                        while *p as libc::c_int != '\0' as i32 {
                                            p = p.offset(1);
                                            p;
                                        }
                                    } else {
                                        let mut expbuf_0: [libc::c_char; 7] = [0; 7];
                                        let mut ep_0: *const libc::c_char = 0
                                            as *const libc::c_char;
                                        sprintf(
                                            expbuf_0.as_mut_ptr(),
                                            b"%+d\0" as *const u8 as *const libc::c_char,
                                            exponent_0,
                                        );
                                        ep_0 = expbuf_0.as_mut_ptr();
                                        loop {
                                            *p = *ep_0;
                                            if !(*p as libc::c_int != '\0' as i32) {
                                                break;
                                            }
                                            p = p.offset(1);
                                            p;
                                            ep_0 = ep_0.offset(1);
                                            ep_0;
                                        }
                                    }
                                }
                            }
                        }
                        count = p.offset_from(tmp) as libc::c_long as size_t;
                        if count < width {
                            let mut pad: size_t = width.wrapping_sub(count);
                            let mut end: *mut libc::c_char = p.offset(pad as isize);
                            if flags & 2 as libc::c_int != 0 {
                                while pad > 0 as libc::c_int as libc::c_ulong {
                                    let fresh49 = p;
                                    p = p.offset(1);
                                    *fresh49 = ' ' as i32 as libc::c_char;
                                    pad = pad.wrapping_sub(1);
                                    pad;
                                }
                            } else if flags & 32 as libc::c_int != 0
                                && !pad_ptr.is_null()
                            {
                                let mut q_1: *mut libc::c_char = end;
                                while p > pad_ptr {
                                    p = p.offset(-1);
                                    q_1 = q_1.offset(-1);
                                    *q_1 = *p;
                                }
                                while pad > 0 as libc::c_int as libc::c_ulong {
                                    let fresh50 = p;
                                    p = p.offset(1);
                                    *fresh50 = '0' as i32 as libc::c_char;
                                    pad = pad.wrapping_sub(1);
                                    pad;
                                }
                            } else {
                                let mut q_2: *mut libc::c_char = end;
                                while p > tmp {
                                    p = p.offset(-1);
                                    q_2 = q_2.offset(-1);
                                    *q_2 = *p;
                                }
                                while pad > 0 as libc::c_int as libc::c_ulong {
                                    let fresh51 = p;
                                    p = p.offset(1);
                                    *fresh51 = ' ' as i32 as libc::c_char;
                                    pad = pad.wrapping_sub(1);
                                    pad;
                                }
                            }
                            p = end;
                        }
                        count = p.offset_from(tmp) as libc::c_long as size_t;
                        if count >= tmp_length {
                            abort();
                        }
                        if count >= allocated.wrapping_sub(length) {
                            let mut n_0: size_t = xsum(length, count);
                            if n_0 > allocated {
                                let mut memory_size_1: size_t = 0;
                                let mut memory_1: *mut libc::c_char = 0
                                    as *mut libc::c_char;
                                allocated = if allocated > 0 as libc::c_int as libc::c_ulong
                                {
                                    if allocated
                                        <= (18446744073709551615 as libc::c_ulong)
                                            .wrapping_div(2 as libc::c_int as libc::c_ulong)
                                    {
                                        allocated.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    } else {
                                        18446744073709551615 as libc::c_ulong
                                    }
                                } else {
                                    12 as libc::c_int as libc::c_ulong
                                };
                                if n_0 > allocated {
                                    allocated = n_0;
                                }
                                memory_size_1 = if allocated
                                    <= (18446744073709551615 as libc::c_ulong)
                                        .wrapping_div(
                                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        )
                                {
                                    allocated
                                        .wrapping_mul(
                                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        )
                                } else {
                                    18446744073709551615 as libc::c_ulong
                                };
                                if memory_size_1 == 18446744073709551615 as libc::c_ulong {
                                    current_block = 8340077360075150893;
                                    break;
                                }
                                if result == resultbuf || result.is_null() {
                                    memory_1 = malloc(memory_size_1) as *mut libc::c_char;
                                } else {
                                    memory_1 = realloc(
                                        result as *mut libc::c_void,
                                        memory_size_1,
                                    ) as *mut libc::c_char;
                                }
                                if memory_1.is_null() {
                                    current_block = 8340077360075150893;
                                    break;
                                }
                                if result == resultbuf
                                    && length > 0 as libc::c_int as libc::c_ulong
                                {
                                    memcpy(
                                        memory_1 as *mut libc::c_void,
                                        result as *const libc::c_void,
                                        length,
                                    );
                                }
                                result = memory_1;
                            }
                        }
                        memcpy(
                            result.offset(length as isize) as *mut libc::c_void,
                            tmp as *const libc::c_void,
                            count
                                .wrapping_mul(
                                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                ),
                        );
                        if tmp != tmpbuf.as_mut_ptr() {
                            free(tmp as *mut libc::c_void);
                        }
                        length = (length as libc::c_ulong).wrapping_add(count) as size_t
                            as size_t;
                    } else if ((*dp).conversion as libc::c_int == 'f' as i32
                        || (*dp).conversion as libc::c_int == 'F' as i32
                        || (*dp).conversion as libc::c_int == 'e' as i32
                        || (*dp).conversion as libc::c_int == 'E' as i32
                        || (*dp).conversion as libc::c_int == 'g' as i32
                        || (*dp).conversion as libc::c_int == 'G' as i32
                        || (*dp).conversion as libc::c_int == 'a' as i32
                        || (*dp).conversion as libc::c_int == 'A' as i32)
                        && (0 as libc::c_int != 0
                            || (*(a.arg).offset((*dp).arg_index as isize)).type_0
                                as libc::c_uint
                                == TYPE_DOUBLE as libc::c_int as libc::c_uint
                            || (*(a.arg).offset((*dp).arg_index as isize)).type_0
                                as libc::c_uint
                                == TYPE_LONGDOUBLE as libc::c_int as libc::c_uint)
                    {
                        let mut type_1: arg_type = (*(a.arg)
                            .offset((*dp).arg_index as isize))
                            .type_0;
                        let mut flags_0: libc::c_int = (*dp).flags;
                        let mut width_0: size_t = 0;
                        let mut count_0: size_t = 0;
                        let mut has_precision_0: libc::c_int = 0;
                        let mut precision_0: size_t = 0;
                        let mut tmp_length_0: size_t = 0;
                        let mut tmpbuf_0: [libc::c_char; 700] = [0; 700];
                        let mut tmp_0: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut pad_ptr_0: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut p_0: *mut libc::c_char = 0 as *mut libc::c_char;
                        width_0 = 0 as libc::c_int as size_t;
                        if (*dp).width_start != (*dp).width_end {
                            if (*dp).width_arg_index != !(0 as libc::c_int as size_t) {
                                let mut arg_3: libc::c_int = 0;
                                if !((*(a.arg).offset((*dp).width_arg_index as isize))
                                    .type_0 as libc::c_uint
                                    == TYPE_INT as libc::c_int as libc::c_uint)
                                {
                                    abort();
                                }
                                arg_3 = (*(a.arg).offset((*dp).width_arg_index as isize))
                                    .a
                                    .a_int;
                                width_0 = arg_3 as size_t;
                                if arg_3 < 0 as libc::c_int {
                                    flags_0 |= 2 as libc::c_int;
                                    width_0 = width_0.wrapping_neg();
                                }
                            } else {
                                let mut digitp_1: *const libc::c_char = (*dp).width_start;
                                loop {
                                    let fresh52 = digitp_1;
                                    digitp_1 = digitp_1.offset(1);
                                    width_0 = xsum(
                                        if width_0
                                            <= (18446744073709551615 as libc::c_ulong)
                                                .wrapping_div(10 as libc::c_int as libc::c_ulong)
                                        {
                                            width_0.wrapping_mul(10 as libc::c_int as libc::c_ulong)
                                        } else {
                                            18446744073709551615 as libc::c_ulong
                                        },
                                        (*fresh52 as libc::c_int - '0' as i32) as size_t,
                                    );
                                    if !(digitp_1 != (*dp).width_end) {
                                        break;
                                    }
                                }
                            }
                        }
                        has_precision_0 = 0 as libc::c_int;
                        precision_0 = 0 as libc::c_int as size_t;
                        if (*dp).precision_start != (*dp).precision_end {
                            if (*dp).precision_arg_index != !(0 as libc::c_int as size_t)
                            {
                                let mut arg_4: libc::c_int = 0;
                                if !((*(a.arg).offset((*dp).precision_arg_index as isize))
                                    .type_0 as libc::c_uint
                                    == TYPE_INT as libc::c_int as libc::c_uint)
                                {
                                    abort();
                                }
                                arg_4 = (*(a.arg)
                                    .offset((*dp).precision_arg_index as isize))
                                    .a
                                    .a_int;
                                if arg_4 >= 0 as libc::c_int {
                                    precision_0 = arg_4 as size_t;
                                    has_precision_0 = 1 as libc::c_int;
                                }
                            } else {
                                let mut digitp_2: *const libc::c_char = ((*dp)
                                    .precision_start)
                                    .offset(1 as libc::c_int as isize);
                                precision_0 = 0 as libc::c_int as size_t;
                                while digitp_2 != (*dp).precision_end {
                                    let fresh53 = digitp_2;
                                    digitp_2 = digitp_2.offset(1);
                                    precision_0 = xsum(
                                        if precision_0
                                            <= (18446744073709551615 as libc::c_ulong)
                                                .wrapping_div(10 as libc::c_int as libc::c_ulong)
                                        {
                                            precision_0.wrapping_mul(10 as libc::c_int as libc::c_ulong)
                                        } else {
                                            18446744073709551615 as libc::c_ulong
                                        },
                                        (*fresh53 as libc::c_int - '0' as i32) as size_t,
                                    );
                                }
                                has_precision_0 = 1 as libc::c_int;
                            }
                        }
                        if has_precision_0 == 0 {
                            if !((*dp).conversion as libc::c_int == 'a' as i32
                                || (*dp).conversion as libc::c_int == 'A' as i32)
                            {
                                precision_0 = 6 as libc::c_int as size_t;
                            }
                        }
                        tmp_length_0 = (if type_1 as libc::c_uint
                            == TYPE_LONGDOUBLE as libc::c_int as libc::c_uint
                        {
                            18 as libc::c_int + 1 as libc::c_int
                        } else {
                            15 as libc::c_int + 1 as libc::c_int
                        }) as size_t;
                        if tmp_length_0 < precision_0 {
                            tmp_length_0 = precision_0;
                        }
                        if type_1 as libc::c_uint
                            == TYPE_LONGDOUBLE as libc::c_int as libc::c_uint
                        {
                            if (*dp).conversion as libc::c_int == 'f' as i32
                                || (*dp).conversion as libc::c_int == 'F' as i32
                            {
                                let mut arg_5: f128::f128 = (*(a.arg)
                                    .offset((*dp).arg_index as isize))
                                    .a
                                    .a_longdouble;
                                if !(arg_5.is_nan() as i32 != 0 || arg_5 + arg_5 == arg_5) {
                                    let mut exponent_1: libc::c_int = floorlog10l(
                                        if arg_5 < f128::f128::new(0 as libc::c_int) {
                                            -arg_5
                                        } else {
                                            arg_5
                                        },
                                    );
                                    if exponent_1 >= 0 as libc::c_int
                                        && tmp_length_0
                                            < (exponent_1 as libc::c_ulong).wrapping_add(precision_0)
                                    {
                                        tmp_length_0 = (exponent_1 as libc::c_ulong)
                                            .wrapping_add(precision_0);
                                    }
                                }
                            }
                        }
                        if type_1 as libc::c_uint
                            == TYPE_DOUBLE as libc::c_int as libc::c_uint
                        {
                            if (*dp).conversion as libc::c_int == 'f' as i32
                                || (*dp).conversion as libc::c_int == 'F' as i32
                            {
                                let mut arg_6: libc::c_double = (*(a.arg)
                                    .offset((*dp).arg_index as isize))
                                    .a
                                    .a_double;
                                if !(arg_6.is_nan() as i32 != 0 || arg_6 + arg_6 == arg_6) {
                                    let mut exponent_2: libc::c_int = floorlog10(
                                        if arg_6 < 0 as libc::c_int as libc::c_double {
                                            -arg_6
                                        } else {
                                            arg_6
                                        },
                                    );
                                    if exponent_2 >= 0 as libc::c_int
                                        && tmp_length_0
                                            < (exponent_2 as libc::c_ulong).wrapping_add(precision_0)
                                    {
                                        tmp_length_0 = (exponent_2 as libc::c_ulong)
                                            .wrapping_add(precision_0);
                                    }
                                }
                            }
                        }
                        tmp_length_0 = xsum(tmp_length_0, 12 as libc::c_int as size_t);
                        if tmp_length_0 < width_0 {
                            tmp_length_0 = width_0;
                        }
                        tmp_length_0 = xsum(tmp_length_0, 1 as libc::c_int as size_t);
                        if tmp_length_0
                            <= (::core::mem::size_of::<[libc::c_char; 700]>()
                                as libc::c_ulong)
                                .wrapping_div(
                                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                )
                        {
                            tmp_0 = tmpbuf_0.as_mut_ptr();
                        } else {
                            let mut tmp_memsize_0: size_t = if tmp_length_0
                                <= (18446744073709551615 as libc::c_ulong)
                                    .wrapping_div(
                                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                    )
                            {
                                tmp_length_0
                                    .wrapping_mul(
                                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                    )
                            } else {
                                18446744073709551615 as libc::c_ulong
                            };
                            if tmp_memsize_0 == 18446744073709551615 as libc::c_ulong {
                                current_block = 8340077360075150893;
                                break;
                            } else {
                                tmp_0 = malloc(tmp_memsize_0) as *mut libc::c_char;
                                if tmp_0.is_null() {
                                    current_block = 8340077360075150893;
                                    break;
                                }
                            }
                        }
                        pad_ptr_0 = 0 as *mut libc::c_char;
                        p_0 = tmp_0;
                        if type_1 as libc::c_uint
                            == TYPE_LONGDOUBLE as libc::c_int as libc::c_uint
                        {
                            let mut arg_7: f128::f128 = (*(a.arg)
                                .offset((*dp).arg_index as isize))
                                .a
                                .a_longdouble;
                            if arg_7.is_nan() as i32 != 0 {
                                if (*dp).conversion as libc::c_int >= 'A' as i32
                                    && (*dp).conversion as libc::c_int <= 'Z' as i32
                                {
                                    let fresh54 = p_0;
                                    p_0 = p_0.offset(1);
                                    *fresh54 = 'N' as i32 as libc::c_char;
                                    let fresh55 = p_0;
                                    p_0 = p_0.offset(1);
                                    *fresh55 = 'A' as i32 as libc::c_char;
                                    let fresh56 = p_0;
                                    p_0 = p_0.offset(1);
                                    *fresh56 = 'N' as i32 as libc::c_char;
                                } else {
                                    let fresh57 = p_0;
                                    p_0 = p_0.offset(1);
                                    *fresh57 = 'n' as i32 as libc::c_char;
                                    let fresh58 = p_0;
                                    p_0 = p_0.offset(1);
                                    *fresh58 = 'a' as i32 as libc::c_char;
                                    let fresh59 = p_0;
                                    p_0 = p_0.offset(1);
                                    *fresh59 = 'n' as i32 as libc::c_char;
                                }
                            } else {
                                let mut sign_1: libc::c_int = 0 as libc::c_int;
                                let mut oldcw_0: fpucw_t = 0;
                                oldcw_0 = {
                                    let mut _cw: fpucw_t = 0;
                                    asm!(
                                        "fnstcw [{0}]", in (reg) & mut _cw, options(preserves_flags)
                                    );
                                    _cw
                                };
                                let mut _ncw: fpucw_t = (oldcw_0 as libc::c_int
                                    & !(0x300 as libc::c_int) | 0x300 as libc::c_int)
                                    as fpucw_t;
                                asm!(
                                    "fldcw [{0}]", in (reg) & _ncw, options(preserves_flags)
                                );
                                'c_12904: {
                                    let mut _ncw: fpucw_t = (oldcw_0 as libc::c_int
                                        & !(0x300 as libc::c_int) | 0x300 as libc::c_int)
                                        as fpucw_t;
                                    asm!(
                                        "fldcw [{0}]", in (reg) & _ncw, options(preserves_flags)
                                    );
                                };
                                if if ::core::mem::size_of::<f128::f128>() as libc::c_ulong
                                    == ::core::mem::size_of::<f128::f128>() as libc::c_ulong
                                {
                                    arg_7.is_sign_negative() as libc::c_int
                                } else if ::core::mem::size_of::<f128::f128>()
                                    as libc::c_ulong
                                    == ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
                                {
                                    arg_7.is_sign_negative() as libc::c_int
                                } else {
                                    arg_7.to_f32().unwrap().is_sign_negative() as libc::c_int
                                } != 0
                                {
                                    sign_1 = -(1 as libc::c_int);
                                    arg_7 = -arg_7;
                                }
                                if sign_1 < 0 as libc::c_int {
                                    let fresh60 = p_0;
                                    p_0 = p_0.offset(1);
                                    *fresh60 = '-' as i32 as libc::c_char;
                                } else if flags_0 & 4 as libc::c_int != 0 {
                                    let fresh61 = p_0;
                                    p_0 = p_0.offset(1);
                                    *fresh61 = '+' as i32 as libc::c_char;
                                } else if flags_0 & 8 as libc::c_int != 0 {
                                    let fresh62 = p_0;
                                    p_0 = p_0.offset(1);
                                    *fresh62 = ' ' as i32 as libc::c_char;
                                }
                                if arg_7 > f128::f128::new(0.0) && arg_7 + arg_7 == arg_7 {
                                    if (*dp).conversion as libc::c_int >= 'A' as i32
                                        && (*dp).conversion as libc::c_int <= 'Z' as i32
                                    {
                                        let fresh63 = p_0;
                                        p_0 = p_0.offset(1);
                                        *fresh63 = 'I' as i32 as libc::c_char;
                                        let fresh64 = p_0;
                                        p_0 = p_0.offset(1);
                                        *fresh64 = 'N' as i32 as libc::c_char;
                                        let fresh65 = p_0;
                                        p_0 = p_0.offset(1);
                                        *fresh65 = 'F' as i32 as libc::c_char;
                                    } else {
                                        let fresh66 = p_0;
                                        p_0 = p_0.offset(1);
                                        *fresh66 = 'i' as i32 as libc::c_char;
                                        let fresh67 = p_0;
                                        p_0 = p_0.offset(1);
                                        *fresh67 = 'n' as i32 as libc::c_char;
                                        let fresh68 = p_0;
                                        p_0 = p_0.offset(1);
                                        *fresh68 = 'f' as i32 as libc::c_char;
                                    }
                                } else {
                                    pad_ptr_0 = p_0;
                                    if (*dp).conversion as libc::c_int == 'f' as i32
                                        || (*dp).conversion as libc::c_int == 'F' as i32
                                    {
                                        let mut digits: *mut libc::c_char = 0 as *mut libc::c_char;
                                        let mut ndigits: size_t = 0;
                                        digits = scale10_round_decimal_long_double(
                                            arg_7,
                                            precision_0 as libc::c_int,
                                        );
                                        if digits.is_null() {
                                            let mut _ncw: fpucw_t = oldcw_0;
                                            asm!(
                                                "fldcw [{0}]", in (reg) & _ncw, options(preserves_flags)
                                            );
                                            'c_12702: {
                                                let mut _ncw: fpucw_t = oldcw_0;
                                                asm!(
                                                    "fldcw [{0}]", in (reg) & _ncw, options(preserves_flags)
                                                );
                                            };
                                            current_block = 8340077360075150893;
                                            break;
                                        } else {
                                            ndigits = strlen(digits);
                                            if ndigits > precision_0 {
                                                loop {
                                                    ndigits = ndigits.wrapping_sub(1);
                                                    ndigits;
                                                    let fresh69 = p_0;
                                                    p_0 = p_0.offset(1);
                                                    *fresh69 = *digits.offset(ndigits as isize);
                                                    if !(ndigits > precision_0) {
                                                        break;
                                                    }
                                                }
                                            } else {
                                                let fresh70 = p_0;
                                                p_0 = p_0.offset(1);
                                                *fresh70 = '0' as i32 as libc::c_char;
                                            }
                                            if flags_0 & 16 as libc::c_int != 0
                                                || precision_0 > 0 as libc::c_int as libc::c_ulong
                                            {
                                                let fresh71 = p_0;
                                                p_0 = p_0.offset(1);
                                                *fresh71 = decimal_point_char();
                                                while precision_0 > ndigits {
                                                    let fresh72 = p_0;
                                                    p_0 = p_0.offset(1);
                                                    *fresh72 = '0' as i32 as libc::c_char;
                                                    precision_0 = precision_0.wrapping_sub(1);
                                                    precision_0;
                                                }
                                                while ndigits > 0 as libc::c_int as libc::c_ulong {
                                                    ndigits = ndigits.wrapping_sub(1);
                                                    ndigits;
                                                    let fresh73 = p_0;
                                                    p_0 = p_0.offset(1);
                                                    *fresh73 = *digits.offset(ndigits as isize);
                                                }
                                            }
                                            free(digits as *mut libc::c_void);
                                        }
                                    } else if (*dp).conversion as libc::c_int == 'e' as i32
                                        || (*dp).conversion as libc::c_int == 'E' as i32
                                    {
                                        let mut exponent_3: libc::c_int = 0;
                                        if arg_7 == f128::f128::new(0.0) {
                                            exponent_3 = 0 as libc::c_int;
                                            let fresh74 = p_0;
                                            p_0 = p_0.offset(1);
                                            *fresh74 = '0' as i32 as libc::c_char;
                                            if flags_0 & 16 as libc::c_int != 0
                                                || precision_0 > 0 as libc::c_int as libc::c_ulong
                                            {
                                                let fresh75 = p_0;
                                                p_0 = p_0.offset(1);
                                                *fresh75 = decimal_point_char();
                                                while precision_0 > 0 as libc::c_int as libc::c_ulong {
                                                    let fresh76 = p_0;
                                                    p_0 = p_0.offset(1);
                                                    *fresh76 = '0' as i32 as libc::c_char;
                                                    precision_0 = precision_0.wrapping_sub(1);
                                                    precision_0;
                                                }
                                            }
                                        } else {
                                            let mut adjusted: libc::c_int = 0;
                                            let mut digits_0: *mut libc::c_char = 0
                                                as *mut libc::c_char;
                                            let mut ndigits_0: size_t = 0;
                                            exponent_3 = floorlog10l(arg_7);
                                            adjusted = 0 as libc::c_int;
                                            loop {
                                                digits_0 = scale10_round_decimal_long_double(
                                                    arg_7,
                                                    precision_0 as libc::c_int - exponent_3,
                                                );
                                                if digits_0.is_null() {
                                                    let mut _ncw: fpucw_t = oldcw_0;
                                                    asm!(
                                                        "fldcw [{0}]", in (reg) & _ncw, options(preserves_flags)
                                                    );
                                                    'c_12479: {
                                                        let mut _ncw: fpucw_t = oldcw_0;
                                                        asm!(
                                                            "fldcw [{0}]", in (reg) & _ncw, options(preserves_flags)
                                                        );
                                                    };
                                                    current_block = 8340077360075150893;
                                                    break 's_139;
                                                } else {
                                                    ndigits_0 = strlen(digits_0);
                                                    if ndigits_0
                                                        == precision_0
                                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                    {
                                                        break;
                                                    }
                                                    if ndigits_0 < precision_0
                                                        || ndigits_0
                                                            > precision_0
                                                                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                    {
                                                        abort();
                                                    }
                                                    if adjusted != 0 {
                                                        abort();
                                                    }
                                                    free(digits_0 as *mut libc::c_void);
                                                    if ndigits_0 == precision_0 {
                                                        exponent_3 -= 1 as libc::c_int;
                                                    } else {
                                                        exponent_3 += 1 as libc::c_int;
                                                    }
                                                    adjusted = 1 as libc::c_int;
                                                }
                                            }
                                            if is_borderline(digits_0, precision_0) != 0 {
                                                let mut digits2: *mut libc::c_char = scale10_round_decimal_long_double(
                                                    arg_7,
                                                    precision_0 as libc::c_int - exponent_3 + 1 as libc::c_int,
                                                );
                                                if digits2.is_null() {
                                                    free(digits_0 as *mut libc::c_void);
                                                    let mut _ncw: fpucw_t = oldcw_0;
                                                    asm!(
                                                        "fldcw [{0}]", in (reg) & _ncw, options(preserves_flags)
                                                    );
                                                    'c_12369: {
                                                        let mut _ncw: fpucw_t = oldcw_0;
                                                        asm!(
                                                            "fldcw [{0}]", in (reg) & _ncw, options(preserves_flags)
                                                        );
                                                    };
                                                    current_block = 8340077360075150893;
                                                    break;
                                                } else if strlen(digits2)
                                                    == precision_0
                                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                {
                                                    free(digits_0 as *mut libc::c_void);
                                                    digits_0 = digits2;
                                                    exponent_3 -= 1 as libc::c_int;
                                                } else {
                                                    free(digits2 as *mut libc::c_void);
                                                }
                                            }
                                            ndigits_0 = ndigits_0.wrapping_sub(1);
                                            let fresh77 = p_0;
                                            p_0 = p_0.offset(1);
                                            *fresh77 = *digits_0.offset(ndigits_0 as isize);
                                            if flags_0 & 16 as libc::c_int != 0
                                                || precision_0 > 0 as libc::c_int as libc::c_ulong
                                            {
                                                let fresh78 = p_0;
                                                p_0 = p_0.offset(1);
                                                *fresh78 = decimal_point_char();
                                                while ndigits_0 > 0 as libc::c_int as libc::c_ulong {
                                                    ndigits_0 = ndigits_0.wrapping_sub(1);
                                                    ndigits_0;
                                                    let fresh79 = p_0;
                                                    p_0 = p_0.offset(1);
                                                    *fresh79 = *digits_0.offset(ndigits_0 as isize);
                                                }
                                            }
                                            free(digits_0 as *mut libc::c_void);
                                        }
                                        let fresh80 = p_0;
                                        p_0 = p_0.offset(1);
                                        *fresh80 = (*dp).conversion;
                                        if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 1 as libc::c_int as libc::c_ulong
                                        {
                                            sprintf(
                                                p_0,
                                                b"%+.2d\0" as *const u8 as *const libc::c_char,
                                                exponent_3,
                                            );
                                            while *p_0 as libc::c_int != '\0' as i32 {
                                                p_0 = p_0.offset(1);
                                                p_0;
                                            }
                                        } else {
                                            let mut expbuf_1: [libc::c_char; 7] = [0; 7];
                                            let mut ep_1: *const libc::c_char = 0
                                                as *const libc::c_char;
                                            sprintf(
                                                expbuf_1.as_mut_ptr(),
                                                b"%+.2d\0" as *const u8 as *const libc::c_char,
                                                exponent_3,
                                            );
                                            ep_1 = expbuf_1.as_mut_ptr();
                                            loop {
                                                *p_0 = *ep_1;
                                                if !(*p_0 as libc::c_int != '\0' as i32) {
                                                    break;
                                                }
                                                p_0 = p_0.offset(1);
                                                p_0;
                                                ep_1 = ep_1.offset(1);
                                                ep_1;
                                            }
                                        }
                                    } else if (*dp).conversion as libc::c_int == 'g' as i32
                                        || (*dp).conversion as libc::c_int == 'G' as i32
                                    {
                                        if precision_0 == 0 as libc::c_int as libc::c_ulong {
                                            precision_0 = 1 as libc::c_int as size_t;
                                        }
                                        if arg_7 == f128::f128::new(0.0) {
                                            let mut ndigits_1: size_t = precision_0;
                                            let mut nzeroes: size_t = if flags_0 & 16 as libc::c_int
                                                != 0
                                            {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                precision_0.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            };
                                            ndigits_1 = ndigits_1.wrapping_sub(1);
                                            ndigits_1;
                                            let fresh81 = p_0;
                                            p_0 = p_0.offset(1);
                                            *fresh81 = '0' as i32 as libc::c_char;
                                            if flags_0 & 16 as libc::c_int != 0 || ndigits_1 > nzeroes {
                                                let fresh82 = p_0;
                                                p_0 = p_0.offset(1);
                                                *fresh82 = decimal_point_char();
                                                while ndigits_1 > nzeroes {
                                                    ndigits_1 = ndigits_1.wrapping_sub(1);
                                                    ndigits_1;
                                                    let fresh83 = p_0;
                                                    p_0 = p_0.offset(1);
                                                    *fresh83 = '0' as i32 as libc::c_char;
                                                }
                                            }
                                        } else {
                                            let mut exponent_4: libc::c_int = 0;
                                            let mut adjusted_0: libc::c_int = 0;
                                            let mut digits_1: *mut libc::c_char = 0
                                                as *mut libc::c_char;
                                            let mut ndigits_2: size_t = 0;
                                            let mut nzeroes_0: size_t = 0;
                                            exponent_4 = floorlog10l(arg_7);
                                            adjusted_0 = 0 as libc::c_int;
                                            loop {
                                                digits_1 = scale10_round_decimal_long_double(
                                                    arg_7,
                                                    precision_0.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                        as libc::c_int - exponent_4,
                                                );
                                                if digits_1.is_null() {
                                                    let mut _ncw: fpucw_t = oldcw_0;
                                                    asm!(
                                                        "fldcw [{0}]", in (reg) & _ncw, options(preserves_flags)
                                                    );
                                                    'c_11636: {
                                                        let mut _ncw: fpucw_t = oldcw_0;
                                                        asm!(
                                                            "fldcw [{0}]", in (reg) & _ncw, options(preserves_flags)
                                                        );
                                                    };
                                                    current_block = 8340077360075150893;
                                                    break 's_139;
                                                } else {
                                                    ndigits_2 = strlen(digits_1);
                                                    if ndigits_2 == precision_0 {
                                                        break;
                                                    }
                                                    if ndigits_2
                                                        < precision_0
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                        || ndigits_2
                                                            > precision_0
                                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                    {
                                                        abort();
                                                    }
                                                    if adjusted_0 != 0 {
                                                        abort();
                                                    }
                                                    free(digits_1 as *mut libc::c_void);
                                                    if ndigits_2 < precision_0 {
                                                        exponent_4 -= 1 as libc::c_int;
                                                    } else {
                                                        exponent_4 += 1 as libc::c_int;
                                                    }
                                                    adjusted_0 = 1 as libc::c_int;
                                                }
                                            }
                                            if is_borderline(
                                                digits_1,
                                                precision_0.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                            ) != 0
                                            {
                                                let mut digits2_0: *mut libc::c_char = scale10_round_decimal_long_double(
                                                    arg_7,
                                                    precision_0.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                        as libc::c_int - exponent_4 + 1 as libc::c_int,
                                                );
                                                if digits2_0.is_null() {
                                                    free(digits_1 as *mut libc::c_void);
                                                    let mut _ncw: fpucw_t = oldcw_0;
                                                    asm!(
                                                        "fldcw [{0}]", in (reg) & _ncw, options(preserves_flags)
                                                    );
                                                    'c_11523: {
                                                        let mut _ncw: fpucw_t = oldcw_0;
                                                        asm!(
                                                            "fldcw [{0}]", in (reg) & _ncw, options(preserves_flags)
                                                        );
                                                    };
                                                    current_block = 8340077360075150893;
                                                    break;
                                                } else if strlen(digits2_0) == precision_0 {
                                                    free(digits_1 as *mut libc::c_void);
                                                    digits_1 = digits2_0;
                                                    exponent_4 -= 1 as libc::c_int;
                                                } else {
                                                    free(digits2_0 as *mut libc::c_void);
                                                }
                                            }
                                            nzeroes_0 = 0 as libc::c_int as size_t;
                                            if flags_0 & 16 as libc::c_int == 0 as libc::c_int {
                                                while nzeroes_0 < ndigits_2
                                                    && *digits_1.offset(nzeroes_0 as isize) as libc::c_int
                                                        == '0' as i32
                                                {
                                                    nzeroes_0 = nzeroes_0.wrapping_add(1);
                                                    nzeroes_0;
                                                }
                                            }
                                            if exponent_4 >= -(4 as libc::c_int)
                                                && (exponent_4 as libc::c_long)
                                                    < precision_0 as libc::c_long
                                            {
                                                if exponent_4 >= 0 as libc::c_int {
                                                    let mut ecount: size_t = (exponent_4 + 1 as libc::c_int)
                                                        as size_t;
                                                    while ecount > 0 as libc::c_int as libc::c_ulong {
                                                        ndigits_2 = ndigits_2.wrapping_sub(1);
                                                        let fresh84 = p_0;
                                                        p_0 = p_0.offset(1);
                                                        *fresh84 = *digits_1.offset(ndigits_2 as isize);
                                                        ecount = ecount.wrapping_sub(1);
                                                        ecount;
                                                    }
                                                    if flags_0 & 16 as libc::c_int != 0 || ndigits_2 > nzeroes_0
                                                    {
                                                        let fresh85 = p_0;
                                                        p_0 = p_0.offset(1);
                                                        *fresh85 = decimal_point_char();
                                                        while ndigits_2 > nzeroes_0 {
                                                            ndigits_2 = ndigits_2.wrapping_sub(1);
                                                            ndigits_2;
                                                            let fresh86 = p_0;
                                                            p_0 = p_0.offset(1);
                                                            *fresh86 = *digits_1.offset(ndigits_2 as isize);
                                                        }
                                                    }
                                                } else {
                                                    let mut ecount_0: size_t = (-exponent_4 - 1 as libc::c_int)
                                                        as size_t;
                                                    let fresh87 = p_0;
                                                    p_0 = p_0.offset(1);
                                                    *fresh87 = '0' as i32 as libc::c_char;
                                                    let fresh88 = p_0;
                                                    p_0 = p_0.offset(1);
                                                    *fresh88 = decimal_point_char();
                                                    while ecount_0 > 0 as libc::c_int as libc::c_ulong {
                                                        let fresh89 = p_0;
                                                        p_0 = p_0.offset(1);
                                                        *fresh89 = '0' as i32 as libc::c_char;
                                                        ecount_0 = ecount_0.wrapping_sub(1);
                                                        ecount_0;
                                                    }
                                                    while ndigits_2 > nzeroes_0 {
                                                        ndigits_2 = ndigits_2.wrapping_sub(1);
                                                        ndigits_2;
                                                        let fresh90 = p_0;
                                                        p_0 = p_0.offset(1);
                                                        *fresh90 = *digits_1.offset(ndigits_2 as isize);
                                                    }
                                                }
                                            } else {
                                                ndigits_2 = ndigits_2.wrapping_sub(1);
                                                let fresh91 = p_0;
                                                p_0 = p_0.offset(1);
                                                *fresh91 = *digits_1.offset(ndigits_2 as isize);
                                                if flags_0 & 16 as libc::c_int != 0 || ndigits_2 > nzeroes_0
                                                {
                                                    let fresh92 = p_0;
                                                    p_0 = p_0.offset(1);
                                                    *fresh92 = decimal_point_char();
                                                    while ndigits_2 > nzeroes_0 {
                                                        ndigits_2 = ndigits_2.wrapping_sub(1);
                                                        ndigits_2;
                                                        let fresh93 = p_0;
                                                        p_0 = p_0.offset(1);
                                                        *fresh93 = *digits_1.offset(ndigits_2 as isize);
                                                    }
                                                }
                                                let fresh94 = p_0;
                                                p_0 = p_0.offset(1);
                                                *fresh94 = ((*dp).conversion as libc::c_int - 'G' as i32
                                                    + 'E' as i32) as libc::c_char;
                                                if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                                                    == 1 as libc::c_int as libc::c_ulong
                                                {
                                                    sprintf(
                                                        p_0,
                                                        b"%+.2d\0" as *const u8 as *const libc::c_char,
                                                        exponent_4,
                                                    );
                                                    while *p_0 as libc::c_int != '\0' as i32 {
                                                        p_0 = p_0.offset(1);
                                                        p_0;
                                                    }
                                                } else {
                                                    let mut expbuf_2: [libc::c_char; 7] = [0; 7];
                                                    let mut ep_2: *const libc::c_char = 0
                                                        as *const libc::c_char;
                                                    sprintf(
                                                        expbuf_2.as_mut_ptr(),
                                                        b"%+.2d\0" as *const u8 as *const libc::c_char,
                                                        exponent_4,
                                                    );
                                                    ep_2 = expbuf_2.as_mut_ptr();
                                                    loop {
                                                        *p_0 = *ep_2;
                                                        if !(*p_0 as libc::c_int != '\0' as i32) {
                                                            break;
                                                        }
                                                        p_0 = p_0.offset(1);
                                                        p_0;
                                                        ep_2 = ep_2.offset(1);
                                                        ep_2;
                                                    }
                                                }
                                            }
                                            free(digits_1 as *mut libc::c_void);
                                        }
                                    } else {
                                        abort();
                                    }
                                }
                                let mut _ncw: fpucw_t = oldcw_0;
                                asm!(
                                    "fldcw [{0}]", in (reg) & _ncw, options(preserves_flags)
                                );
                                'c_10790: {
                                    let mut _ncw: fpucw_t = oldcw_0;
                                    asm!(
                                        "fldcw [{0}]", in (reg) & _ncw, options(preserves_flags)
                                    );
                                };
                            }
                        } else {
                            let mut arg_8: libc::c_double = (*(a.arg)
                                .offset((*dp).arg_index as isize))
                                .a
                                .a_double;
                            if arg_8.is_nan() as i32 != 0 {
                                if (*dp).conversion as libc::c_int >= 'A' as i32
                                    && (*dp).conversion as libc::c_int <= 'Z' as i32
                                {
                                    let fresh95 = p_0;
                                    p_0 = p_0.offset(1);
                                    *fresh95 = 'N' as i32 as libc::c_char;
                                    let fresh96 = p_0;
                                    p_0 = p_0.offset(1);
                                    *fresh96 = 'A' as i32 as libc::c_char;
                                    let fresh97 = p_0;
                                    p_0 = p_0.offset(1);
                                    *fresh97 = 'N' as i32 as libc::c_char;
                                } else {
                                    let fresh98 = p_0;
                                    p_0 = p_0.offset(1);
                                    *fresh98 = 'n' as i32 as libc::c_char;
                                    let fresh99 = p_0;
                                    p_0 = p_0.offset(1);
                                    *fresh99 = 'a' as i32 as libc::c_char;
                                    let fresh100 = p_0;
                                    p_0 = p_0.offset(1);
                                    *fresh100 = 'n' as i32 as libc::c_char;
                                }
                            } else {
                                let mut sign_2: libc::c_int = 0 as libc::c_int;
                                if if ::core::mem::size_of::<libc::c_double>()
                                    as libc::c_ulong
                                    == ::core::mem::size_of::<f128::f128>() as libc::c_ulong
                                {
                                    (f128::f128::new(arg_8)).is_sign_negative() as libc::c_int
                                } else if ::core::mem::size_of::<libc::c_double>()
                                    as libc::c_ulong
                                    == ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
                                {
                                    arg_8.is_sign_negative() as libc::c_int
                                } else {
                                    (arg_8 as libc::c_float).is_sign_negative() as libc::c_int
                                } != 0
                                {
                                    sign_2 = -(1 as libc::c_int);
                                    arg_8 = -arg_8;
                                }
                                if sign_2 < 0 as libc::c_int {
                                    let fresh101 = p_0;
                                    p_0 = p_0.offset(1);
                                    *fresh101 = '-' as i32 as libc::c_char;
                                } else if flags_0 & 4 as libc::c_int != 0 {
                                    let fresh102 = p_0;
                                    p_0 = p_0.offset(1);
                                    *fresh102 = '+' as i32 as libc::c_char;
                                } else if flags_0 & 8 as libc::c_int != 0 {
                                    let fresh103 = p_0;
                                    p_0 = p_0.offset(1);
                                    *fresh103 = ' ' as i32 as libc::c_char;
                                }
                                if arg_8 > 0.0f64 && arg_8 + arg_8 == arg_8 {
                                    if (*dp).conversion as libc::c_int >= 'A' as i32
                                        && (*dp).conversion as libc::c_int <= 'Z' as i32
                                    {
                                        let fresh104 = p_0;
                                        p_0 = p_0.offset(1);
                                        *fresh104 = 'I' as i32 as libc::c_char;
                                        let fresh105 = p_0;
                                        p_0 = p_0.offset(1);
                                        *fresh105 = 'N' as i32 as libc::c_char;
                                        let fresh106 = p_0;
                                        p_0 = p_0.offset(1);
                                        *fresh106 = 'F' as i32 as libc::c_char;
                                    } else {
                                        let fresh107 = p_0;
                                        p_0 = p_0.offset(1);
                                        *fresh107 = 'i' as i32 as libc::c_char;
                                        let fresh108 = p_0;
                                        p_0 = p_0.offset(1);
                                        *fresh108 = 'n' as i32 as libc::c_char;
                                        let fresh109 = p_0;
                                        p_0 = p_0.offset(1);
                                        *fresh109 = 'f' as i32 as libc::c_char;
                                    }
                                } else {
                                    pad_ptr_0 = p_0;
                                    if (*dp).conversion as libc::c_int == 'f' as i32
                                        || (*dp).conversion as libc::c_int == 'F' as i32
                                    {
                                        let mut digits_2: *mut libc::c_char = 0
                                            as *mut libc::c_char;
                                        let mut ndigits_3: size_t = 0;
                                        digits_2 = scale10_round_decimal_double(
                                            arg_8,
                                            precision_0 as libc::c_int,
                                        );
                                        if digits_2.is_null() {
                                            current_block = 8340077360075150893;
                                            break;
                                        }
                                        ndigits_3 = strlen(digits_2);
                                        if ndigits_3 > precision_0 {
                                            loop {
                                                ndigits_3 = ndigits_3.wrapping_sub(1);
                                                ndigits_3;
                                                let fresh110 = p_0;
                                                p_0 = p_0.offset(1);
                                                *fresh110 = *digits_2.offset(ndigits_3 as isize);
                                                if !(ndigits_3 > precision_0) {
                                                    break;
                                                }
                                            }
                                        } else {
                                            let fresh111 = p_0;
                                            p_0 = p_0.offset(1);
                                            *fresh111 = '0' as i32 as libc::c_char;
                                        }
                                        if flags_0 & 16 as libc::c_int != 0
                                            || precision_0 > 0 as libc::c_int as libc::c_ulong
                                        {
                                            let fresh112 = p_0;
                                            p_0 = p_0.offset(1);
                                            *fresh112 = decimal_point_char();
                                            while precision_0 > ndigits_3 {
                                                let fresh113 = p_0;
                                                p_0 = p_0.offset(1);
                                                *fresh113 = '0' as i32 as libc::c_char;
                                                precision_0 = precision_0.wrapping_sub(1);
                                                precision_0;
                                            }
                                            while ndigits_3 > 0 as libc::c_int as libc::c_ulong {
                                                ndigits_3 = ndigits_3.wrapping_sub(1);
                                                ndigits_3;
                                                let fresh114 = p_0;
                                                p_0 = p_0.offset(1);
                                                *fresh114 = *digits_2.offset(ndigits_3 as isize);
                                            }
                                        }
                                        free(digits_2 as *mut libc::c_void);
                                    } else if (*dp).conversion as libc::c_int == 'e' as i32
                                        || (*dp).conversion as libc::c_int == 'E' as i32
                                    {
                                        let mut exponent_5: libc::c_int = 0;
                                        if arg_8 == 0.0f64 {
                                            exponent_5 = 0 as libc::c_int;
                                            let fresh115 = p_0;
                                            p_0 = p_0.offset(1);
                                            *fresh115 = '0' as i32 as libc::c_char;
                                            if flags_0 & 16 as libc::c_int != 0
                                                || precision_0 > 0 as libc::c_int as libc::c_ulong
                                            {
                                                let fresh116 = p_0;
                                                p_0 = p_0.offset(1);
                                                *fresh116 = decimal_point_char();
                                                while precision_0 > 0 as libc::c_int as libc::c_ulong {
                                                    let fresh117 = p_0;
                                                    p_0 = p_0.offset(1);
                                                    *fresh117 = '0' as i32 as libc::c_char;
                                                    precision_0 = precision_0.wrapping_sub(1);
                                                    precision_0;
                                                }
                                            }
                                        } else {
                                            let mut adjusted_1: libc::c_int = 0;
                                            let mut digits_3: *mut libc::c_char = 0
                                                as *mut libc::c_char;
                                            let mut ndigits_4: size_t = 0;
                                            exponent_5 = floorlog10(arg_8);
                                            adjusted_1 = 0 as libc::c_int;
                                            loop {
                                                digits_3 = scale10_round_decimal_double(
                                                    arg_8,
                                                    precision_0 as libc::c_int - exponent_5,
                                                );
                                                if digits_3.is_null() {
                                                    current_block = 8340077360075150893;
                                                    break 's_139;
                                                }
                                                ndigits_4 = strlen(digits_3);
                                                if ndigits_4
                                                    == precision_0
                                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                {
                                                    break;
                                                }
                                                if ndigits_4 < precision_0
                                                    || ndigits_4
                                                        > precision_0
                                                            .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                {
                                                    abort();
                                                }
                                                if adjusted_1 != 0 {
                                                    abort();
                                                }
                                                free(digits_3 as *mut libc::c_void);
                                                if ndigits_4 == precision_0 {
                                                    exponent_5 -= 1 as libc::c_int;
                                                } else {
                                                    exponent_5 += 1 as libc::c_int;
                                                }
                                                adjusted_1 = 1 as libc::c_int;
                                            }
                                            if is_borderline(digits_3, precision_0) != 0 {
                                                let mut digits2_1: *mut libc::c_char = scale10_round_decimal_double(
                                                    arg_8,
                                                    precision_0 as libc::c_int - exponent_5 + 1 as libc::c_int,
                                                );
                                                if digits2_1.is_null() {
                                                    free(digits_3 as *mut libc::c_void);
                                                    current_block = 8340077360075150893;
                                                    break;
                                                } else if strlen(digits2_1)
                                                    == precision_0
                                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                {
                                                    free(digits_3 as *mut libc::c_void);
                                                    digits_3 = digits2_1;
                                                    exponent_5 -= 1 as libc::c_int;
                                                } else {
                                                    free(digits2_1 as *mut libc::c_void);
                                                }
                                            }
                                            ndigits_4 = ndigits_4.wrapping_sub(1);
                                            let fresh118 = p_0;
                                            p_0 = p_0.offset(1);
                                            *fresh118 = *digits_3.offset(ndigits_4 as isize);
                                            if flags_0 & 16 as libc::c_int != 0
                                                || precision_0 > 0 as libc::c_int as libc::c_ulong
                                            {
                                                let fresh119 = p_0;
                                                p_0 = p_0.offset(1);
                                                *fresh119 = decimal_point_char();
                                                while ndigits_4 > 0 as libc::c_int as libc::c_ulong {
                                                    ndigits_4 = ndigits_4.wrapping_sub(1);
                                                    ndigits_4;
                                                    let fresh120 = p_0;
                                                    p_0 = p_0.offset(1);
                                                    *fresh120 = *digits_3.offset(ndigits_4 as isize);
                                                }
                                            }
                                            free(digits_3 as *mut libc::c_void);
                                        }
                                        let fresh121 = p_0;
                                        p_0 = p_0.offset(1);
                                        *fresh121 = (*dp).conversion;
                                        static mut decimal_format: [libc::c_char; 6] = unsafe {
                                            *::core::mem::transmute::<
                                                &[u8; 6],
                                                &[libc::c_char; 6],
                                            >(b"%+.2d\0")
                                        };
                                        if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 1 as libc::c_int as libc::c_ulong
                                        {
                                            sprintf(p_0, decimal_format.as_ptr(), exponent_5);
                                            while *p_0 as libc::c_int != '\0' as i32 {
                                                p_0 = p_0.offset(1);
                                                p_0;
                                            }
                                        } else {
                                            let mut expbuf_3: [libc::c_char; 7] = [0; 7];
                                            let mut ep_3: *const libc::c_char = 0
                                                as *const libc::c_char;
                                            sprintf(
                                                expbuf_3.as_mut_ptr(),
                                                decimal_format.as_ptr(),
                                                exponent_5,
                                            );
                                            ep_3 = expbuf_3.as_mut_ptr();
                                            loop {
                                                *p_0 = *ep_3;
                                                if !(*p_0 as libc::c_int != '\0' as i32) {
                                                    break;
                                                }
                                                p_0 = p_0.offset(1);
                                                p_0;
                                                ep_3 = ep_3.offset(1);
                                                ep_3;
                                            }
                                        }
                                    } else if (*dp).conversion as libc::c_int == 'g' as i32
                                        || (*dp).conversion as libc::c_int == 'G' as i32
                                    {
                                        if precision_0 == 0 as libc::c_int as libc::c_ulong {
                                            precision_0 = 1 as libc::c_int as size_t;
                                        }
                                        if arg_8 == 0.0f64 {
                                            let mut ndigits_5: size_t = precision_0;
                                            let mut nzeroes_1: size_t = if flags_0 & 16 as libc::c_int
                                                != 0
                                            {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                precision_0.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            };
                                            ndigits_5 = ndigits_5.wrapping_sub(1);
                                            ndigits_5;
                                            let fresh122 = p_0;
                                            p_0 = p_0.offset(1);
                                            *fresh122 = '0' as i32 as libc::c_char;
                                            if flags_0 & 16 as libc::c_int != 0 || ndigits_5 > nzeroes_1
                                            {
                                                let fresh123 = p_0;
                                                p_0 = p_0.offset(1);
                                                *fresh123 = decimal_point_char();
                                                while ndigits_5 > nzeroes_1 {
                                                    ndigits_5 = ndigits_5.wrapping_sub(1);
                                                    ndigits_5;
                                                    let fresh124 = p_0;
                                                    p_0 = p_0.offset(1);
                                                    *fresh124 = '0' as i32 as libc::c_char;
                                                }
                                            }
                                        } else {
                                            let mut exponent_6: libc::c_int = 0;
                                            let mut adjusted_2: libc::c_int = 0;
                                            let mut digits_4: *mut libc::c_char = 0
                                                as *mut libc::c_char;
                                            let mut ndigits_6: size_t = 0;
                                            let mut nzeroes_2: size_t = 0;
                                            exponent_6 = floorlog10(arg_8);
                                            adjusted_2 = 0 as libc::c_int;
                                            loop {
                                                digits_4 = scale10_round_decimal_double(
                                                    arg_8,
                                                    precision_0.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                        as libc::c_int - exponent_6,
                                                );
                                                if digits_4.is_null() {
                                                    current_block = 8340077360075150893;
                                                    break 's_139;
                                                }
                                                ndigits_6 = strlen(digits_4);
                                                if ndigits_6 == precision_0 {
                                                    break;
                                                }
                                                if ndigits_6
                                                    < precision_0
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    || ndigits_6
                                                        > precision_0
                                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                {
                                                    abort();
                                                }
                                                if adjusted_2 != 0 {
                                                    abort();
                                                }
                                                free(digits_4 as *mut libc::c_void);
                                                if ndigits_6 < precision_0 {
                                                    exponent_6 -= 1 as libc::c_int;
                                                } else {
                                                    exponent_6 += 1 as libc::c_int;
                                                }
                                                adjusted_2 = 1 as libc::c_int;
                                            }
                                            if is_borderline(
                                                digits_4,
                                                precision_0.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                            ) != 0
                                            {
                                                let mut digits2_2: *mut libc::c_char = scale10_round_decimal_double(
                                                    arg_8,
                                                    precision_0.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                        as libc::c_int - exponent_6 + 1 as libc::c_int,
                                                );
                                                if digits2_2.is_null() {
                                                    free(digits_4 as *mut libc::c_void);
                                                    current_block = 8340077360075150893;
                                                    break;
                                                } else if strlen(digits2_2) == precision_0 {
                                                    free(digits_4 as *mut libc::c_void);
                                                    digits_4 = digits2_2;
                                                    exponent_6 -= 1 as libc::c_int;
                                                } else {
                                                    free(digits2_2 as *mut libc::c_void);
                                                }
                                            }
                                            nzeroes_2 = 0 as libc::c_int as size_t;
                                            if flags_0 & 16 as libc::c_int == 0 as libc::c_int {
                                                while nzeroes_2 < ndigits_6
                                                    && *digits_4.offset(nzeroes_2 as isize) as libc::c_int
                                                        == '0' as i32
                                                {
                                                    nzeroes_2 = nzeroes_2.wrapping_add(1);
                                                    nzeroes_2;
                                                }
                                            }
                                            if exponent_6 >= -(4 as libc::c_int)
                                                && (exponent_6 as libc::c_long)
                                                    < precision_0 as libc::c_long
                                            {
                                                if exponent_6 >= 0 as libc::c_int {
                                                    let mut ecount_1: size_t = (exponent_6 + 1 as libc::c_int)
                                                        as size_t;
                                                    while ecount_1 > 0 as libc::c_int as libc::c_ulong {
                                                        ndigits_6 = ndigits_6.wrapping_sub(1);
                                                        let fresh125 = p_0;
                                                        p_0 = p_0.offset(1);
                                                        *fresh125 = *digits_4.offset(ndigits_6 as isize);
                                                        ecount_1 = ecount_1.wrapping_sub(1);
                                                        ecount_1;
                                                    }
                                                    if flags_0 & 16 as libc::c_int != 0 || ndigits_6 > nzeroes_2
                                                    {
                                                        let fresh126 = p_0;
                                                        p_0 = p_0.offset(1);
                                                        *fresh126 = decimal_point_char();
                                                        while ndigits_6 > nzeroes_2 {
                                                            ndigits_6 = ndigits_6.wrapping_sub(1);
                                                            ndigits_6;
                                                            let fresh127 = p_0;
                                                            p_0 = p_0.offset(1);
                                                            *fresh127 = *digits_4.offset(ndigits_6 as isize);
                                                        }
                                                    }
                                                } else {
                                                    let mut ecount_2: size_t = (-exponent_6 - 1 as libc::c_int)
                                                        as size_t;
                                                    let fresh128 = p_0;
                                                    p_0 = p_0.offset(1);
                                                    *fresh128 = '0' as i32 as libc::c_char;
                                                    let fresh129 = p_0;
                                                    p_0 = p_0.offset(1);
                                                    *fresh129 = decimal_point_char();
                                                    while ecount_2 > 0 as libc::c_int as libc::c_ulong {
                                                        let fresh130 = p_0;
                                                        p_0 = p_0.offset(1);
                                                        *fresh130 = '0' as i32 as libc::c_char;
                                                        ecount_2 = ecount_2.wrapping_sub(1);
                                                        ecount_2;
                                                    }
                                                    while ndigits_6 > nzeroes_2 {
                                                        ndigits_6 = ndigits_6.wrapping_sub(1);
                                                        ndigits_6;
                                                        let fresh131 = p_0;
                                                        p_0 = p_0.offset(1);
                                                        *fresh131 = *digits_4.offset(ndigits_6 as isize);
                                                    }
                                                }
                                            } else {
                                                ndigits_6 = ndigits_6.wrapping_sub(1);
                                                let fresh132 = p_0;
                                                p_0 = p_0.offset(1);
                                                *fresh132 = *digits_4.offset(ndigits_6 as isize);
                                                if flags_0 & 16 as libc::c_int != 0 || ndigits_6 > nzeroes_2
                                                {
                                                    let fresh133 = p_0;
                                                    p_0 = p_0.offset(1);
                                                    *fresh133 = decimal_point_char();
                                                    while ndigits_6 > nzeroes_2 {
                                                        ndigits_6 = ndigits_6.wrapping_sub(1);
                                                        ndigits_6;
                                                        let fresh134 = p_0;
                                                        p_0 = p_0.offset(1);
                                                        *fresh134 = *digits_4.offset(ndigits_6 as isize);
                                                    }
                                                }
                                                let fresh135 = p_0;
                                                p_0 = p_0.offset(1);
                                                *fresh135 = ((*dp).conversion as libc::c_int - 'G' as i32
                                                    + 'E' as i32) as libc::c_char;
                                                static mut decimal_format_0: [libc::c_char; 6] = unsafe {
                                                    *::core::mem::transmute::<
                                                        &[u8; 6],
                                                        &[libc::c_char; 6],
                                                    >(b"%+.2d\0")
                                                };
                                                if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                                                    == 1 as libc::c_int as libc::c_ulong
                                                {
                                                    sprintf(p_0, decimal_format_0.as_ptr(), exponent_6);
                                                    while *p_0 as libc::c_int != '\0' as i32 {
                                                        p_0 = p_0.offset(1);
                                                        p_0;
                                                    }
                                                } else {
                                                    let mut expbuf_4: [libc::c_char; 7] = [0; 7];
                                                    let mut ep_4: *const libc::c_char = 0
                                                        as *const libc::c_char;
                                                    sprintf(
                                                        expbuf_4.as_mut_ptr(),
                                                        decimal_format_0.as_ptr(),
                                                        exponent_6,
                                                    );
                                                    ep_4 = expbuf_4.as_mut_ptr();
                                                    loop {
                                                        *p_0 = *ep_4;
                                                        if !(*p_0 as libc::c_int != '\0' as i32) {
                                                            break;
                                                        }
                                                        p_0 = p_0.offset(1);
                                                        p_0;
                                                        ep_4 = ep_4.offset(1);
                                                        ep_4;
                                                    }
                                                }
                                            }
                                            free(digits_4 as *mut libc::c_void);
                                        }
                                    } else {
                                        abort();
                                    }
                                }
                            }
                        }
                        count_0 = p_0.offset_from(tmp_0) as libc::c_long as size_t;
                        if count_0 < width_0 {
                            let mut pad_0: size_t = width_0.wrapping_sub(count_0);
                            let mut end_0: *mut libc::c_char = p_0
                                .offset(pad_0 as isize);
                            if flags_0 & 2 as libc::c_int != 0 {
                                while pad_0 > 0 as libc::c_int as libc::c_ulong {
                                    let fresh136 = p_0;
                                    p_0 = p_0.offset(1);
                                    *fresh136 = ' ' as i32 as libc::c_char;
                                    pad_0 = pad_0.wrapping_sub(1);
                                    pad_0;
                                }
                            } else if flags_0 & 32 as libc::c_int != 0
                                && !pad_ptr_0.is_null()
                            {
                                let mut q_3: *mut libc::c_char = end_0;
                                while p_0 > pad_ptr_0 {
                                    p_0 = p_0.offset(-1);
                                    q_3 = q_3.offset(-1);
                                    *q_3 = *p_0;
                                }
                                while pad_0 > 0 as libc::c_int as libc::c_ulong {
                                    let fresh137 = p_0;
                                    p_0 = p_0.offset(1);
                                    *fresh137 = '0' as i32 as libc::c_char;
                                    pad_0 = pad_0.wrapping_sub(1);
                                    pad_0;
                                }
                            } else {
                                let mut q_4: *mut libc::c_char = end_0;
                                while p_0 > tmp_0 {
                                    p_0 = p_0.offset(-1);
                                    q_4 = q_4.offset(-1);
                                    *q_4 = *p_0;
                                }
                                while pad_0 > 0 as libc::c_int as libc::c_ulong {
                                    let fresh138 = p_0;
                                    p_0 = p_0.offset(1);
                                    *fresh138 = ' ' as i32 as libc::c_char;
                                    pad_0 = pad_0.wrapping_sub(1);
                                    pad_0;
                                }
                            }
                            p_0 = end_0;
                        }
                        count_0 = p_0.offset_from(tmp_0) as libc::c_long as size_t;
                        if count_0 >= tmp_length_0 {
                            abort();
                        }
                        if count_0 >= allocated.wrapping_sub(length) {
                            let mut n_1: size_t = xsum(length, count_0);
                            if n_1 > allocated {
                                let mut memory_size_2: size_t = 0;
                                let mut memory_2: *mut libc::c_char = 0
                                    as *mut libc::c_char;
                                allocated = if allocated > 0 as libc::c_int as libc::c_ulong
                                {
                                    if allocated
                                        <= (18446744073709551615 as libc::c_ulong)
                                            .wrapping_div(2 as libc::c_int as libc::c_ulong)
                                    {
                                        allocated.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    } else {
                                        18446744073709551615 as libc::c_ulong
                                    }
                                } else {
                                    12 as libc::c_int as libc::c_ulong
                                };
                                if n_1 > allocated {
                                    allocated = n_1;
                                }
                                memory_size_2 = if allocated
                                    <= (18446744073709551615 as libc::c_ulong)
                                        .wrapping_div(
                                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        )
                                {
                                    allocated
                                        .wrapping_mul(
                                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        )
                                } else {
                                    18446744073709551615 as libc::c_ulong
                                };
                                if memory_size_2 == 18446744073709551615 as libc::c_ulong {
                                    current_block = 8340077360075150893;
                                    break;
                                }
                                if result == resultbuf || result.is_null() {
                                    memory_2 = malloc(memory_size_2) as *mut libc::c_char;
                                } else {
                                    memory_2 = realloc(
                                        result as *mut libc::c_void,
                                        memory_size_2,
                                    ) as *mut libc::c_char;
                                }
                                if memory_2.is_null() {
                                    current_block = 8340077360075150893;
                                    break;
                                }
                                if result == resultbuf
                                    && length > 0 as libc::c_int as libc::c_ulong
                                {
                                    memcpy(
                                        memory_2 as *mut libc::c_void,
                                        result as *const libc::c_void,
                                        length,
                                    );
                                }
                                result = memory_2;
                            }
                        }
                        memcpy(
                            result.offset(length as isize) as *mut libc::c_void,
                            tmp_0 as *const libc::c_void,
                            count_0
                                .wrapping_mul(
                                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                ),
                        );
                        if tmp_0 != tmpbuf_0.as_mut_ptr() {
                            free(tmp_0 as *mut libc::c_void);
                        }
                        length = (length as libc::c_ulong).wrapping_add(count_0)
                            as size_t as size_t;
                    } else {
                        let mut type_2: arg_type = (*(a.arg)
                            .offset((*dp).arg_index as isize))
                            .type_0;
                        let mut flags_1: libc::c_int = (*dp).flags;
                        let mut fbp: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut prefix_count: libc::c_uint = 0;
                        let mut prefixes: [libc::c_int; 2] = [0; 2];
                        let mut orig_errno: libc::c_int = 0;
                        fbp = buf;
                        let fresh139 = fbp;
                        fbp = fbp.offset(1);
                        *fresh139 = '%' as i32 as libc::c_char;
                        if flags_1 & 1 as libc::c_int != 0 {
                            let fresh140 = fbp;
                            fbp = fbp.offset(1);
                            *fresh140 = '\'' as i32 as libc::c_char;
                        }
                        if flags_1 & 2 as libc::c_int != 0 {
                            let fresh141 = fbp;
                            fbp = fbp.offset(1);
                            *fresh141 = '-' as i32 as libc::c_char;
                        }
                        if flags_1 & 4 as libc::c_int != 0 {
                            let fresh142 = fbp;
                            fbp = fbp.offset(1);
                            *fresh142 = '+' as i32 as libc::c_char;
                        }
                        if flags_1 & 8 as libc::c_int != 0 {
                            let fresh143 = fbp;
                            fbp = fbp.offset(1);
                            *fresh143 = ' ' as i32 as libc::c_char;
                        }
                        if flags_1 & 16 as libc::c_int != 0 {
                            let fresh144 = fbp;
                            fbp = fbp.offset(1);
                            *fresh144 = '#' as i32 as libc::c_char;
                        }
                        if flags_1 & 64 as libc::c_int != 0 {
                            let fresh145 = fbp;
                            fbp = fbp.offset(1);
                            *fresh145 = 'I' as i32 as libc::c_char;
                        }
                        if 0 as libc::c_int == 0 {
                            if flags_1 & 32 as libc::c_int != 0 {
                                let fresh146 = fbp;
                                fbp = fbp.offset(1);
                                *fresh146 = '0' as i32 as libc::c_char;
                            }
                            if (*dp).width_start != (*dp).width_end {
                                let mut n_2: size_t = ((*dp).width_end)
                                    .offset_from((*dp).width_start) as libc::c_long as size_t;
                                if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                                    == ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                                {
                                    memcpy(
                                        fbp as *mut libc::c_void,
                                        (*dp).width_start as *const libc::c_void,
                                        n_2
                                            .wrapping_mul(
                                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            ),
                                    );
                                    fbp = fbp.offset(n_2 as isize);
                                } else {
                                    let mut mp: *const libc::c_char = (*dp).width_start;
                                    loop {
                                        let fresh147 = mp;
                                        mp = mp.offset(1);
                                        let fresh148 = fbp;
                                        fbp = fbp.offset(1);
                                        *fresh148 = *fresh147;
                                        n_2 = n_2.wrapping_sub(1);
                                        if !(n_2 > 0 as libc::c_int as libc::c_ulong) {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        if 0 as libc::c_int == 0 {
                            if (*dp).precision_start != (*dp).precision_end {
                                let mut n_3: size_t = ((*dp).precision_end)
                                    .offset_from((*dp).precision_start) as libc::c_long
                                    as size_t;
                                if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                                    == ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                                {
                                    memcpy(
                                        fbp as *mut libc::c_void,
                                        (*dp).precision_start as *const libc::c_void,
                                        n_3
                                            .wrapping_mul(
                                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            ),
                                    );
                                    fbp = fbp.offset(n_3 as isize);
                                } else {
                                    let mut mp_0: *const libc::c_char = (*dp).precision_start;
                                    loop {
                                        let fresh149 = mp_0;
                                        mp_0 = mp_0.offset(1);
                                        let fresh150 = fbp;
                                        fbp = fbp.offset(1);
                                        *fresh150 = *fresh149;
                                        n_3 = n_3.wrapping_sub(1);
                                        if !(n_3 > 0 as libc::c_int as libc::c_ulong) {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        let mut current_block_834: u64;
                        match type_2 as libc::c_uint {
                            9 | 10 => {
                                let fresh151 = fbp;
                                fbp = fbp.offset(1);
                                *fresh151 = 'l' as i32 as libc::c_char;
                                current_block_834 = 8576423635294300709;
                            }
                            7 | 8 | 14 | 16 => {
                                current_block_834 = 8576423635294300709;
                            }
                            12 => {
                                let fresh153 = fbp;
                                fbp = fbp.offset(1);
                                *fresh153 = 'L' as i32 as libc::c_char;
                                current_block_834 = 5808407062788349298;
                            }
                            _ => {
                                current_block_834 = 5808407062788349298;
                            }
                        }
                        match current_block_834 {
                            8576423635294300709 => {
                                let fresh152 = fbp;
                                fbp = fbp.offset(1);
                                *fresh152 = 'l' as i32 as libc::c_char;
                            }
                            _ => {}
                        }
                        *fbp = (*dp).conversion;
                        *fbp
                            .offset(
                                1 as libc::c_int as isize,
                            ) = '\0' as i32 as libc::c_char;
                        prefix_count = 0 as libc::c_int as libc::c_uint;
                        if 0 as libc::c_int == 0
                            && (*dp).width_arg_index != !(0 as libc::c_int as size_t)
                        {
                            if !((*(a.arg).offset((*dp).width_arg_index as isize)).type_0
                                as libc::c_uint == TYPE_INT as libc::c_int as libc::c_uint)
                            {
                                abort();
                            }
                            let fresh154 = prefix_count;
                            prefix_count = prefix_count.wrapping_add(1);
                            prefixes[fresh154
                                as usize] = (*(a.arg)
                                .offset((*dp).width_arg_index as isize))
                                .a
                                .a_int;
                        }
                        if 0 as libc::c_int == 0
                            && (*dp).precision_arg_index != !(0 as libc::c_int as size_t)
                        {
                            if !((*(a.arg).offset((*dp).precision_arg_index as isize))
                                .type_0 as libc::c_uint
                                == TYPE_INT as libc::c_int as libc::c_uint)
                            {
                                abort();
                            }
                            let fresh155 = prefix_count;
                            prefix_count = prefix_count.wrapping_add(1);
                            prefixes[fresh155
                                as usize] = (*(a.arg)
                                .offset((*dp).precision_arg_index as isize))
                                .a
                                .a_int;
                        }
                        if xsum(
                            length,
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(
                                    (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_div(
                                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        ),
                                )
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_div(
                                    (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_div(
                                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        ),
                                ),
                        ) > allocated
                        {
                            let mut memory_size_3: size_t = 0;
                            let mut memory_3: *mut libc::c_char = 0 as *mut libc::c_char;
                            allocated = if allocated > 0 as libc::c_int as libc::c_ulong
                            {
                                if allocated
                                    <= (18446744073709551615 as libc::c_ulong)
                                        .wrapping_div(2 as libc::c_int as libc::c_ulong)
                                {
                                    allocated.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                } else {
                                    18446744073709551615 as libc::c_ulong
                                }
                            } else {
                                12 as libc::c_int as libc::c_ulong
                            };
                            if xsum(
                                length,
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(
                                        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_div(
                                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            ),
                                    )
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_div(
                                        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_div(
                                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            ),
                                    ),
                            ) > allocated
                            {
                                allocated = xsum(
                                    length,
                                    (2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(
                                            (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                                .wrapping_div(
                                                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                ),
                                        )
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_div(
                                            (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                                .wrapping_div(
                                                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                ),
                                        ),
                                );
                            }
                            memory_size_3 = if allocated
                                <= (18446744073709551615 as libc::c_ulong)
                                    .wrapping_div(
                                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                    )
                            {
                                allocated
                                    .wrapping_mul(
                                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                    )
                            } else {
                                18446744073709551615 as libc::c_ulong
                            };
                            if memory_size_3 == 18446744073709551615 as libc::c_ulong {
                                current_block = 8340077360075150893;
                                break;
                            }
                            if result == resultbuf || result.is_null() {
                                memory_3 = malloc(memory_size_3) as *mut libc::c_char;
                            } else {
                                memory_3 = realloc(
                                    result as *mut libc::c_void,
                                    memory_size_3,
                                ) as *mut libc::c_char;
                            }
                            if memory_3.is_null() {
                                current_block = 8340077360075150893;
                                break;
                            }
                            if result == resultbuf
                                && length > 0 as libc::c_int as libc::c_ulong
                            {
                                memcpy(
                                    memory_3 as *mut libc::c_void,
                                    result as *const libc::c_void,
                                    length,
                                );
                            }
                            result = memory_3;
                        }
                        *result.offset(length as isize) = '\0' as i32 as libc::c_char;
                        orig_errno = *__errno_location();
                        loop {
                            let mut count_1: libc::c_int = -(1 as libc::c_int);
                            let mut retcount: libc::c_int = 0 as libc::c_int;
                            let mut maxlen: size_t = allocated.wrapping_sub(length);
                            if maxlen
                                > (2147483647 as libc::c_int as libc::c_ulong)
                                    .wrapping_div(
                                        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_div(
                                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            ),
                                    )
                            {
                                maxlen = (2147483647 as libc::c_int as libc::c_ulong)
                                    .wrapping_div(
                                        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_div(
                                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            ),
                                    );
                            }
                            maxlen = maxlen
                                .wrapping_mul(
                                    (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_div(
                                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        ),
                                );
                            *__errno_location() = 0 as libc::c_int;
                            match type_2 as libc::c_uint {
                                1 => {
                                    let mut arg_9: libc::c_int = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_schar as libc::c_int;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_9,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                arg_9,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                prefixes[1 as libc::c_int as usize],
                                                arg_9,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                2 => {
                                    let mut arg_10: libc::c_uint = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_uchar as libc::c_uint;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_10,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                arg_10,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                prefixes[1 as libc::c_int as usize],
                                                arg_10,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                3 => {
                                    let mut arg_11: libc::c_int = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_short as libc::c_int;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_11,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                arg_11,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                prefixes[1 as libc::c_int as usize],
                                                arg_11,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                4 => {
                                    let mut arg_12: libc::c_uint = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_ushort as libc::c_uint;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_12,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                arg_12,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                prefixes[1 as libc::c_int as usize],
                                                arg_12,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                5 => {
                                    let mut arg_13: libc::c_int = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_int;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_13,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                arg_13,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                prefixes[1 as libc::c_int as usize],
                                                arg_13,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                6 => {
                                    let mut arg_14: libc::c_uint = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_uint;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_14,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                arg_14,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                prefixes[1 as libc::c_int as usize],
                                                arg_14,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                7 => {
                                    let mut arg_15: libc::c_long = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_longint;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_15,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                arg_15,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                prefixes[1 as libc::c_int as usize],
                                                arg_15,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                8 => {
                                    let mut arg_16: libc::c_ulong = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_ulongint;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_16,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                arg_16,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                prefixes[1 as libc::c_int as usize],
                                                arg_16,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                9 => {
                                    let mut arg_17: libc::c_longlong = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_longlongint;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_17,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                arg_17,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                prefixes[1 as libc::c_int as usize],
                                                arg_17,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                10 => {
                                    let mut arg_18: libc::c_ulonglong = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_ulonglongint;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_18,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                arg_18,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                prefixes[1 as libc::c_int as usize],
                                                arg_18,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                11 => {
                                    let mut arg_19: libc::c_double = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_double;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_19,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                arg_19,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                prefixes[1 as libc::c_int as usize],
                                                arg_19,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                12 => {
                                    let mut arg_20: f128::f128 = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_longdouble;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_20,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                arg_20,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                prefixes[1 as libc::c_int as usize],
                                                arg_20,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                13 => {
                                    let mut arg_21: libc::c_int = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_char;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_21,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                arg_21,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                prefixes[1 as libc::c_int as usize],
                                                arg_21,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                14 => {
                                    let mut arg_22: wint_t = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_wide_char;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_22,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                arg_22,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                prefixes[1 as libc::c_int as usize],
                                                arg_22,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                15 => {
                                    let mut arg_23: *const libc::c_char = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_string;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_23,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                arg_23,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                prefixes[1 as libc::c_int as usize],
                                                arg_23,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                16 => {
                                    let mut arg_24: *const wchar_t = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_wide_string;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_24,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                arg_24,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                prefixes[1 as libc::c_int as usize],
                                                arg_24,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                17 => {
                                    let mut arg_25: *mut libc::c_void = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_pointer;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_25,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                arg_25,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as libc::c_int as usize],
                                                prefixes[1 as libc::c_int as usize],
                                                arg_25,
                                                &mut count_1 as *mut libc::c_int,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                _ => {
                                    abort();
                                }
                            }
                            if count_1 >= 0 as libc::c_int {
                                if (count_1 as libc::c_uint as libc::c_ulong) < maxlen
                                    && *result.offset(length as isize).offset(count_1 as isize)
                                        as libc::c_int != '\0' as i32
                                {
                                    abort();
                                }
                                if retcount > count_1 {
                                    count_1 = retcount;
                                }
                            } else if *fbp.offset(1 as libc::c_int as isize)
                                as libc::c_int != '\0' as i32
                            {
                                *fbp
                                    .offset(
                                        1 as libc::c_int as isize,
                                    ) = '\0' as i32 as libc::c_char;
                                continue;
                            } else if !(retcount < 0 as libc::c_int) {
                                count_1 = retcount;
                            }
                            if count_1 < 0 as libc::c_int {
                                if *__errno_location() == 0 as libc::c_int {
                                    if (*dp).conversion as libc::c_int == 'c' as i32
                                        || (*dp).conversion as libc::c_int == 's' as i32
                                    {
                                        *__errno_location() = 84 as libc::c_int;
                                    } else {
                                        *__errno_location() = 22 as libc::c_int;
                                    }
                                }
                                if !(result == resultbuf || result.is_null()) {
                                    free(result as *mut libc::c_void);
                                }
                                if !buf_malloced.is_null() {
                                    free(buf_malloced as *mut libc::c_void);
                                }
                                if d.dir != (d.direct_alloc_dir).as_mut_ptr() {
                                    free(d.dir as *mut libc::c_void);
                                }
                                if a.arg != (a.direct_alloc_arg).as_mut_ptr() {
                                    free(a.arg as *mut libc::c_void);
                                }
                                return 0 as *mut libc::c_char;
                            }
                            if (count_1 as libc::c_uint)
                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                                as libc::c_ulong >= maxlen
                            {
                                if maxlen
                                    == (2147483647 as libc::c_int as libc::c_ulong)
                                        .wrapping_div(
                                            (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                                .wrapping_div(
                                                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                ),
                                        )
                                {
                                    current_block = 14082494282832470779;
                                    break 's_139;
                                }
                                let mut n_4: size_t = xmax(
                                    xsum(
                                        length,
                                        ((count_1 as libc::c_uint)
                                            .wrapping_add(2 as libc::c_int as libc::c_uint)
                                            as libc::c_ulong)
                                            .wrapping_add(
                                                (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                                    .wrapping_div(
                                                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                    ),
                                            )
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_div(
                                                (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                                    .wrapping_div(
                                                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                    ),
                                            ),
                                    ),
                                    if allocated
                                        <= (18446744073709551615 as libc::c_ulong)
                                            .wrapping_div(2 as libc::c_int as libc::c_ulong)
                                    {
                                        allocated.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    } else {
                                        18446744073709551615 as libc::c_ulong
                                    },
                                );
                                if !(n_4 > allocated) {
                                    continue;
                                }
                                let mut memory_size_4: size_t = 0;
                                let mut memory_4: *mut libc::c_char = 0
                                    as *mut libc::c_char;
                                allocated = if allocated > 0 as libc::c_int as libc::c_ulong
                                {
                                    if allocated
                                        <= (18446744073709551615 as libc::c_ulong)
                                            .wrapping_div(2 as libc::c_int as libc::c_ulong)
                                    {
                                        allocated.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    } else {
                                        18446744073709551615 as libc::c_ulong
                                    }
                                } else {
                                    12 as libc::c_int as libc::c_ulong
                                };
                                if n_4 > allocated {
                                    allocated = n_4;
                                }
                                memory_size_4 = if allocated
                                    <= (18446744073709551615 as libc::c_ulong)
                                        .wrapping_div(
                                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        )
                                {
                                    allocated
                                        .wrapping_mul(
                                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        )
                                } else {
                                    18446744073709551615 as libc::c_ulong
                                };
                                if memory_size_4 == 18446744073709551615 as libc::c_ulong {
                                    current_block = 8340077360075150893;
                                    break 's_139;
                                }
                                if result == resultbuf || result.is_null() {
                                    memory_4 = malloc(memory_size_4) as *mut libc::c_char;
                                } else {
                                    memory_4 = realloc(
                                        result as *mut libc::c_void,
                                        memory_size_4,
                                    ) as *mut libc::c_char;
                                }
                                if memory_4.is_null() {
                                    current_block = 8340077360075150893;
                                    break 's_139;
                                }
                                if result == resultbuf
                                    && length > 0 as libc::c_int as libc::c_ulong
                                {
                                    memcpy(
                                        memory_4 as *mut libc::c_void,
                                        result as *const libc::c_void,
                                        length,
                                    );
                                }
                                result = memory_4;
                            } else {
                                length = (length as libc::c_ulong)
                                    .wrapping_add(count_1 as libc::c_ulong) as size_t as size_t;
                                break;
                            }
                        }
                        *__errno_location() = orig_errno;
                    }
                }
                cp = (*dp).dir_end;
                i = i.wrapping_add(1);
                i;
                dp = dp.offset(1);
                dp;
            }
            match current_block {
                701926054148271290 => {
                    if xsum(length, 1 as libc::c_int as size_t) > allocated {
                        let mut memory_size_5: size_t = 0;
                        let mut memory_5: *mut libc::c_char = 0 as *mut libc::c_char;
                        allocated = if allocated > 0 as libc::c_int as libc::c_ulong {
                            if allocated
                                <= (18446744073709551615 as libc::c_ulong)
                                    .wrapping_div(2 as libc::c_int as libc::c_ulong)
                            {
                                allocated.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            } else {
                                18446744073709551615 as libc::c_ulong
                            }
                        } else {
                            12 as libc::c_int as libc::c_ulong
                        };
                        if xsum(length, 1 as libc::c_int as size_t) > allocated {
                            allocated = xsum(length, 1 as libc::c_int as size_t);
                        }
                        memory_size_5 = if allocated
                            <= (18446744073709551615 as libc::c_ulong)
                                .wrapping_div(
                                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                )
                        {
                            allocated
                                .wrapping_mul(
                                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                )
                        } else {
                            18446744073709551615 as libc::c_ulong
                        };
                        if memory_size_5 == 18446744073709551615 as libc::c_ulong {
                            current_block = 8340077360075150893;
                        } else {
                            if result == resultbuf || result.is_null() {
                                memory_5 = malloc(memory_size_5) as *mut libc::c_char;
                            } else {
                                memory_5 = realloc(
                                    result as *mut libc::c_void,
                                    memory_size_5,
                                ) as *mut libc::c_char;
                            }
                            if memory_5.is_null() {
                                current_block = 8340077360075150893;
                            } else {
                                if result == resultbuf
                                    && length > 0 as libc::c_int as libc::c_ulong
                                {
                                    memcpy(
                                        memory_5 as *mut libc::c_void,
                                        result as *const libc::c_void,
                                        length,
                                    );
                                }
                                result = memory_5;
                                current_block = 15394985432210284516;
                            }
                        }
                    } else {
                        current_block = 15394985432210284516;
                    }
                    match current_block {
                        8340077360075150893 => {}
                        _ => {
                            *result
                                .offset(length as isize) = '\0' as i32 as libc::c_char;
                            if result != resultbuf
                                && length.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    < allocated
                            {
                                let mut memory_6: *mut libc::c_char = 0
                                    as *mut libc::c_char;
                                memory_6 = realloc(
                                    result as *mut libc::c_void,
                                    length
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        ),
                                ) as *mut libc::c_char;
                                if !memory_6.is_null() {
                                    result = memory_6;
                                }
                            }
                            if !buf_malloced.is_null() {
                                free(buf_malloced as *mut libc::c_void);
                            }
                            if d.dir != (d.direct_alloc_dir).as_mut_ptr() {
                                free(d.dir as *mut libc::c_void);
                            }
                            if a.arg != (a.direct_alloc_arg).as_mut_ptr() {
                                free(a.arg as *mut libc::c_void);
                            }
                            *lengthp = length;
                            return result;
                        }
                    }
                }
                14082494282832470779 => {
                    if !(result == resultbuf || result.is_null()) {
                        free(result as *mut libc::c_void);
                    }
                    if !buf_malloced.is_null() {
                        free(buf_malloced as *mut libc::c_void);
                    }
                    if d.dir != (d.direct_alloc_dir).as_mut_ptr() {
                        free(d.dir as *mut libc::c_void);
                    }
                    if a.arg != (a.direct_alloc_arg).as_mut_ptr() {
                        free(a.arg as *mut libc::c_void);
                    }
                    *__errno_location() = 75 as libc::c_int;
                    return 0 as *mut libc::c_char;
                }
                _ => {}
            }
            if !(result == resultbuf || result.is_null()) {
                free(result as *mut libc::c_void);
            }
            if !buf_malloced.is_null() {
                free(buf_malloced as *mut libc::c_void);
            }
        }
        _ => {}
    }
    if d.dir != (d.direct_alloc_dir).as_mut_ptr() {
        free(d.dir as *mut libc::c_void);
    }
    if a.arg != (a.direct_alloc_arg).as_mut_ptr() {
        free(a.arg as *mut libc::c_void);
    }
    *__errno_location() = 12 as libc::c_int;
    return 0 as *mut libc::c_char;
}
#[inline]
unsafe extern "C" fn xsum(mut size1: size_t, mut size2: size_t) -> size_t {
    let mut sum: size_t = size1.wrapping_add(size2);
    return if sum >= size1 { sum } else { 18446744073709551615 as libc::c_ulong };
}
#[inline]
unsafe extern "C" fn xmax(mut size1: size_t, mut size2: size_t) -> size_t {
    return if size1 >= size2 { size1 } else { size2 };
}
#[inline]
unsafe extern "C" fn xsum4(
    mut size1: size_t,
    mut size2: size_t,
    mut size3: size_t,
    mut size4: size_t,
) -> size_t {
    return xsum(xsum(xsum(size1, size2), size3), size4);
}
unsafe extern "C" fn decimal_point_char() -> libc::c_char {
    let mut point: *const libc::c_char = 0 as *const libc::c_char;
    point = nl_langinfo(RADIXCHAR as libc::c_int);
    return (if *point.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        *point.offset(0 as libc::c_int as isize) as libc::c_int
    } else {
        '.' as i32
    }) as libc::c_char;
}
unsafe extern "C" fn multiply(
    mut src1: mpn_t,
    mut src2: mpn_t,
    mut dest: *mut mpn_t,
) -> *mut libc::c_void {
    let mut p1: *const mp_limb_t = 0 as *const mp_limb_t;
    let mut p2: *const mp_limb_t = 0 as *const mp_limb_t;
    let mut len1: size_t = 0;
    let mut len2: size_t = 0;
    if src1.nlimbs <= src2.nlimbs {
        len1 = src1.nlimbs;
        p1 = src1.limbs;
        len2 = src2.nlimbs;
        p2 = src2.limbs;
    } else {
        len1 = src2.nlimbs;
        p1 = src2.limbs;
        len2 = src1.nlimbs;
        p2 = src1.limbs;
    }
    if len1 == 0 as libc::c_int as libc::c_ulong {
        (*dest).nlimbs = 0 as libc::c_int as size_t;
        (*dest).limbs = malloc(1 as libc::c_int as libc::c_ulong) as *mut mp_limb_t;
    } else {
        let mut dlen: size_t = 0;
        let mut dp: *mut mp_limb_t = 0 as *mut mp_limb_t;
        let mut k: size_t = 0;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        dlen = len1.wrapping_add(len2);
        dp = malloc(
            dlen.wrapping_mul(::core::mem::size_of::<mp_limb_t>() as libc::c_ulong),
        ) as *mut mp_limb_t;
        if dp.is_null() {
            return 0 as *mut libc::c_void;
        }
        k = len2;
        while k > 0 as libc::c_int as libc::c_ulong {
            k = k.wrapping_sub(1);
            *dp.offset(k as isize) = 0 as libc::c_int as mp_limb_t;
        }
        i = 0 as libc::c_int as size_t;
        while i < len1 {
            let mut digit1: mp_limb_t = *p1.offset(i as isize);
            let mut carry: mp_twolimb_t = 0 as libc::c_int as mp_twolimb_t;
            j = 0 as libc::c_int as size_t;
            while j < len2 {
                let mut digit2: mp_limb_t = *p2.offset(j as isize);
                carry = (carry as libc::c_ulonglong)
                    .wrapping_add(
                        (digit1 as mp_twolimb_t).wrapping_mul(digit2 as mp_twolimb_t),
                    ) as mp_twolimb_t as mp_twolimb_t;
                carry = (carry as libc::c_ulonglong)
                    .wrapping_add(
                        *dp.offset(i.wrapping_add(j) as isize) as libc::c_ulonglong,
                    ) as mp_twolimb_t as mp_twolimb_t;
                *dp.offset(i.wrapping_add(j) as isize) = carry as mp_limb_t;
                carry = carry >> 32 as libc::c_int;
                j = j.wrapping_add(1);
                j;
            }
            *dp.offset(i.wrapping_add(len2) as isize) = carry as mp_limb_t;
            i = i.wrapping_add(1);
            i;
        }
        while dlen > 0 as libc::c_int as libc::c_ulong
            && *dp.offset(dlen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                == 0 as libc::c_int as libc::c_uint
        {
            dlen = dlen.wrapping_sub(1);
            dlen;
        }
        (*dest).nlimbs = dlen;
        (*dest).limbs = dp;
    }
    return (*dest).limbs as *mut libc::c_void;
}
unsafe extern "C" fn divide(
    mut a: mpn_t,
    mut b: mpn_t,
    mut q: *mut mpn_t,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut a_ptr: *const mp_limb_t = a.limbs;
    let mut a_len: size_t = a.nlimbs;
    let mut b_ptr: *const mp_limb_t = b.limbs;
    let mut b_len: size_t = b.nlimbs;
    let mut roomptr: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut tmp_roomptr: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut q_ptr: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut q_len: size_t = 0;
    let mut r_ptr: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut r_len: size_t = 0;
    roomptr = malloc(
        a_len
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<mp_limb_t>() as libc::c_ulong),
    ) as *mut mp_limb_t;
    if roomptr.is_null() {
        return 0 as *mut libc::c_void;
    }
    while a_len > 0 as libc::c_int as libc::c_ulong
        && *a_ptr.offset(a_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            == 0 as libc::c_int as libc::c_uint
    {
        a_len = a_len.wrapping_sub(1);
        a_len;
    }
    loop {
        if b_len == 0 as libc::c_int as libc::c_ulong {
            abort();
        }
        if !(*b_ptr
            .offset(b_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            == 0 as libc::c_int as libc::c_uint)
        {
            break;
        }
        b_len = b_len.wrapping_sub(1);
        b_len;
    }
    if a_len < b_len {
        r_ptr = roomptr;
        r_len = a_len;
        memcpy(
            r_ptr as *mut libc::c_void,
            a_ptr as *const libc::c_void,
            a_len.wrapping_mul(::core::mem::size_of::<mp_limb_t>() as libc::c_ulong),
        );
        q_ptr = roomptr.offset(a_len as isize);
        q_len = 0 as libc::c_int as size_t;
    } else if b_len == 1 as libc::c_int as libc::c_ulong {
        r_ptr = roomptr;
        q_ptr = roomptr.offset(1 as libc::c_int as isize);
        let mut den: mp_limb_t = *b_ptr.offset(0 as libc::c_int as isize);
        let mut rem: mp_limb_t = 0 as libc::c_int as mp_limb_t;
        let mut sourceptr: *const mp_limb_t = a_ptr.offset(a_len as isize);
        let mut destptr: *mut mp_limb_t = q_ptr.offset(a_len as isize);
        let mut count: size_t = 0;
        count = a_len;
        while count > 0 as libc::c_int as libc::c_ulong {
            sourceptr = sourceptr.offset(-1);
            let mut num: mp_twolimb_t = (rem as mp_twolimb_t) << 32 as libc::c_int
                | *sourceptr as libc::c_ulonglong;
            destptr = destptr.offset(-1);
            *destptr = num.wrapping_div(den as libc::c_ulonglong) as mp_limb_t;
            rem = num.wrapping_rem(den as libc::c_ulonglong) as mp_limb_t;
            count = count.wrapping_sub(1);
            count;
        }
        if rem > 0 as libc::c_int as libc::c_uint {
            *r_ptr.offset(0 as libc::c_int as isize) = rem;
            r_len = 1 as libc::c_int as size_t;
        } else {
            r_len = 0 as libc::c_int as size_t;
        }
        q_len = a_len;
        if *q_ptr.offset(q_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            == 0 as libc::c_int as libc::c_uint
        {
            q_len = q_len.wrapping_sub(1);
            q_len;
        }
    } else {
        let mut s: size_t = 0;
        let mut msd: mp_limb_t = *b_ptr
            .offset(b_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        s = msd.leading_zeros() as i32 as size_t;
        if s > 0 as libc::c_int as libc::c_ulong {
            tmp_roomptr = malloc(
                b_len.wrapping_mul(::core::mem::size_of::<mp_limb_t>() as libc::c_ulong),
            ) as *mut mp_limb_t;
            if tmp_roomptr.is_null() {
                free(roomptr as *mut libc::c_void);
                return 0 as *mut libc::c_void;
            }
            let mut sourceptr_0: *const mp_limb_t = b_ptr;
            let mut destptr_0: *mut mp_limb_t = tmp_roomptr;
            let mut accu: mp_twolimb_t = 0 as libc::c_int as mp_twolimb_t;
            let mut count_0: size_t = 0;
            count_0 = b_len;
            while count_0 > 0 as libc::c_int as libc::c_ulong {
                let fresh156 = sourceptr_0;
                sourceptr_0 = sourceptr_0.offset(1);
                accu = (accu as libc::c_ulonglong)
                    .wrapping_add((*fresh156 as mp_twolimb_t) << s) as mp_twolimb_t
                    as mp_twolimb_t;
                let fresh157 = destptr_0;
                destptr_0 = destptr_0.offset(1);
                *fresh157 = accu as mp_limb_t;
                accu = accu >> 32 as libc::c_int;
                count_0 = count_0.wrapping_sub(1);
                count_0;
            }
            if accu != 0 as libc::c_int as libc::c_ulonglong {
                abort();
            }
            b_ptr = tmp_roomptr;
        }
        r_ptr = roomptr;
        if s == 0 as libc::c_int as libc::c_ulong {
            memcpy(
                r_ptr as *mut libc::c_void,
                a_ptr as *const libc::c_void,
                a_len.wrapping_mul(::core::mem::size_of::<mp_limb_t>() as libc::c_ulong),
            );
            *r_ptr.offset(a_len as isize) = 0 as libc::c_int as mp_limb_t;
        } else {
            let mut sourceptr_1: *const mp_limb_t = a_ptr;
            let mut destptr_1: *mut mp_limb_t = r_ptr;
            let mut accu_0: mp_twolimb_t = 0 as libc::c_int as mp_twolimb_t;
            let mut count_1: size_t = 0;
            count_1 = a_len;
            while count_1 > 0 as libc::c_int as libc::c_ulong {
                let fresh158 = sourceptr_1;
                sourceptr_1 = sourceptr_1.offset(1);
                accu_0 = (accu_0 as libc::c_ulonglong)
                    .wrapping_add((*fresh158 as mp_twolimb_t) << s) as mp_twolimb_t
                    as mp_twolimb_t;
                let fresh159 = destptr_1;
                destptr_1 = destptr_1.offset(1);
                *fresh159 = accu_0 as mp_limb_t;
                accu_0 = accu_0 >> 32 as libc::c_int;
                count_1 = count_1.wrapping_sub(1);
                count_1;
            }
            let fresh160 = destptr_1;
            destptr_1 = destptr_1.offset(1);
            *fresh160 = accu_0 as mp_limb_t;
        }
        q_ptr = roomptr.offset(b_len as isize);
        q_len = a_len
            .wrapping_sub(b_len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        let mut j: size_t = a_len.wrapping_sub(b_len);
        let mut b_msd: mp_limb_t = *b_ptr
            .offset(b_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        let mut b_2msd: mp_limb_t = *b_ptr
            .offset(b_len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize);
        let mut b_msdd: mp_twolimb_t = (b_msd as mp_twolimb_t) << 32 as libc::c_int
            | b_2msd as libc::c_ulonglong;
        let mut current_block_95: u64;
        loop {
            let mut q_star: mp_limb_t = 0;
            let mut c1: mp_limb_t = 0;
            if *r_ptr.offset(j.wrapping_add(b_len) as isize) < b_msd {
                let mut num_0: mp_twolimb_t = (*r_ptr
                    .offset(j.wrapping_add(b_len) as isize) as mp_twolimb_t)
                    << 32 as libc::c_int
                    | *r_ptr
                        .offset(
                            j
                                .wrapping_add(b_len)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_ulonglong;
                q_star = num_0.wrapping_div(b_msd as libc::c_ulonglong) as mp_limb_t;
                c1 = num_0.wrapping_rem(b_msd as libc::c_ulonglong) as mp_limb_t;
                current_block_95 = 777662472977924419;
            } else {
                q_star = !(0 as libc::c_int as mp_limb_t);
                if *r_ptr.offset(j.wrapping_add(b_len) as isize) > b_msd
                    || {
                        c1 = (*r_ptr
                            .offset(
                                j
                                    .wrapping_add(b_len)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ))
                            .wrapping_add(b_msd);
                        c1 < b_msd
                    }
                {
                    current_block_95 = 17616119356875028127;
                } else {
                    current_block_95 = 777662472977924419;
                }
            }
            match current_block_95 {
                777662472977924419 => {
                    let mut c2: mp_twolimb_t = (c1 as mp_twolimb_t) << 32 as libc::c_int
                        | *r_ptr
                            .offset(
                                j
                                    .wrapping_add(b_len)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_ulonglong;
                    let mut c3: mp_twolimb_t = (b_2msd as mp_twolimb_t)
                        .wrapping_mul(q_star as mp_twolimb_t);
                    if c3 > c2 {
                        q_star = q_star.wrapping_sub(1 as libc::c_int as libc::c_uint);
                        if c3.wrapping_sub(c2) > b_msdd {
                            q_star = q_star
                                .wrapping_sub(1 as libc::c_int as libc::c_uint);
                        }
                    }
                    if q_star > 0 as libc::c_int as libc::c_uint {
                        current_block_95 = 17616119356875028127;
                    } else {
                        current_block_95 = 6712462580143783635;
                    }
                }
                _ => {}
            }
            match current_block_95 {
                17616119356875028127 => {
                    let mut cr: mp_limb_t = 0;
                    let mut sourceptr_2: *const mp_limb_t = b_ptr;
                    let mut destptr_2: *mut mp_limb_t = r_ptr.offset(j as isize);
                    let mut carry: mp_twolimb_t = 0 as libc::c_int as mp_twolimb_t;
                    let mut count_2: size_t = 0;
                    count_2 = b_len;
                    while count_2 > 0 as libc::c_int as libc::c_ulong {
                        let fresh161 = sourceptr_2;
                        sourceptr_2 = sourceptr_2.offset(1);
                        carry = carry
                            .wrapping_add(
                                (q_star as mp_twolimb_t)
                                    .wrapping_mul(*fresh161 as mp_twolimb_t),
                            )
                            .wrapping_add(!*destptr_2 as libc::c_ulonglong);
                        let fresh162 = destptr_2;
                        destptr_2 = destptr_2.offset(1);
                        *fresh162 = !(carry as mp_limb_t);
                        carry = carry >> 32 as libc::c_int;
                        count_2 = count_2.wrapping_sub(1);
                        count_2;
                    }
                    cr = carry as mp_limb_t;
                    if cr > *r_ptr.offset(j.wrapping_add(b_len) as isize) {
                        q_star = q_star.wrapping_sub(1 as libc::c_int as libc::c_uint);
                        let mut sourceptr_3: *const mp_limb_t = b_ptr;
                        let mut destptr_3: *mut mp_limb_t = r_ptr.offset(j as isize);
                        let mut carry_0: mp_limb_t = 0 as libc::c_int as mp_limb_t;
                        let mut count_3: size_t = 0;
                        count_3 = b_len;
                        while count_3 > 0 as libc::c_int as libc::c_ulong {
                            let fresh163 = sourceptr_3;
                            sourceptr_3 = sourceptr_3.offset(1);
                            let mut source1: mp_limb_t = *fresh163;
                            let mut source2: mp_limb_t = *destptr_3;
                            let fresh164 = destptr_3;
                            destptr_3 = destptr_3.offset(1);
                            *fresh164 = source1
                                .wrapping_add(source2)
                                .wrapping_add(carry_0);
                            carry_0 = (if carry_0 != 0 {
                                (source1 >= !source2) as libc::c_int
                            } else {
                                (source1 > !source2) as libc::c_int
                            }) as mp_limb_t;
                            count_3 = count_3.wrapping_sub(1);
                            count_3;
                        }
                    }
                }
                _ => {}
            }
            *q_ptr.offset(j as isize) = q_star;
            if j == 0 as libc::c_int as libc::c_ulong {
                break;
            }
            j = j.wrapping_sub(1);
            j;
        }
        r_len = b_len;
        if *q_ptr.offset(q_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            == 0 as libc::c_int as libc::c_uint
        {
            q_len = q_len.wrapping_sub(1);
            q_len;
        }
        while r_len > 0 as libc::c_int as libc::c_ulong
            && *r_ptr
                .offset(r_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                == 0 as libc::c_int as libc::c_uint
        {
            r_len = r_len.wrapping_sub(1);
            r_len;
        }
    }
    if r_len > b_len {
        current_block = 11718254377427810743;
    } else {
        let mut i: size_t = 0;
        i = b_len;
        loop {
            let mut r_i: mp_limb_t = (if i <= r_len
                && i > 0 as libc::c_int as libc::c_ulong
            {
                *r_ptr.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    >> 32 as libc::c_int - 1 as libc::c_int
            } else {
                0 as libc::c_int as libc::c_uint
            })
                | (if i < r_len {
                    *r_ptr.offset(i as isize) << 1 as libc::c_int
                } else {
                    0 as libc::c_int as libc::c_uint
                });
            let mut b_i: mp_limb_t = if i < b_len {
                *b_ptr.offset(i as isize)
            } else {
                0 as libc::c_int as libc::c_uint
            };
            if r_i > b_i {
                current_block = 11718254377427810743;
                break;
            }
            if r_i < b_i {
                current_block = 13636302669916279169;
                break;
            }
            if i == 0 as libc::c_int as libc::c_ulong {
                current_block = 1739363794695357236;
                break;
            }
            i = i.wrapping_sub(1);
            i;
        }
        match current_block {
            13636302669916279169 => {}
            11718254377427810743 => {}
            _ => {
                if q_len > 0 as libc::c_int as libc::c_ulong
                    && *q_ptr.offset(0 as libc::c_int as isize)
                        & 1 as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                {
                    current_block = 11718254377427810743;
                } else {
                    current_block = 13636302669916279169;
                }
            }
        }
    }
    match current_block {
        11718254377427810743 => {
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            loop {
                if !(i_0 < q_len) {
                    current_block = 16667286137552459707;
                    break;
                }
                let ref mut fresh165 = *q_ptr.offset(i_0 as isize);
                *fresh165 = (*fresh165).wrapping_add(1);
                if *fresh165 != 0 as libc::c_int as libc::c_uint {
                    current_block = 13636302669916279169;
                    break;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
            match current_block {
                13636302669916279169 => {}
                _ => {
                    let fresh166 = q_len;
                    q_len = q_len.wrapping_add(1);
                    *q_ptr.offset(fresh166 as isize) = 1 as libc::c_int as mp_limb_t;
                }
            }
        }
        _ => {}
    }
    if !tmp_roomptr.is_null() {
        free(tmp_roomptr as *mut libc::c_void);
    }
    (*q).limbs = q_ptr;
    (*q).nlimbs = q_len;
    return roomptr as *mut libc::c_void;
}
unsafe extern "C" fn convert_to_decimal(
    mut a: mpn_t,
    mut extra_zeroes: size_t,
) -> *mut libc::c_char {
    let mut a_ptr: *mut mp_limb_t = a.limbs;
    let mut a_len: size_t = a.nlimbs;
    let mut c_len: size_t = (9 as libc::c_int as libc::c_ulong)
        .wrapping_mul(
            ((a_len as libc::c_float * (32 as libc::c_int as libc::c_float * 0.03345f32))
                as size_t)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    let mut c_ptr: *mut libc::c_char = malloc(
        xsum(xsum(extra_zeroes, c_len), 1 as libc::c_int as size_t),
    ) as *mut libc::c_char;
    if !c_ptr.is_null() {
        let mut d_ptr: *mut libc::c_char = c_ptr;
        while extra_zeroes > 0 as libc::c_int as libc::c_ulong {
            let fresh167 = d_ptr;
            d_ptr = d_ptr.offset(1);
            *fresh167 = '0' as i32 as libc::c_char;
            extra_zeroes = extra_zeroes.wrapping_sub(1);
            extra_zeroes;
        }
        while a_len > 0 as libc::c_int as libc::c_ulong {
            let mut rem: mp_limb_t = 0 as libc::c_int as mp_limb_t;
            let mut ptr: *mut mp_limb_t = a_ptr.offset(a_len as isize);
            let mut count: size_t = 0;
            count = a_len;
            while count > 0 as libc::c_int as libc::c_ulong {
                ptr = ptr.offset(-1);
                let mut num: mp_twolimb_t = (rem as mp_twolimb_t) << 32 as libc::c_int
                    | *ptr as libc::c_ulonglong;
                *ptr = num.wrapping_div(1000000000 as libc::c_int as libc::c_ulonglong)
                    as mp_limb_t;
                rem = num.wrapping_rem(1000000000 as libc::c_int as libc::c_ulonglong)
                    as mp_limb_t;
                count = count.wrapping_sub(1);
                count;
            }
            count = 9 as libc::c_int as size_t;
            while count > 0 as libc::c_int as libc::c_ulong {
                let fresh168 = d_ptr;
                d_ptr = d_ptr.offset(1);
                *fresh168 = ('0' as i32 as libc::c_uint)
                    .wrapping_add(rem.wrapping_rem(10 as libc::c_int as libc::c_uint))
                    as libc::c_char;
                rem = rem.wrapping_div(10 as libc::c_int as libc::c_uint);
                count = count.wrapping_sub(1);
                count;
            }
            if *a_ptr
                .offset(a_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                == 0 as libc::c_int as libc::c_uint
            {
                a_len = a_len.wrapping_sub(1);
                a_len;
            }
        }
        while d_ptr > c_ptr
            && *d_ptr.offset(-(1 as libc::c_int) as isize) as libc::c_int == '0' as i32
        {
            d_ptr = d_ptr.offset(-1);
            d_ptr;
        }
        if d_ptr == c_ptr {
            let fresh169 = d_ptr;
            d_ptr = d_ptr.offset(1);
            *fresh169 = '0' as i32 as libc::c_char;
        }
        *d_ptr = '\0' as i32 as libc::c_char;
    }
    return c_ptr;
}
unsafe extern "C" fn decode_long_double(
    mut x: f128::f128,
    mut ep: *mut libc::c_int,
    mut mp: *mut mpn_t,
) -> *mut libc::c_void {
    let mut m: mpn_t = mpn_t {
        nlimbs: 0,
        limbs: 0 as *mut mp_limb_t,
    };
    let mut expo: libc::c_int = 0;
    let mut y: f128::f128 = f128::f128::ZERO;
    let mut i: size_t = 0;
    m
        .nlimbs = ((64 as libc::c_int + 32 as libc::c_int - 1 as libc::c_int)
        / 32 as libc::c_int) as size_t;
    m
        .limbs = malloc(
        (m.nlimbs).wrapping_mul(::core::mem::size_of::<mp_limb_t>() as libc::c_ulong),
    ) as *mut mp_limb_t;
    if (m.limbs).is_null() {
        return 0 as *mut libc::c_void;
    }
    y = frexpl(x, &mut expo);
    if !(y >= f128::f128::new(0.0) && y < f128::f128::new(1.0)) {
        abort();
    }
    i = (64 as libc::c_int / 32 as libc::c_int) as size_t;
    while i > 0 as libc::c_int as libc::c_ulong {
        let mut hi: mp_limb_t = 0;
        let mut lo: mp_limb_t = 0;
        y
            *= f128::f128::new(
                (1 as libc::c_int as mp_limb_t) << 32 as libc::c_int / 2 as libc::c_int,
            );
        hi = y.to_i32().unwrap() as mp_limb_t;
        y -= f128::f128::new(hi);
        if !(y >= f128::f128::new(0.0) && y < f128::f128::new(1.0)) {
            abort();
        }
        y
            *= f128::f128::new(
                (1 as libc::c_int as mp_limb_t) << 32 as libc::c_int / 2 as libc::c_int,
            );
        lo = y.to_i32().unwrap() as mp_limb_t;
        y -= f128::f128::new(lo);
        if !(y >= f128::f128::new(0.0) && y < f128::f128::new(1.0)) {
            abort();
        }
        i = i.wrapping_sub(1);
        *(m.limbs).offset(i as isize) = hi << 32 as libc::c_int / 2 as libc::c_int | lo;
    }
    while m.nlimbs > 0 as libc::c_int as libc::c_ulong
        && *(m.limbs)
            .offset((m.nlimbs).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            == 0 as libc::c_int as libc::c_uint
    {
        m.nlimbs = (m.nlimbs).wrapping_sub(1);
        m.nlimbs;
    }
    *mp = m;
    *ep = expo - 64 as libc::c_int;
    return m.limbs as *mut libc::c_void;
}
unsafe extern "C" fn decode_double(
    mut x: libc::c_double,
    mut ep: *mut libc::c_int,
    mut mp: *mut mpn_t,
) -> *mut libc::c_void {
    let mut m: mpn_t = mpn_t {
        nlimbs: 0,
        limbs: 0 as *mut mp_limb_t,
    };
    let mut expo: libc::c_int = 0;
    let mut y: libc::c_double = 0.;
    let mut i: size_t = 0;
    m
        .nlimbs = ((53 as libc::c_int + 32 as libc::c_int - 1 as libc::c_int)
        / 32 as libc::c_int) as size_t;
    m
        .limbs = malloc(
        (m.nlimbs).wrapping_mul(::core::mem::size_of::<mp_limb_t>() as libc::c_ulong),
    ) as *mut mp_limb_t;
    if (m.limbs).is_null() {
        return 0 as *mut libc::c_void;
    }
    y = frexp(x, &mut expo);
    if !(y >= 0.0f64 && y < 1.0f64) {
        abort();
    }
    let mut hi: mp_limb_t = 0;
    let mut lo: mp_limb_t = 0;
    y
        *= ((1 as libc::c_int as mp_limb_t)
            << 53 as libc::c_int % (32 as libc::c_int / 2 as libc::c_int))
            as libc::c_double;
    hi = y as libc::c_int as mp_limb_t;
    y -= hi as libc::c_double;
    if !(y >= 0.0f64 && y < 1.0f64) {
        abort();
    }
    y
        *= ((1 as libc::c_int as mp_limb_t) << 32 as libc::c_int / 2 as libc::c_int)
            as libc::c_double;
    lo = y as libc::c_int as mp_limb_t;
    y -= lo as libc::c_double;
    if !(y >= 0.0f64 && y < 1.0f64) {
        abort();
    }
    *(m.limbs)
        .offset(
            (53 as libc::c_int / 32 as libc::c_int) as isize,
        ) = hi << 32 as libc::c_int / 2 as libc::c_int | lo;
    i = (53 as libc::c_int / 32 as libc::c_int) as size_t;
    while i > 0 as libc::c_int as libc::c_ulong {
        let mut hi_0: mp_limb_t = 0;
        let mut lo_0: mp_limb_t = 0;
        y
            *= ((1 as libc::c_int as mp_limb_t) << 32 as libc::c_int / 2 as libc::c_int)
                as libc::c_double;
        hi_0 = y as libc::c_int as mp_limb_t;
        y -= hi_0 as libc::c_double;
        if !(y >= 0.0f64 && y < 1.0f64) {
            abort();
        }
        y
            *= ((1 as libc::c_int as mp_limb_t) << 32 as libc::c_int / 2 as libc::c_int)
                as libc::c_double;
        lo_0 = y as libc::c_int as mp_limb_t;
        y -= lo_0 as libc::c_double;
        if !(y >= 0.0f64 && y < 1.0f64) {
            abort();
        }
        i = i.wrapping_sub(1);
        *(m.limbs)
            .offset(i as isize) = hi_0 << 32 as libc::c_int / 2 as libc::c_int | lo_0;
    }
    if !(y == 0.0f64) {
        abort();
    }
    while m.nlimbs > 0 as libc::c_int as libc::c_ulong
        && *(m.limbs)
            .offset((m.nlimbs).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            == 0 as libc::c_int as libc::c_uint
    {
        m.nlimbs = (m.nlimbs).wrapping_sub(1);
        m.nlimbs;
    }
    *mp = m;
    *ep = expo - 53 as libc::c_int;
    return m.limbs as *mut libc::c_void;
}
unsafe extern "C" fn scale10_round_decimal_decoded(
    mut e: libc::c_int,
    mut m: mpn_t,
    mut memory: *mut libc::c_void,
    mut n: libc::c_int,
) -> *mut libc::c_char {
    let mut s: libc::c_int = 0;
    let mut extra_zeroes: size_t = 0;
    let mut abs_n: libc::c_uint = 0;
    let mut abs_s: libc::c_uint = 0;
    let mut pow5_ptr: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut pow5_len: size_t = 0;
    let mut s_limbs: libc::c_uint = 0;
    let mut s_bits: libc::c_uint = 0;
    let mut pow5: mpn_t = mpn_t {
        nlimbs: 0,
        limbs: 0 as *mut mp_limb_t,
    };
    let mut z: mpn_t = mpn_t {
        nlimbs: 0,
        limbs: 0 as *mut mp_limb_t,
    };
    let mut z_memory: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut digits: *mut libc::c_char = 0 as *mut libc::c_char;
    if memory.is_null() {
        return 0 as *mut libc::c_char;
    }
    s = e + n;
    extra_zeroes = 0 as libc::c_int as size_t;
    if s > 0 as libc::c_int && n > 0 as libc::c_int {
        extra_zeroes = (if s < n { s } else { n }) as size_t;
        s = (s as libc::c_ulong).wrapping_sub(extra_zeroes) as libc::c_int
            as libc::c_int;
        n = (n as libc::c_ulong).wrapping_sub(extra_zeroes) as libc::c_int
            as libc::c_int;
    }
    abs_n = (if n >= 0 as libc::c_int { n } else { -n }) as libc::c_uint;
    abs_s = (if s >= 0 as libc::c_int { s } else { -s }) as libc::c_uint;
    pow5_ptr = malloc(
        ((((abs_n as libc::c_float * (2.322f32 / 32 as libc::c_int as libc::c_float))
            as libc::c_int + 1 as libc::c_int) as libc::c_uint)
            .wrapping_add(abs_s.wrapping_div(32 as libc::c_int as libc::c_uint))
            .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<mp_limb_t>() as libc::c_ulong),
    ) as *mut mp_limb_t;
    if pow5_ptr.is_null() {
        free(memory);
        return 0 as *mut libc::c_char;
    }
    *pow5_ptr.offset(0 as libc::c_int as isize) = 1 as libc::c_int as mp_limb_t;
    pow5_len = 1 as libc::c_int as size_t;
    if abs_n > 0 as libc::c_int as libc::c_uint {
        static mut small_pow5: [mp_limb_t; 14] = [
            1 as libc::c_int as mp_limb_t,
            5 as libc::c_int as mp_limb_t,
            25 as libc::c_int as mp_limb_t,
            125 as libc::c_int as mp_limb_t,
            625 as libc::c_int as mp_limb_t,
            3125 as libc::c_int as mp_limb_t,
            15625 as libc::c_int as mp_limb_t,
            78125 as libc::c_int as mp_limb_t,
            390625 as libc::c_int as mp_limb_t,
            1953125 as libc::c_int as mp_limb_t,
            9765625 as libc::c_int as mp_limb_t,
            48828125 as libc::c_int as mp_limb_t,
            244140625 as libc::c_int as mp_limb_t,
            1220703125 as libc::c_int as mp_limb_t,
        ];
        let mut n13: libc::c_uint = 0;
        n13 = 0 as libc::c_int as libc::c_uint;
        while n13 <= abs_n {
            let mut digit1: mp_limb_t = small_pow5[(if n13
                .wrapping_add(13 as libc::c_int as libc::c_uint) <= abs_n
            {
                13 as libc::c_int as libc::c_uint
            } else {
                abs_n.wrapping_sub(n13)
            }) as usize];
            let mut j: size_t = 0;
            let mut carry: mp_twolimb_t = 0 as libc::c_int as mp_twolimb_t;
            j = 0 as libc::c_int as size_t;
            while j < pow5_len {
                let mut digit2: mp_limb_t = *pow5_ptr.offset(j as isize);
                carry = (carry as libc::c_ulonglong)
                    .wrapping_add(
                        (digit1 as mp_twolimb_t).wrapping_mul(digit2 as mp_twolimb_t),
                    ) as mp_twolimb_t as mp_twolimb_t;
                *pow5_ptr.offset(j as isize) = carry as mp_limb_t;
                carry = carry >> 32 as libc::c_int;
                j = j.wrapping_add(1);
                j;
            }
            if carry > 0 as libc::c_int as libc::c_ulonglong {
                let fresh170 = pow5_len;
                pow5_len = pow5_len.wrapping_add(1);
                *pow5_ptr.offset(fresh170 as isize) = carry as mp_limb_t;
            }
            n13 = n13.wrapping_add(13 as libc::c_int as libc::c_uint);
        }
    }
    s_limbs = abs_s.wrapping_div(32 as libc::c_int as libc::c_uint);
    s_bits = abs_s.wrapping_rem(32 as libc::c_int as libc::c_uint);
    if if n >= 0 as libc::c_int {
        (s >= 0 as libc::c_int) as libc::c_int
    } else {
        (s <= 0 as libc::c_int) as libc::c_int
    } != 0
    {
        if s_bits > 0 as libc::c_int as libc::c_uint {
            let mut ptr: *mut mp_limb_t = pow5_ptr;
            let mut accu: mp_twolimb_t = 0 as libc::c_int as mp_twolimb_t;
            let mut count: size_t = 0;
            count = pow5_len;
            while count > 0 as libc::c_int as libc::c_ulong {
                accu = (accu as libc::c_ulonglong)
                    .wrapping_add((*ptr as mp_twolimb_t) << s_bits) as mp_twolimb_t
                    as mp_twolimb_t;
                let fresh171 = ptr;
                ptr = ptr.offset(1);
                *fresh171 = accu as mp_limb_t;
                accu = accu >> 32 as libc::c_int;
                count = count.wrapping_sub(1);
                count;
            }
            if accu > 0 as libc::c_int as libc::c_ulonglong {
                *ptr = accu as mp_limb_t;
                pow5_len = pow5_len.wrapping_add(1);
                pow5_len;
            }
        }
        if s_limbs > 0 as libc::c_int as libc::c_uint {
            let mut count_0: size_t = 0;
            count_0 = pow5_len;
            while count_0 > 0 as libc::c_int as libc::c_ulong {
                count_0 = count_0.wrapping_sub(1);
                count_0;
                *pow5_ptr
                    .offset(
                        (s_limbs as libc::c_ulong).wrapping_add(count_0) as isize,
                    ) = *pow5_ptr.offset(count_0 as isize);
            }
            count_0 = s_limbs as size_t;
            while count_0 > 0 as libc::c_int as libc::c_ulong {
                count_0 = count_0.wrapping_sub(1);
                count_0;
                *pow5_ptr.offset(count_0 as isize) = 0 as libc::c_int as mp_limb_t;
            }
            pow5_len = (pow5_len as libc::c_ulong).wrapping_add(s_limbs as libc::c_ulong)
                as size_t as size_t;
        }
        pow5.limbs = pow5_ptr;
        pow5.nlimbs = pow5_len;
        if n >= 0 as libc::c_int {
            z_memory = multiply(m, pow5, &mut z);
        } else {
            z_memory = divide(m, pow5, &mut z);
        }
    } else {
        pow5.limbs = pow5_ptr;
        pow5.nlimbs = pow5_len;
        if n >= 0 as libc::c_int {
            let mut numerator: mpn_t = mpn_t {
                nlimbs: 0,
                limbs: 0 as *mut mp_limb_t,
            };
            let mut denominator: mpn_t = mpn_t {
                nlimbs: 0,
                limbs: 0 as *mut mp_limb_t,
            };
            let mut tmp_memory: *mut libc::c_void = 0 as *mut libc::c_void;
            tmp_memory = multiply(m, pow5, &mut numerator);
            if tmp_memory.is_null() {
                free(pow5_ptr as *mut libc::c_void);
                free(memory);
                return 0 as *mut libc::c_char;
            }
            let mut ptr_0: *mut mp_limb_t = pow5_ptr.offset(pow5_len as isize);
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < s_limbs as libc::c_ulong {
                *ptr_0.offset(i as isize) = 0 as libc::c_int as mp_limb_t;
                i = i.wrapping_add(1);
                i;
            }
            *ptr_0.offset(s_limbs as isize) = (1 as libc::c_int as mp_limb_t) << s_bits;
            denominator.limbs = ptr_0;
            denominator
                .nlimbs = s_limbs.wrapping_add(1 as libc::c_int as libc::c_uint)
                as size_t;
            z_memory = divide(numerator, denominator, &mut z);
            free(tmp_memory);
        } else {
            let mut numerator_0: mpn_t = mpn_t {
                nlimbs: 0,
                limbs: 0 as *mut mp_limb_t,
            };
            let mut num_ptr: *mut mp_limb_t = 0 as *mut mp_limb_t;
            num_ptr = malloc(
                (m.nlimbs)
                    .wrapping_add(s_limbs as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<mp_limb_t>() as libc::c_ulong),
            ) as *mut mp_limb_t;
            if num_ptr.is_null() {
                free(pow5_ptr as *mut libc::c_void);
                free(memory);
                return 0 as *mut libc::c_char;
            }
            let mut destptr: *mut mp_limb_t = num_ptr;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < s_limbs as libc::c_ulong {
                let fresh172 = destptr;
                destptr = destptr.offset(1);
                *fresh172 = 0 as libc::c_int as mp_limb_t;
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
            if s_bits > 0 as libc::c_int as libc::c_uint {
                let mut sourceptr: *const mp_limb_t = m.limbs;
                let mut accu_0: mp_twolimb_t = 0 as libc::c_int as mp_twolimb_t;
                let mut count_1: size_t = 0;
                count_1 = m.nlimbs;
                while count_1 > 0 as libc::c_int as libc::c_ulong {
                    let fresh173 = sourceptr;
                    sourceptr = sourceptr.offset(1);
                    accu_0 = (accu_0 as libc::c_ulonglong)
                        .wrapping_add((*fresh173 as mp_twolimb_t) << s_bits)
                        as mp_twolimb_t as mp_twolimb_t;
                    let fresh174 = destptr;
                    destptr = destptr.offset(1);
                    *fresh174 = accu_0 as mp_limb_t;
                    accu_0 = accu_0 >> 32 as libc::c_int;
                    count_1 = count_1.wrapping_sub(1);
                    count_1;
                }
                if accu_0 > 0 as libc::c_int as libc::c_ulonglong {
                    let fresh175 = destptr;
                    destptr = destptr.offset(1);
                    *fresh175 = accu_0 as mp_limb_t;
                }
            } else {
                let mut sourceptr_0: *const mp_limb_t = m.limbs;
                let mut count_2: size_t = 0;
                count_2 = m.nlimbs;
                while count_2 > 0 as libc::c_int as libc::c_ulong {
                    let fresh176 = sourceptr_0;
                    sourceptr_0 = sourceptr_0.offset(1);
                    let fresh177 = destptr;
                    destptr = destptr.offset(1);
                    *fresh177 = *fresh176;
                    count_2 = count_2.wrapping_sub(1);
                    count_2;
                }
            }
            numerator_0.limbs = num_ptr;
            numerator_0.nlimbs = destptr.offset_from(num_ptr) as libc::c_long as size_t;
            z_memory = divide(numerator_0, pow5, &mut z);
            free(num_ptr as *mut libc::c_void);
        }
    }
    free(pow5_ptr as *mut libc::c_void);
    free(memory);
    if z_memory.is_null() {
        return 0 as *mut libc::c_char;
    }
    digits = convert_to_decimal(z, extra_zeroes);
    free(z_memory);
    return digits;
}
unsafe extern "C" fn scale10_round_decimal_long_double(
    mut x: f128::f128,
    mut n: libc::c_int,
) -> *mut libc::c_char {
    let mut e: libc::c_int = 0;
    let mut m: mpn_t = mpn_t {
        nlimbs: 0,
        limbs: 0 as *mut mp_limb_t,
    };
    let mut memory: *mut libc::c_void = decode_long_double(x, &mut e, &mut m);
    return scale10_round_decimal_decoded(e, m, memory, n);
}
unsafe extern "C" fn scale10_round_decimal_double(
    mut x: libc::c_double,
    mut n: libc::c_int,
) -> *mut libc::c_char {
    let mut e: libc::c_int = 0;
    let mut m: mpn_t = mpn_t {
        nlimbs: 0,
        limbs: 0 as *mut mp_limb_t,
    };
    let mut memory: *mut libc::c_void = decode_double(x, &mut e, &mut m);
    return scale10_round_decimal_decoded(e, m, memory, n);
}
unsafe extern "C" fn floorlog10l(mut x: f128::f128) -> libc::c_int {
    let mut expo: libc::c_int = 0;
    let mut y: f128::f128 = f128::f128::ZERO;
    let mut z: libc::c_double = 0.;
    let mut l: libc::c_double = 0.;
    y = frexpl(x, &mut expo);
    if !(y >= f128::f128::new(0.0) && y < f128::f128::new(1.0)) {
        abort();
    }
    if y == f128::f128::new(0.0) {
        return -(2147483647 as libc::c_int) - 1 as libc::c_int;
    }
    if y < f128::f128::new(0.5) {
        while y
            < f128::f128::new(1.0)
                / f128::f128::new(
                    (1 as libc::c_int) << 32 as libc::c_int / 2 as libc::c_int,
                )
                / f128::f128::new(
                    (1 as libc::c_int) << 32 as libc::c_int / 2 as libc::c_int,
                )
        {
            y
                *= f128::f128::new(1.0)
                    * f128::f128::new(
                        (1 as libc::c_int) << 32 as libc::c_int / 2 as libc::c_int,
                    )
                    * f128::f128::new(
                        (1 as libc::c_int) << 32 as libc::c_int / 2 as libc::c_int,
                    );
            expo -= 32 as libc::c_int;
        }
        if y
            < f128::f128::new(1.0)
                / f128::f128::new((1 as libc::c_int) << 16 as libc::c_int)
        {
            y
                *= f128::f128::new(1.0)
                    * f128::f128::new((1 as libc::c_int) << 16 as libc::c_int);
            expo -= 16 as libc::c_int;
        }
        if y
            < f128::f128::new(1.0)
                / f128::f128::new((1 as libc::c_int) << 8 as libc::c_int)
        {
            y
                *= f128::f128::new(1.0)
                    * f128::f128::new((1 as libc::c_int) << 8 as libc::c_int);
            expo -= 8 as libc::c_int;
        }
        if y
            < f128::f128::new(1.0)
                / f128::f128::new((1 as libc::c_int) << 4 as libc::c_int)
        {
            y
                *= f128::f128::new(1.0)
                    * f128::f128::new((1 as libc::c_int) << 4 as libc::c_int);
            expo -= 4 as libc::c_int;
        }
        if y
            < f128::f128::new(1.0)
                / f128::f128::new((1 as libc::c_int) << 2 as libc::c_int)
        {
            y
                *= f128::f128::new(1.0)
                    * f128::f128::new((1 as libc::c_int) << 2 as libc::c_int);
            expo -= 2 as libc::c_int;
        }
        if y
            < f128::f128::new(1.0)
                / f128::f128::new((1 as libc::c_int) << 1 as libc::c_int)
        {
            y
                *= f128::f128::new(1.0)
                    * f128::f128::new((1 as libc::c_int) << 1 as libc::c_int);
            expo -= 1 as libc::c_int;
        }
    }
    if !(y >= f128::f128::new(0.5) && y < f128::f128::new(1.0)) {
        abort();
    }
    l = expo as libc::c_double;
    z = y.to_f64().unwrap();
    if z < 0.70710678118654752444f64 {
        z *= 1.4142135623730950488f64;
        l -= 0.5f64;
    }
    if z < 0.8408964152537145431f64 {
        z *= 1.1892071150027210667f64;
        l -= 0.25f64;
    }
    if z < 0.91700404320467123175f64 {
        z *= 1.0905077326652576592f64;
        l -= 0.125f64;
    }
    if z < 0.9576032806985736469f64 {
        z *= 1.0442737824274138403f64;
        l -= 0.0625f64;
    }
    z = 1 as libc::c_int as libc::c_double - z;
    l
        -= 1.4426950408889634074f64 * z
            * (1.0f64
                + z
                    * (0.5f64
                        + z
                            * (1.0f64 / 3 as libc::c_int as libc::c_double
                                + z * 0.25f64)));
    l *= 0.30102999566398119523f64;
    return l as libc::c_int
        + (if l < 0 as libc::c_int as libc::c_double {
            -(1 as libc::c_int)
        } else {
            0 as libc::c_int
        });
}
unsafe extern "C" fn floorlog10(mut x: libc::c_double) -> libc::c_int {
    let mut expo: libc::c_int = 0;
    let mut y: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    let mut l: libc::c_double = 0.;
    y = frexp(x, &mut expo);
    if !(y >= 0.0f64 && y < 1.0f64) {
        abort();
    }
    if y == 0.0f64 {
        return -(2147483647 as libc::c_int) - 1 as libc::c_int;
    }
    if y < 0.5f64 {
        while y
            < 1.0f64
                / ((1 as libc::c_int) << 32 as libc::c_int / 2 as libc::c_int)
                    as libc::c_double
                / ((1 as libc::c_int) << 32 as libc::c_int / 2 as libc::c_int)
                    as libc::c_double
        {
            y
                *= 1.0f64
                    * ((1 as libc::c_int) << 32 as libc::c_int / 2 as libc::c_int)
                        as libc::c_double
                    * ((1 as libc::c_int) << 32 as libc::c_int / 2 as libc::c_int)
                        as libc::c_double;
            expo -= 32 as libc::c_int;
        }
        if y < 1.0f64 / ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_double {
            y *= 1.0f64 * ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_double;
            expo -= 16 as libc::c_int;
        }
        if y < 1.0f64 / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double {
            y *= 1.0f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double;
            expo -= 8 as libc::c_int;
        }
        if y < 1.0f64 / ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_double {
            y *= 1.0f64 * ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_double;
            expo -= 4 as libc::c_int;
        }
        if y < 1.0f64 / ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_double {
            y *= 1.0f64 * ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_double;
            expo -= 2 as libc::c_int;
        }
        if y < 1.0f64 / ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_double {
            y *= 1.0f64 * ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_double;
            expo -= 1 as libc::c_int;
        }
    }
    if !(y >= 0.5f64 && y < 1.0f64) {
        abort();
    }
    l = expo as libc::c_double;
    z = y;
    if z < 0.70710678118654752444f64 {
        z *= 1.4142135623730950488f64;
        l -= 0.5f64;
    }
    if z < 0.8408964152537145431f64 {
        z *= 1.1892071150027210667f64;
        l -= 0.25f64;
    }
    if z < 0.91700404320467123175f64 {
        z *= 1.0905077326652576592f64;
        l -= 0.125f64;
    }
    if z < 0.9576032806985736469f64 {
        z *= 1.0442737824274138403f64;
        l -= 0.0625f64;
    }
    z = 1 as libc::c_int as libc::c_double - z;
    l
        -= 1.4426950408889634074f64 * z
            * (1.0f64
                + z
                    * (0.5f64
                        + z
                            * (1.0f64 / 3 as libc::c_int as libc::c_double
                                + z * 0.25f64)));
    l *= 0.30102999566398119523f64;
    return l as libc::c_int
        + (if l < 0 as libc::c_int as libc::c_double {
            -(1 as libc::c_int)
        } else {
            0 as libc::c_int
        });
}
unsafe extern "C" fn is_borderline(
    mut digits: *const libc::c_char,
    mut precision: size_t,
) -> libc::c_int {
    while precision > 0 as libc::c_int as libc::c_ulong {
        if *digits as libc::c_int != '0' as i32 {
            return 0 as libc::c_int;
        }
        precision = precision.wrapping_sub(1);
        precision;
        digits = digits.offset(1);
        digits;
    }
    if *digits as libc::c_int != '1' as i32 {
        return 0 as libc::c_int;
    }
    digits = digits.offset(1);
    digits;
    return (*digits as libc::c_int == '\0' as i32) as libc::c_int;
}
