/* automatically generated by rust-bindgen 0.55.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct crypto_sign_vtable {
    pub hash: ::std::option::Option<
        unsafe extern "C" fn(hash: *mut u8, message: *const u8, message_size: usize),
    >,
    pub init: ::std::option::Option<unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void)>,
    pub update: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut ::std::os::raw::c_void,
            message: *const u8,
            message_size: usize,
        ),
    >,
    pub final_: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, hash: *mut u8),
    >,
    pub ctx_size: usize,
}
#[test]
fn bindgen_test_layout_crypto_sign_vtable() {
    assert_eq!(
        ::std::mem::size_of::<crypto_sign_vtable>(),
        40usize,
        concat!("Size of: ", stringify!(crypto_sign_vtable))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_sign_vtable>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_sign_vtable))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_vtable>())).hash as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_vtable),
            "::",
            stringify!(hash)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_vtable>())).init as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_vtable),
            "::",
            stringify!(init)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_vtable>())).update as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_vtable),
            "::",
            stringify!(update)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_vtable>())).final_ as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_vtable),
            "::",
            stringify!(final_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_vtable>())).ctx_size as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_vtable),
            "::",
            stringify!(ctx_size)
        )
    );
}
#[repr(C)]
pub struct crypto_poly1305_ctx {
    pub r: [u32; 4usize],
    pub h: [u32; 5usize],
    pub c: [u32; 5usize],
    pub pad: [u32; 4usize],
    pub c_idx: usize,
}
#[test]
fn bindgen_test_layout_crypto_poly1305_ctx() {
    assert_eq!(
        ::std::mem::size_of::<crypto_poly1305_ctx>(),
        80usize,
        concat!("Size of: ", stringify!(crypto_poly1305_ctx))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_poly1305_ctx>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_poly1305_ctx))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_poly1305_ctx>())).r as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_poly1305_ctx),
            "::",
            stringify!(r)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_poly1305_ctx>())).h as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_poly1305_ctx),
            "::",
            stringify!(h)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_poly1305_ctx>())).c as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_poly1305_ctx),
            "::",
            stringify!(c)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_poly1305_ctx>())).pad as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_poly1305_ctx),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_poly1305_ctx>())).c_idx as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_poly1305_ctx),
            "::",
            stringify!(c_idx)
        )
    );
}
#[repr(C)]
pub struct crypto_blake2b_ctx {
    pub hash: [u64; 8usize],
    pub input_offset: [u64; 2usize],
    pub input: [u64; 16usize],
    pub input_idx: usize,
    pub hash_size: usize,
}
#[test]
fn bindgen_test_layout_crypto_blake2b_ctx() {
    assert_eq!(
        ::std::mem::size_of::<crypto_blake2b_ctx>(),
        224usize,
        concat!("Size of: ", stringify!(crypto_blake2b_ctx))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_blake2b_ctx>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_blake2b_ctx))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_blake2b_ctx>())).hash as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_blake2b_ctx),
            "::",
            stringify!(hash)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_blake2b_ctx>())).input_offset as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_blake2b_ctx),
            "::",
            stringify!(input_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_blake2b_ctx>())).input as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_blake2b_ctx),
            "::",
            stringify!(input)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_blake2b_ctx>())).input_idx as *const _ as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_blake2b_ctx),
            "::",
            stringify!(input_idx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_blake2b_ctx>())).hash_size as *const _ as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_blake2b_ctx),
            "::",
            stringify!(hash_size)
        )
    );
}
#[repr(C)]
pub struct crypto_sign_ctx_abstract {
    pub hash: *const crypto_sign_vtable,
    pub buf: [u8; 96usize],
    pub pk: [u8; 32usize],
}
#[test]
fn bindgen_test_layout_crypto_sign_ctx_abstract() {
    assert_eq!(
        ::std::mem::size_of::<crypto_sign_ctx_abstract>(),
        136usize,
        concat!("Size of: ", stringify!(crypto_sign_ctx_abstract))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_sign_ctx_abstract>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_sign_ctx_abstract))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_ctx_abstract>())).hash as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_ctx_abstract),
            "::",
            stringify!(hash)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_ctx_abstract>())).buf as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_ctx_abstract),
            "::",
            stringify!(buf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_ctx_abstract>())).pk as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_ctx_abstract),
            "::",
            stringify!(pk)
        )
    );
}
pub type crypto_check_ctx_abstract = crypto_sign_ctx_abstract;
#[repr(C)]
pub struct crypto_sign_ctx {
    pub ctx: crypto_sign_ctx_abstract,
    pub hash: crypto_blake2b_ctx,
}
#[test]
fn bindgen_test_layout_crypto_sign_ctx() {
    assert_eq!(
        ::std::mem::size_of::<crypto_sign_ctx>(),
        360usize,
        concat!("Size of: ", stringify!(crypto_sign_ctx))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_sign_ctx>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_sign_ctx))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_ctx>())).ctx as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_ctx),
            "::",
            stringify!(ctx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_ctx>())).hash as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_ctx),
            "::",
            stringify!(hash)
        )
    );
}
pub type crypto_check_ctx = crypto_sign_ctx;
extern "C" {
    pub fn crypto_verify16(a: *const u8, b: *const u8) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_verify32(a: *const u8, b: *const u8) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_verify64(a: *const u8, b: *const u8) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_wipe(secret: *mut ::std::os::raw::c_void, size: usize);
}
extern "C" {
    pub fn crypto_lock(
        mac: *mut u8,
        cipher_text: *mut u8,
        key: *const u8,
        nonce: *const u8,
        plain_text: *const u8,
        text_size: usize,
    );
}
extern "C" {
    pub fn crypto_unlock(
        plain_text: *mut u8,
        key: *const u8,
        nonce: *const u8,
        mac: *const u8,
        cipher_text: *const u8,
        text_size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_lock_aead(
        mac: *mut u8,
        cipher_text: *mut u8,
        key: *const u8,
        nonce: *const u8,
        ad: *const u8,
        ad_size: usize,
        plain_text: *const u8,
        text_size: usize,
    );
}
extern "C" {
    pub fn crypto_unlock_aead(
        plain_text: *mut u8,
        key: *const u8,
        nonce: *const u8,
        mac: *const u8,
        ad: *const u8,
        ad_size: usize,
        cipher_text: *const u8,
        text_size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_blake2b(hash: *mut u8, message: *const u8, message_size: usize);
}
extern "C" {
    pub fn crypto_blake2b_general(
        hash: *mut u8,
        hash_size: usize,
        key: *const u8,
        key_size: usize,
        message: *const u8,
        message_size: usize,
    );
}
extern "C" {
    pub fn crypto_blake2b_init(ctx: *mut crypto_blake2b_ctx);
}
extern "C" {
    pub fn crypto_blake2b_update(
        ctx: *mut crypto_blake2b_ctx,
        message: *const u8,
        message_size: usize,
    );
}
extern "C" {
    pub fn crypto_blake2b_final(ctx: *mut crypto_blake2b_ctx, hash: *mut u8);
}
extern "C" {
    pub fn crypto_blake2b_general_init(
        ctx: *mut crypto_blake2b_ctx,
        hash_size: usize,
        key: *const u8,
        key_size: usize,
    );
}
extern "C" {
    pub static crypto_blake2b_vtable: crypto_sign_vtable;
}
extern "C" {
    pub fn crypto_argon2i(
        hash: *mut u8,
        hash_size: u32,
        work_area: *mut ::std::os::raw::c_void,
        nb_blocks: u32,
        nb_iterations: u32,
        password: *const u8,
        password_size: u32,
        salt: *const u8,
        salt_size: u32,
    );
}
extern "C" {
    pub fn crypto_argon2i_general(
        hash: *mut u8,
        hash_size: u32,
        work_area: *mut ::std::os::raw::c_void,
        nb_blocks: u32,
        nb_iterations: u32,
        password: *const u8,
        password_size: u32,
        salt: *const u8,
        salt_size: u32,
        key: *const u8,
        key_size: u32,
        ad: *const u8,
        ad_size: u32,
    );
}
extern "C" {
    pub fn crypto_key_exchange(
        shared_key: *mut u8,
        your_secret_key: *const u8,
        their_public_key: *const u8,
    );
}
extern "C" {
    pub fn crypto_sign_public_key(public_key: *mut u8, secret_key: *const u8);
}
extern "C" {
    pub fn crypto_sign(
        signature: *mut u8,
        secret_key: *const u8,
        public_key: *const u8,
        message: *const u8,
        message_size: usize,
    );
}
extern "C" {
    pub fn crypto_check(
        signature: *const u8,
        public_key: *const u8,
        message: *const u8,
        message_size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_init_first_pass(
        ctx: *mut crypto_sign_ctx_abstract,
        secret_key: *const u8,
        public_key: *const u8,
    );
}
extern "C" {
    pub fn crypto_sign_update(
        ctx: *mut crypto_sign_ctx_abstract,
        message: *const u8,
        message_size: usize,
    );
}
extern "C" {
    pub fn crypto_sign_init_second_pass(ctx: *mut crypto_sign_ctx_abstract);
}
extern "C" {
    pub fn crypto_sign_final(ctx: *mut crypto_sign_ctx_abstract, signature: *mut u8);
}
extern "C" {
    pub fn crypto_check_init(
        ctx: *mut crypto_check_ctx_abstract,
        signature: *const u8,
        public_key: *const u8,
    );
}
extern "C" {
    pub fn crypto_check_update(
        ctx: *mut crypto_check_ctx_abstract,
        message: *const u8,
        message_size: usize,
    );
}
extern "C" {
    pub fn crypto_check_final(ctx: *mut crypto_check_ctx_abstract) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_public_key_custom_hash(
        public_key: *mut u8,
        secret_key: *const u8,
        hash: *const crypto_sign_vtable,
    );
}
extern "C" {
    pub fn crypto_sign_init_first_pass_custom_hash(
        ctx: *mut crypto_sign_ctx_abstract,
        secret_key: *const u8,
        public_key: *const u8,
        hash: *const crypto_sign_vtable,
    );
}
extern "C" {
    pub fn crypto_check_init_custom_hash(
        ctx: *mut crypto_check_ctx_abstract,
        signature: *const u8,
        public_key: *const u8,
        hash: *const crypto_sign_vtable,
    );
}
extern "C" {
    pub fn crypto_from_eddsa_private(x25519: *mut u8, eddsa: *const u8);
}
extern "C" {
    pub fn crypto_from_eddsa_public(x25519: *mut u8, eddsa: *const u8);
}
extern "C" {
    pub fn crypto_hidden_to_curve(curve: *mut u8, hidden: *const u8);
}
extern "C" {
    pub fn crypto_curve_to_hidden(
        hidden: *mut u8,
        curve: *const u8,
        tweak: u8,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_hidden_key_pair(hidden: *mut u8, secret_key: *mut u8, seed: *mut u8);
}
extern "C" {
    pub fn crypto_hchacha20(out: *mut u8, key: *const u8, in_: *const u8);
}
extern "C" {
    pub fn crypto_chacha20(
        cipher_text: *mut u8,
        plain_text: *const u8,
        text_size: usize,
        key: *const u8,
        nonce: *const u8,
    );
}
extern "C" {
    pub fn crypto_xchacha20(
        cipher_text: *mut u8,
        plain_text: *const u8,
        text_size: usize,
        key: *const u8,
        nonce: *const u8,
    );
}
extern "C" {
    pub fn crypto_ietf_chacha20(
        cipher_text: *mut u8,
        plain_text: *const u8,
        text_size: usize,
        key: *const u8,
        nonce: *const u8,
    );
}
extern "C" {
    pub fn crypto_chacha20_ctr(
        cipher_text: *mut u8,
        plain_text: *const u8,
        text_size: usize,
        key: *const u8,
        nonce: *const u8,
        ctr: u64,
    ) -> u64;
}
extern "C" {
    pub fn crypto_xchacha20_ctr(
        cipher_text: *mut u8,
        plain_text: *const u8,
        text_size: usize,
        key: *const u8,
        nonce: *const u8,
        ctr: u64,
    ) -> u64;
}
extern "C" {
    pub fn crypto_ietf_chacha20_ctr(
        cipher_text: *mut u8,
        plain_text: *const u8,
        text_size: usize,
        key: *const u8,
        nonce: *const u8,
        ctr: u32,
    ) -> u32;
}
extern "C" {
    pub fn crypto_poly1305(mac: *mut u8, message: *const u8, message_size: usize, key: *const u8);
}
extern "C" {
    pub fn crypto_poly1305_init(ctx: *mut crypto_poly1305_ctx, key: *const u8);
}
extern "C" {
    pub fn crypto_poly1305_update(
        ctx: *mut crypto_poly1305_ctx,
        message: *const u8,
        message_size: usize,
    );
}
extern "C" {
    pub fn crypto_poly1305_final(ctx: *mut crypto_poly1305_ctx, mac: *mut u8);
}
extern "C" {
    pub fn crypto_x25519_public_key(public_key: *mut u8, secret_key: *const u8);
}
extern "C" {
    pub fn crypto_x25519(
        raw_shared_secret: *mut u8,
        your_secret_key: *const u8,
        their_public_key: *const u8,
    );
}
extern "C" {
    pub fn crypto_x25519_dirty_small(pk: *mut u8, sk: *const u8);
}
extern "C" {
    pub fn crypto_x25519_dirty_fast(pk: *mut u8, sk: *const u8);
}
extern "C" {
    pub fn crypto_x25519_inverse(
        blind_salt: *mut u8,
        private_key: *const u8,
        curve_point: *const u8,
    );
}