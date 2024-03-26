//! # LibSWIFFT
//!
//! `libswifft` is a high-level, safe, and Rust idiomatic interface to the `LibSWIFFT` library, providing efficient implementations of SWIFFT-based hashing algorithms.
//!
//! ## Features
//! - Easy-to-use API for SWIFFT hashing.
//! - Safe abstraction over the low-level `libswifft_sys` bindings.
//! - Additional utilities for working with SWIFFT hashes.
//!
//! ## Quick Start
//! Here's how you can compute a SWIFFT hash of some data:
//!
//! ```
//! use libswifft::hash;
//!
//! let data = b"Hello, world!";
//! let hash = hash::compute(data);
//! println!("Hash: {:?}", hash);
//! ```
//!
//! ## Note on Safety
//! While `libswifft` aims to provide a safe abstraction over the `LibSWIFFT` library, users should be aware that...


pub mod bindgen_ffi;
pub mod buffer;
pub mod hash;
pub mod arithmetic;
pub mod reference;