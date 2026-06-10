// SPDX-License-Identifier: MPL-2.0
//! C ABI probe and first Rust-backed utility functions for the staged rewrite.
//!
//! This crate deliberately keeps public ZeroMQ API symbols in C/C++. Rust exports
//! internal `zmq_rs_*` symbols that C++ wrappers call when the opt-in rewrite
//! build is enabled.

use core::ffi::c_void;

use libzmq_core::atomic_counter::AtomicCounter;

/// Internal probe used by CMake/CI to verify that Rust code can be compiled and
/// linked without colliding with the existing public C ABI.
#[no_mangle]
pub extern "C" fn zmq_rs_version_probe() -> u32 {
    libzmq_core::RUST_REWRITE_PROBE_VERSION
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
    use core::ffi::c_void;

    use super::*;

    #[test]
    fn probe_reports_scaffold_version() {
        assert_eq!(zmq_rs_version_probe(), 1);
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
