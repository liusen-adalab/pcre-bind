#![allow(non_camel_case_types)]

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Opaque {
    _unused: [u8; 0],
}
type PCRE2_SPTR = *const u8;
type PCRE2_SIZE = usize;
type MutOpaque = *mut Opaque;
type ConstOpaque = *const Opaque;

#[link(name = "pcre2-8")]
extern "C" {
    pub fn pcre2_compile_8(
        pattern: PCRE2_SPTR,
        patlen: PCRE2_SIZE,
        options: u32,
        errorptr: *mut i32,
        erroroffset: *mut PCRE2_SIZE,
        ccontext: *mut Opaque,
    ) -> *mut Opaque;

    pub fn pcre2_match_data_create_from_pattern_8(
        code: *const Opaque,
        gcontext: *const Opaque,
    ) -> *mut Opaque;

    pub fn pcre2_get_ovector_pointer_8(match_data: MutOpaque) -> *const PCRE2_SIZE;

    pub fn pcre2_match_8(
        pcre2_code: ConstOpaque,
        subject: PCRE2_SPTR,
        length: PCRE2_SIZE,
        start_offset: PCRE2_SIZE,
        options: u32,
        match_data: MutOpaque,
        mcontext: MutOpaque,
    ) -> i32;
}

#[rustfmt::skip]
#[allow(dead_code)]
mod options{
    pub const  PCRE2_ALLOW_EMPTY_CLASS   : u32 = 0x00000001;  /* C       */
    pub const  PCRE2_ALT_BSUX            : u32 = 0x00000002;  /* C       */
    pub const  PCRE2_AUTO_CALLOUT        : u32 = 0x00000004;  /* C       */
    pub const  PCRE2_CASELESS            : u32 = 0x00000008;  /* C       */
    pub const  PCRE2_DOLLAR_ENDONLY      : u32 = 0x00000010;  /*   J M D */
    pub const  PCRE2_DOTALL              : u32 = 0x00000020;  /* C       */
    pub const  PCRE2_DUPNAMES            : u32 = 0x00000040;  /* C       */
    pub const  PCRE2_EXTENDED            : u32 = 0x00000080;  /* C       */
    pub const  PCRE2_FIRSTLINE           : u32 = 0x00000100;  /*   J M D */
    pub const  PCRE2_MATCH_UNSET_BACKREF : u32 = 0x00000200;  /* C J M   */
    pub const  PCRE2_MULTILINE           : u32 = 0x00000400;  /* C       */
    pub const  PCRE2_NEVER_UCP           : u32 = 0x00000800;  /* C       */
    pub const  PCRE2_NEVER_UTF           : u32 = 0x00001000;  /* C       */
    pub const  PCRE2_NO_AUTO_CAPTURE     : u32 = 0x00002000;  /* C       */
    pub const  PCRE2_NO_AUTO_POSSESS     : u32 = 0x00004000;  /* C       */
    pub const  PCRE2_NO_DOTSTAR_ANCHOR   : u32 = 0x00008000;  /* C       */
    pub const  PCRE2_NO_START_OPTIMIZE   : u32 = 0x00010000;  /*   J M D */
    pub const  PCRE2_UCP                 : u32 = 0x00020000;  /* C J M D */
    pub const  PCRE2_UNGREEDY            : u32 = 0x00040000;  /* C       */
    pub const  PCRE2_UTF                 : u32 = 0x00080000;  /* C J M D */
    pub const  PCRE2_NEVER_BACKSLASH_C   : u32 = 0x00100000;  /* C       */
    pub const  PCRE2_ALT_CIRCUMFLEX      : u32 = 0x00200000;  /*   J M D */
    pub const  PCRE2_ALT_VERBNAMES       : u32 = 0x00400000;  /* C       */
    pub const  PCRE2_USE_OFFSET_LIMIT    : u32 = 0x00800000;  /*   J M D */
    pub const  PCRE2_EXTENDED_MORE       : u32 = 0x01000000;  /* C       */
    pub const  PCRE2_LITERAL             : u32 = 0x02000000;  /* C       */
    pub const  PCRE2_MATCH_INVALID_UTF   : u32 = 0x04000000;  /*   J M D */
}

pub use options::*;
