// SPDX-License-Identifier: MPL-2.0
//! C ABI probe and first Rust-backed utility functions for the staged rewrite.
//!
//! This crate deliberately keeps public ZeroMQ API symbols in C/C++. Rust exports
//! internal `zmq_rs_*` symbols that C++ wrappers call when the opt-in rewrite
//! build is enabled.

use core::ffi::{c_char, c_void};
use core::slice;

use libzmq_core::atomic_counter::AtomicCounter;

/// Internal probe used by CMake/CI to verify that Rust code can be compiled and
/// linked without colliding with the existing public C ABI.
#[no_mangle]
pub extern "C" fn zmq_rs_version_probe() -> u32 {
    libzmq_core::RUST_REWRITE_PROBE_VERSION
}

#[no_mangle]
pub extern "C" fn zmq_rs_version(major: *mut i32, minor: *mut i32, patch: *mut i32) {
    debug_assert!(!major.is_null());
    debug_assert!(!minor.is_null());
    debug_assert!(!patch.is_null());
    let version = libzmq_core::version::version();
    // SAFETY: callers must pass three valid, writable `int *` pointers, matching
    // the public `zmq_version` C API contract. The pointers are only written once
    // and are not retained after this call.
    unsafe {
        *major = version.major;
        *minor = version.minor;
        *patch = version.patch;
    }
}

#[no_mangle]
pub extern "C" fn zmq_rs_z85_encode(
    dest: *mut c_char,
    data: *const u8,
    size: usize,
) -> *mut c_char {
    if size % 4 != 0 {
        return core::ptr::null_mut();
    }
    debug_assert!(!dest.is_null());
    debug_assert!(!data.is_null());
    let Some(output_len) = libzmq_core::z85::encoded_len(size) else {
        return core::ptr::null_mut();
    };
    // SAFETY: `data` must point to `size` initialized bytes and `dest` must point
    // to at least `output_len + 1` writable bytes, matching `zmq_z85_encode`.
    let data = unsafe { slice::from_raw_parts(data, size) };
    let dest_bytes = unsafe { slice::from_raw_parts_mut(dest.cast::<u8>(), output_len + 1) };
    if libzmq_core::z85::encode(data, dest_bytes) {
        dest
    } else {
        core::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn zmq_rs_z85_decode(dest: *mut u8, string: *const c_char) -> *mut u8 {
    debug_assert!(!dest.is_null());
    debug_assert!(!string.is_null());
    // SAFETY: `string` must be a valid NUL-terminated C string. This mirrors the
    // existing public C API contract.
    let encoded = unsafe { core::ffi::CStr::from_ptr(string) }.to_bytes();
    let Some(output_len) = libzmq_core::z85::decoded_len(encoded) else {
        return core::ptr::null_mut();
    };
    // SAFETY: `dest` must point to at least `output_len` writable bytes, matching
    // the public `zmq_z85_decode` contract.
    let dest_bytes = unsafe { slice::from_raw_parts_mut(dest, output_len) };
    if libzmq_core::z85::decode(encoded, dest_bytes) {
        dest
    } else {
        core::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn zmq_rs_atomic_counter_new() -> *mut c_void {
    Box::into_raw(Box::new(AtomicCounter::new(0))).cast::<c_void>()
}

#[no_mangle]
pub extern "C" fn zmq_rs_atomic_counter_set(counter: *mut c_void, value: i32) {
    debug_assert!(!counter.is_null());
    // SAFETY: `counter` must be a pointer returned by `zmq_rs_atomic_counter_new`
    // and must remain alive for the duration of this call. This mirrors the
    // existing C utility API contract where passing any other pointer is
    // undefined behavior.
    let counter = unsafe { &*counter.cast::<AtomicCounter>() };
    counter.set(value as u32);
}

#[no_mangle]
pub extern "C" fn zmq_rs_atomic_counter_inc(counter: *mut c_void) -> i32 {
    debug_assert!(!counter.is_null());
    // SAFETY: see `zmq_rs_atomic_counter_set` for pointer provenance and lifetime.
    let counter = unsafe { &*counter.cast::<AtomicCounter>() };
    counter.inc() as i32
}

#[no_mangle]
pub extern "C" fn zmq_rs_atomic_counter_dec(counter: *mut c_void) -> i32 {
    debug_assert!(!counter.is_null());
    // SAFETY: see `zmq_rs_atomic_counter_set` for pointer provenance and lifetime.
    let counter = unsafe { &*counter.cast::<AtomicCounter>() };
    if counter.dec() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn zmq_rs_atomic_counter_value(counter: *mut c_void) -> i32 {
    debug_assert!(!counter.is_null());
    // SAFETY: see `zmq_rs_atomic_counter_set` for pointer provenance and lifetime.
    let counter = unsafe { &*counter.cast::<AtomicCounter>() };
    counter.value() as i32
}

#[no_mangle]
pub extern "C" fn zmq_rs_atomic_counter_destroy(counter_p: *mut *mut c_void) {
    debug_assert!(!counter_p.is_null());
    // SAFETY: `counter_p` must be a valid mutable pointer to a counter pointer
    // returned by `zmq_rs_atomic_counter_new`. We reconstruct the Box exactly once
    // and then null the caller's pointer, matching `zmq_atomic_counter_destroy`.
    unsafe {
        let counter = *counter_p;
        if !counter.is_null() {
            drop(Box::from_raw(counter.cast::<AtomicCounter>()));
        }
        *counter_p = core::ptr::null_mut();
    }
}

#[cfg(test)]
mod tests {
    use core::ffi::{c_char, c_void};

    use super::*;

    #[test]
    fn probe_reports_scaffold_version() {
        assert_eq!(zmq_rs_version_probe(), 1);
    }

    #[test]
    fn version_ffi_reports_header_constants() {
        let mut major = 0;
        let mut minor = 0;
        let mut patch = 0;
        zmq_rs_version(&mut major, &mut minor, &mut patch);
        assert_eq!((major, minor, patch), (4, 3, 6));
    }

    #[test]
    fn z85_ffi_roundtrips_rfc32_vector() {
        let data = [0x86, 0x4F, 0xD2, 0x6F, 0xB5, 0x59, 0xF7, 0x5B];
        let mut encoded = [0 as c_char; 11];
        let encoded_ptr = zmq_rs_z85_encode(encoded.as_mut_ptr(), data.as_ptr(), data.len());
        assert!(!encoded_ptr.is_null());

        let mut decoded = [0u8; 8];
        let decoded_ptr = zmq_rs_z85_decode(decoded.as_mut_ptr(), encoded.as_ptr());
        assert!(!decoded_ptr.is_null());
        assert_eq!(decoded, data);
    }

    #[test]
    fn atomic_counter_ffi_matches_cpp_return_values() {
        let mut counter = zmq_rs_atomic_counter_new();
        assert_eq!(zmq_rs_atomic_counter_value(counter), 0);
        assert_eq!(zmq_rs_atomic_counter_inc(counter), 0);
        assert_eq!(zmq_rs_atomic_counter_inc(counter), 1);
        assert_eq!(zmq_rs_atomic_counter_inc(counter), 2);
        assert_eq!(zmq_rs_atomic_counter_value(counter), 3);
        assert_eq!(zmq_rs_atomic_counter_dec(counter), 1);
        assert_eq!(zmq_rs_atomic_counter_dec(counter), 1);
        assert_eq!(zmq_rs_atomic_counter_dec(counter), 0);
        zmq_rs_atomic_counter_set(counter, 2);
        assert_eq!(zmq_rs_atomic_counter_dec(counter), 1);
        assert_eq!(zmq_rs_atomic_counter_dec(counter), 0);
        zmq_rs_atomic_counter_destroy((&mut counter as *mut *mut c_void).cast());
        assert!(counter.is_null());
    }
}
