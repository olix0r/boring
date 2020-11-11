use libc::*;

use *;

pub const X509_FILETYPE_PEM: c_int = 1;
pub const X509_FILETYPE_ASN1: c_int = 2;
pub const X509_FILETYPE_DEFAULT: c_int = 3;

#[repr(C)]
pub struct X509_VAL {
    pub notBefore: *mut ASN1_TIME,
    pub notAfter: *mut ASN1_TIME,
}

pub enum X509_NAME_ENTRY {}

stack!(stack_st_X509_NAME);

pub enum X509_EXTENSION {}

stack!(stack_st_X509_EXTENSION);

stack!(stack_st_X509_ATTRIBUTE);

pub enum X509_REQ_INFO {}

pub enum X509_CRL {}

stack!(stack_st_X509_CRL);

pub enum X509_CRL_INFO {}

pub enum X509_REVOKED {}

stack!(stack_st_X509_REVOKED);

pub enum X509_REQ {}

pub enum X509_CINF {}

stack!(stack_st_X509);

pub enum X509_OBJECT {}

stack!(stack_st_X509_OBJECT);

pub enum X509_LOOKUP {}

stack!(stack_st_X509_LOOKUP);

extern "C" {
    pub fn X509_verify_cert_error_string(n: c_long) -> *const c_char;

    pub fn X509_sign(x: *mut X509, pkey: *mut EVP_PKEY, md: *const EVP_MD) -> c_int;

    pub fn X509_digest(
        x: *const X509,
        digest: *const EVP_MD,
        buf: *mut c_uchar,
        len: *mut c_uint,
    ) -> c_int;

    pub fn X509_REQ_sign(x: *mut X509_REQ, pkey: *mut EVP_PKEY, md: *const EVP_MD) -> c_int;

    pub fn i2d_X509_bio(b: *mut BIO, x: *mut X509) -> c_int;
    pub fn i2d_X509_REQ_bio(b: *mut BIO, x: *mut X509_REQ) -> c_int;
    pub fn i2d_PrivateKey_bio(b: *mut BIO, x: *mut EVP_PKEY) -> c_int;
    pub fn i2d_PUBKEY_bio(b: *mut BIO, x: *mut EVP_PKEY) -> c_int;

    pub fn i2d_PUBKEY(k: *const EVP_PKEY, buf: *mut *mut u8) -> c_int;
    pub fn d2i_PUBKEY(k: *mut *mut EVP_PKEY, buf: *mut *const u8, len: c_long) -> *mut EVP_PKEY;
    pub fn d2i_RSA_PUBKEY(k: *mut *mut RSA, buf: *mut *const u8, len: c_long) -> *mut RSA;
    pub fn i2d_RSA_PUBKEY(k: *const RSA, buf: *mut *mut u8) -> c_int;
    pub fn d2i_DSA_PUBKEY(k: *mut *mut DSA, pp: *mut *const c_uchar, length: c_long) -> *mut DSA;
    pub fn i2d_DSA_PUBKEY(a: *const DSA, pp: *mut *mut c_uchar) -> c_int;
    pub fn d2i_EC_PUBKEY(
        a: *mut *mut EC_KEY,
        pp: *mut *const c_uchar,
        length: c_long,
    ) -> *mut EC_KEY;
    pub fn i2d_EC_PUBKEY(a: *const EC_KEY, pp: *mut *mut c_uchar) -> c_int;
    pub fn i2d_PrivateKey(k: *const EVP_PKEY, buf: *mut *mut u8) -> c_int;

    pub fn d2i_ECPrivateKey(
        k: *mut *mut EC_KEY,
        pp: *mut *const c_uchar,
        length: c_long,
    ) -> *mut EC_KEY;
    pub fn i2d_ECPrivateKey(ec_key: *const EC_KEY, pp: *mut *mut c_uchar) -> c_int;
}

extern "C" {
    pub fn X509_ALGOR_get0(
        paobj: *mut *const ASN1_OBJECT,
        pptype: *mut c_int,
        ppval: *mut *const c_void,
        alg: *const X509_ALGOR,
    );
}

extern "C" {
    pub fn X509_gmtime_adj(time: *mut ASN1_TIME, adj: c_long) -> *mut ASN1_TIME;

    pub fn X509_to_X509_REQ(x: *mut X509, pkey: *mut EVP_PKEY, md: *const EVP_MD) -> *mut X509_REQ;

    pub fn X509_ALGOR_free(x: *mut X509_ALGOR);

    pub fn X509_REVOKED_new() -> *mut X509_REVOKED;
    pub fn X509_REVOKED_free(x: *mut X509_REVOKED);
    pub fn X509_REVOKED_dup(rev: *mut X509_REVOKED) -> *mut X509_REVOKED;
    pub fn d2i_X509_REVOKED(
        a: *mut *mut X509_REVOKED,
        pp: *mut *const c_uchar,
        length: c_long,
    ) -> *mut X509_REVOKED;
    pub fn i2d_X509_REVOKED(x: *mut X509_REVOKED, buf: *mut *mut u8) -> c_int;
    pub fn X509_CRL_new() -> *mut X509_CRL;
    pub fn X509_CRL_free(x: *mut X509_CRL);
    pub fn d2i_X509_CRL(
        a: *mut *mut X509_CRL,
        pp: *mut *const c_uchar,
        length: c_long,
    ) -> *mut X509_CRL;
    pub fn i2d_X509_CRL(x: *mut X509_CRL, buf: *mut *mut u8) -> c_int;

    pub fn X509_REQ_new() -> *mut X509_REQ;
    pub fn X509_REQ_free(x: *mut X509_REQ);
    pub fn d2i_X509_REQ(
        a: *mut *mut X509_REQ,
        pp: *mut *const c_uchar,
        length: c_long,
    ) -> *mut X509_REQ;
    pub fn i2d_X509_REQ(x: *mut X509_REQ, buf: *mut *mut u8) -> c_int;
}

extern "C" {
    pub fn X509_get0_signature(
        psig: *mut *const ASN1_BIT_STRING,
        palg: *mut *const X509_ALGOR,
        x: *const X509,
    );
}

extern "C" {
    pub fn X509_get_signature_nid(x: *const X509) -> c_int;

    pub fn X509_EXTENSION_free(ext: *mut X509_EXTENSION);

    pub fn X509_NAME_ENTRY_free(x: *mut X509_NAME_ENTRY);

    pub fn X509_NAME_new() -> *mut X509_NAME;
    pub fn X509_NAME_free(x: *mut X509_NAME);

    pub fn X509_new() -> *mut X509;
    pub fn X509_free(x: *mut X509);
    pub fn i2d_X509(x: *mut X509, buf: *mut *mut u8) -> c_int;
    pub fn d2i_X509(a: *mut *mut X509, pp: *mut *const c_uchar, length: c_long) -> *mut X509;

    pub fn X509_get_pubkey(x: *mut X509) -> *mut EVP_PKEY;

    pub fn X509_set_version(x: *mut X509, version: c_long) -> c_int;
    pub fn X509_set_serialNumber(x: *mut X509, sn: *mut ASN1_INTEGER) -> c_int;
    pub fn X509_get_serialNumber(x: *mut X509) -> *mut ASN1_INTEGER;
    pub fn X509_set_issuer_name(x: *mut X509, name: *mut X509_NAME) -> c_int;

    pub fn X509_subject_name_hash(x: *mut ::X509) -> c_ulong;
}

extern "C" {
    pub fn X509_get_issuer_name(x: *const ::X509) -> *mut ::X509_NAME;
}

extern "C" {
    pub fn X509_set_subject_name(x: *mut X509, name: *mut X509_NAME) -> c_int;
}

extern "C" {
    pub fn X509_get_subject_name(x: *const ::X509) -> *mut ::X509_NAME;
}

extern "C" {
    pub fn X509_set1_notBefore(x: *mut ::X509, tm: *const ::ASN1_TIME) -> c_int;
    pub fn X509_set1_notAfter(x: *mut ::X509, tm: *const ::ASN1_TIME) -> c_int;
}

extern "C" {
    pub fn X509_REQ_get_version(req: *const X509_REQ) -> c_long;
    pub fn X509_REQ_set_version(req: *mut X509_REQ, version: c_long) -> c_int;
    pub fn X509_REQ_get_subject_name(req: *const X509_REQ) -> *mut X509_NAME;
    pub fn X509_REQ_set_subject_name(req: *mut X509_REQ, name: *mut X509_NAME) -> c_int;
    pub fn X509_REQ_set_pubkey(req: *mut X509_REQ, pkey: *mut EVP_PKEY) -> c_int;
    pub fn X509_REQ_get_pubkey(req: *mut X509_REQ) -> *mut EVP_PKEY;
    pub fn X509_REQ_get_extensions(req: *mut X509_REQ) -> *mut stack_st_X509_EXTENSION;
    pub fn X509_REQ_add_extensions(req: *mut X509_REQ, exts: *mut stack_st_X509_EXTENSION)
        -> c_int;
    pub fn X509_set_pubkey(x: *mut X509, pkey: *mut EVP_PKEY) -> c_int;
    pub fn X509_REQ_verify(req: *mut X509_REQ, pkey: *mut EVP_PKEY) -> c_int;
    pub fn X509_getm_notBefore(x: *mut X509) -> *mut ASN1_TIME;
    pub fn X509_getm_notAfter(x: *mut X509) -> *mut ASN1_TIME;
    pub fn X509_up_ref(x: *mut X509) -> c_int;

    pub fn X509_REVOKED_get0_serialNumber(req: *const X509_REVOKED) -> *const ASN1_INTEGER;
    pub fn X509_REVOKED_get0_revocationDate(req: *const X509_REVOKED) -> *const ASN1_TIME;
    pub fn X509_REVOKED_get0_extensions(r: *const X509_REVOKED) -> *const stack_st_X509_EXTENSION;

    pub fn X509_REVOKED_set_serialNumber(r: *mut X509_REVOKED, serial: *mut ASN1_INTEGER) -> c_int;
    pub fn X509_REVOKED_set_revocationDate(r: *mut X509_REVOKED, tm: *mut ASN1_TIME) -> c_int;

    pub fn X509_CRL_sign(x: *mut X509_CRL, pkey: *mut EVP_PKEY, md: *const EVP_MD) -> c_int;
    pub fn X509_CRL_digest(
        x: *const X509_CRL,
        digest: *const EVP_MD,
        md: *mut c_uchar,
        len: *mut c_uint,
    ) -> c_int;
    pub fn X509_CRL_verify(crl: *mut X509_CRL, pkey: *mut EVP_PKEY) -> c_int;
    pub fn X509_CRL_get0_by_cert(
        x: *mut X509_CRL,
        ret: *mut *mut X509_REVOKED,
        cert: *mut X509,
    ) -> c_int;
    pub fn X509_CRL_get0_by_serial(
        x: *mut X509_CRL,
        ret: *mut *mut X509_REVOKED,
        serial: *mut ASN1_INTEGER,
    ) -> c_int;

    pub fn X509_CRL_get_REVOKED(crl: *mut X509_CRL) -> *mut stack_st_X509_REVOKED;
    pub fn X509_CRL_get0_nextUpdate(x: *const X509_CRL) -> *const ASN1_TIME;
    pub fn X509_CRL_get0_lastUpdate(x: *const X509_CRL) -> *const ASN1_TIME;
    pub fn X509_CRL_get_issuer(x: *const X509_CRL) -> *mut X509_NAME;

    pub fn X509_get0_extensions(req: *const ::X509) -> *const stack_st_X509_EXTENSION;

    pub fn X509_CRL_set_version(crl: *mut X509_CRL, version: c_long) -> c_int;
    pub fn X509_CRL_set_issuer_name(crl: *mut X509_CRL, name: *mut X509_NAME) -> c_int;
    pub fn X509_CRL_sort(crl: *mut X509_CRL) -> c_int;

    pub fn X509_CRL_up_ref(crl: *mut X509_CRL) -> c_int;
    pub fn X509_CRL_add0_revoked(crl: *mut X509_CRL, rev: *mut X509_REVOKED) -> c_int;
}

extern "C" {
    pub fn X509_CRL_set1_lastUpdate(crl: *mut X509_CRL, tm: *const ASN1_TIME) -> c_int;
    pub fn X509_CRL_set1_nextUpdate(crl: *mut X509_CRL, tm: *const ASN1_TIME) -> c_int;
}

extern "C" {
    pub fn X509_NAME_entry_count(n: *const X509_NAME) -> c_int;
}

extern "C" {
    pub fn X509_NAME_get_index_by_NID(n: *const X509_NAME, nid: c_int, last_pos: c_int) -> c_int;
}

extern "C" {
    pub fn X509_NAME_get_entry(n: *const X509_NAME, loc: c_int) -> *mut X509_NAME_ENTRY;
    pub fn X509_NAME_add_entry_by_NID(
        x: *mut X509_NAME,
        field: c_int,
        ty: c_int,
        bytes: *const c_uchar,
        len: c_int,
        loc: c_int,
        set: c_int,
    ) -> c_int;
    pub fn X509_NAME_ENTRY_get_object(ne: *const X509_NAME_ENTRY) -> *mut ASN1_OBJECT;
    pub fn X509_NAME_ENTRY_get_data(ne: *const X509_NAME_ENTRY) -> *mut ASN1_STRING;
}

extern "C" {
    pub fn X509_NAME_add_entry_by_txt(
        x: *mut X509_NAME,
        field: *const c_char,
        ty: c_int,
        bytes: *const c_uchar,
        len: c_int,
        loc: c_int,
        set: c_int,
    ) -> c_int;
}

// "raw" X509_EXTENSION related functions
extern "C" {
    // in X509
    pub fn X509_delete_ext(x: *mut X509, loc: c_int) -> *mut X509_EXTENSION;
    pub fn X509_add_ext(x: *mut X509, ext: *mut X509_EXTENSION, loc: c_int) -> c_int;
    pub fn X509_add1_ext_i2d(
        x: *mut X509,
        nid: c_int,
        value: *mut c_void,
        crit: c_int,
        flags: c_ulong,
    ) -> c_int;
    // in X509_CRL
    pub fn X509_CRL_delete_ext(x: *mut X509_CRL, loc: c_int) -> *mut X509_EXTENSION;
    pub fn X509_CRL_add_ext(x: *mut X509_CRL, ext: *mut X509_EXTENSION, loc: c_int) -> c_int;
    pub fn X509_CRL_add1_ext_i2d(
        x: *mut X509_CRL,
        nid: c_int,
        value: *mut c_void,
        crit: c_int,
        flags: c_ulong,
    ) -> c_int;
    // in X509_REVOKED
    pub fn X509_REVOKED_delete_ext(x: *mut X509_REVOKED, loc: c_int) -> *mut X509_EXTENSION;
    pub fn X509_REVOKED_add_ext(
        x: *mut X509_REVOKED,
        ext: *mut X509_EXTENSION,
        loc: c_int,
    ) -> c_int;
    pub fn X509_REVOKED_add1_ext_i2d(
        x: *mut X509_REVOKED,
        nid: c_int,
        value: *mut c_void,
        crit: c_int,
        flags: c_ulong,
    ) -> c_int;
    // X509_EXTENSION stack
    // - these getters always used *const STACK
    pub fn X509v3_get_ext_count(x: *const stack_st_X509_EXTENSION) -> c_int;
    pub fn X509v3_get_ext_by_NID(
        x: *const stack_st_X509_EXTENSION,
        nid: c_int,
        lastpos: c_int,
    ) -> c_int;
    pub fn X509v3_get_ext_by_critical(
        x: *const stack_st_X509_EXTENSION,
        crit: c_int,
        lastpos: c_int,
    ) -> c_int;
    pub fn X509v3_get_ext(x: *const stack_st_X509_EXTENSION, loc: c_int) -> *mut X509_EXTENSION;
    pub fn X509v3_delete_ext(x: *mut stack_st_X509_EXTENSION, loc: c_int) -> *mut X509_EXTENSION;
    pub fn X509v3_add_ext(
        x: *mut *mut stack_st_X509_EXTENSION,
        ex: *mut X509_EXTENSION,
        loc: c_int,
    ) -> *mut stack_st_X509_EXTENSION;
    // - X509V3_add1_i2d in x509v3.rs
    // X509_EXTENSION itself
    pub fn X509_EXTENSION_create_by_NID(
        ex: *mut *mut X509_EXTENSION,
        nid: c_int,
        crit: c_int,
        data: *const ASN1_OCTET_STRING,
    ) -> *mut X509_EXTENSION;
    pub fn X509_EXTENSION_set_critical(ex: *mut X509_EXTENSION, crit: c_int) -> c_int;
    pub fn X509_EXTENSION_set_data(
        ex: *mut X509_EXTENSION,
        data: *const ASN1_OCTET_STRING,
    ) -> c_int;
    pub fn X509_EXTENSION_get_object(ext: *mut X509_EXTENSION) -> *mut ASN1_OBJECT;
    pub fn X509_EXTENSION_get_data(ext: *mut X509_EXTENSION) -> *mut ASN1_OCTET_STRING;
}

extern "C" {
    // in X509
    pub fn X509_get_ext_count(x: *const X509) -> c_int;
    pub fn X509_get_ext_by_NID(x: *const X509, nid: c_int, lastpos: c_int) -> c_int;
    pub fn X509_get_ext_by_OBJ(x: *const X509, obj: *const ASN1_OBJECT, lastpos: c_int) -> c_int;
    pub fn X509_get_ext_by_critical(x: *const X509, crit: c_int, lastpos: c_int) -> c_int;
    pub fn X509_get_ext(x: *const X509, loc: c_int) -> *mut X509_EXTENSION;
    pub fn X509_get_ext_d2i(
        x: *const ::X509,
        nid: c_int,
        crit: *mut c_int,
        idx: *mut c_int,
    ) -> *mut c_void;
    // in X509_CRL
    pub fn X509_CRL_get_ext_count(x: *const X509_CRL) -> c_int;
    pub fn X509_CRL_get_ext_by_NID(x: *const X509_CRL, nid: c_int, lastpos: c_int) -> c_int;
    pub fn X509_CRL_get_ext_by_OBJ(
        x: *const X509_CRL,
        obj: *const ASN1_OBJECT,
        lastpos: c_int,
    ) -> c_int;
    pub fn X509_CRL_get_ext_by_critical(x: *const X509_CRL, crit: c_int, lastpos: c_int) -> c_int;
    pub fn X509_CRL_get_ext(x: *const X509_CRL, loc: c_int) -> *mut X509_EXTENSION;
    pub fn X509_CRL_get_ext_d2i(
        x: *const ::X509_CRL,
        nid: c_int,
        crit: *mut c_int,
        idx: *mut c_int,
    ) -> *mut c_void;
    // in X509_REVOKED
    pub fn X509_REVOKED_get_ext_count(x: *const X509_REVOKED) -> c_int;
    pub fn X509_REVOKED_get_ext_by_NID(x: *const X509_REVOKED, nid: c_int, lastpos: c_int)
        -> c_int;
    pub fn X509_REVOKED_get_ext_by_OBJ(
        x: *const X509_REVOKED,
        obj: *const ASN1_OBJECT,
        lastpos: c_int,
    ) -> c_int;
    pub fn X509_REVOKED_get_ext_by_critical(
        x: *const X509_REVOKED,
        crit: c_int,
        lastpos: c_int,
    ) -> c_int;
    pub fn X509_REVOKED_get_ext(x: *const X509_REVOKED, loc: c_int) -> *mut X509_EXTENSION;
    pub fn X509_REVOKED_get_ext_d2i(
        x: *const ::X509_REVOKED,
        nid: c_int,
        crit: *mut c_int,
        idx: *mut c_int,
    ) -> *mut c_void;
    // X509_EXTENSION stack
    pub fn X509v3_get_ext_by_OBJ(
        x: *const stack_st_X509_EXTENSION,
        obj: *const ASN1_OBJECT,
        lastpos: c_int,
    ) -> c_int;
    // X509_EXTENSION itself
    pub fn X509_EXTENSION_create_by_OBJ(
        ex: *mut *mut X509_EXTENSION,
        obj: *const ASN1_OBJECT,
        crit: c_int,
        data: *const ASN1_OCTET_STRING,
    ) -> *mut X509_EXTENSION;
    pub fn X509_EXTENSION_set_object(ex: *mut X509_EXTENSION, obj: *const ASN1_OBJECT) -> c_int;
    pub fn X509_EXTENSION_get_critical(ex: *mut X509_EXTENSION) -> c_int;
}

extern "C" {
    pub fn X509_verify_cert(ctx: *mut X509_STORE_CTX) -> c_int;
}

extern "C" {
    pub fn X509_STORE_get0_objects(ctx: *mut X509_STORE) -> *mut stack_st_X509_OBJECT;
    pub fn X509_OBJECT_get0_X509(x: *const X509_OBJECT) -> *mut X509;
}

extern "C" {
    pub fn X509_OBJECT_free_contents(a: *mut X509_OBJECT);
}